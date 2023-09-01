use std::{
    borrow::Cow,
    env::var,
    fs,
    path::{Path, PathBuf},
};

fn protobuf_dir() -> String {
    std::env::var("M10_PROTOBUFS").unwrap_or_else(|_| "protobuf".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(var("OUT_DIR").expect("OUT_DIR environment variable not set"));
    println!("cargo:rerun-if-changed={}", protobuf_dir());
    let rerun_protobuf_dir = PathBuf::from(protobuf_dir());
    let mut rerun_dirs = vec![Cow::from(rerun_protobuf_dir)];
    while let Some(dir) = rerun_dirs.pop() {
        for entry_res in fs::read_dir(dir)? {
            let entry = entry_res?;
            let entry_path = entry.path();
            println!("cargo:rerun-if-changed={}", entry_path.display());
            if entry.metadata()?.is_dir() {
                rerun_dirs.push(Cow::from(entry_path));
            }
        }
    }

    // m10.model.pb
    prost_build::Config::new()
        .file_descriptor_set_path(out_dir.join("m10.model.pb"))
        .compile_protos(
            &[proto_path("sdk/model/model.proto")],
            &[PathBuf::from(protobuf_dir())],
        )?;

    // M10 Services
    let mut prost_config = prost_build::Config::new();
    if cfg!(feature = "dynamic2") {
        prost_config
            .type_attribute(
                ".m10.sdk.RoleBinding",
                "#[derive(looking_glass_derive::Instance)]",
            )
            .type_attribute(
                ".m10.sdk.Expression",
                "#[derive(looking_glass_derive::Instance)]",
            )
            .type_attribute(".m10.sdk.Role", "#[derive(looking_glass_derive::Instance)]")
            .type_attribute(".m10.sdk.Rule", "#[derive(looking_glass_derive::Instance)]")
            .type_attribute(
                ".m10.sdk.Value",
                "#[derive(looking_glass_derive::Instance, serde::Serialize, serde::Deserialize)]",
            )
            .type_attribute(
                ".m10.sdk.Value.value",
                "#[derive(looking_glass_derive::Instance, serde::Serialize, serde::Deserialize)]",
            )
            .type_attribute(
                ".m10.sdk.Operation",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            )
            .type_attribute(
                ".m10.sdk.DocumentOperations",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            )
            .type_attribute(
                ".m10.sdk.CollectionMetadata",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            )
            .type_attribute(
                ".m10.sdk.IndexMetadata",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            )
            .field_attribute(
                "file_descriptor_set",
                "#[serde(skip_serializing, skip_deserializing)]",
            )
            .field_attribute(
                "field_mask",
                "#[serde(skip_serializing, skip_deserializing)]",
            )
            .type_attribute(
                ".m10.sdk.transaction",
                "#[serde_with::serde_as] #[derive(serde::Serialize, serde::Deserialize)]",
            )
            .type_attribute(
                ".m10.sdk.FinalizedTransaction",
                "#[serde_with::serde_as] #[derive(serde::Serialize, serde::Deserialize)]",
            )
            .type_attribute(
                ".m10.sdk.metadata.Attachment",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            )
            .type_attribute(
                ".m10.sdk.metadata.Deposit",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            )
            .type_attribute(
                ".m10.sdk.metadata.Withdraw",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            )
            .field_attribute(
                "metadata",
                "#[serde_as(as = \"Vec<crate::AnySerDeCompat>\")]",
            )
            .type_attribute(
                ".m10.sdk.TransactionMetrics",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            );
    }

    prost_config
        .bytes([".m10.sdk.RoleBinding", ".m10.sdk.Role", "Value"])
        .file_descriptor_set_path(out_dir.join("m10.sdk.bin"));

    let service_protos = with_additional_protos(
        find_protos_in_dirs(&["directory", "sdk"])?,
        &["google/health/health.proto"],
    );
    tonic_build::configure()
        .build_server(cfg!(feature = "server"))
        .build_client(cfg!(feature = "client"))
        .compile_with_config(prost_config, &service_protos, &[protobuf_dir()])?;
    Ok(())
}

// Aux functions
fn proto_path(path: &str) -> PathBuf {
    Path::new(&protobuf_dir()).join(path)
}

fn find_protos_in_dir(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = vec![];
    for dir_entry in fs::read_dir(path)? {
        let dir_entry = dir_entry?.path();
        if dir_entry.is_file()
            && dir_entry
                .extension()
                .map(|ext| ext == "proto")
                .unwrap_or(false)
        {
            files.push(dir_entry.as_path().display().to_string())
        }
        if dir_entry.is_dir() {
            let sub_dir_files = find_protos_in_dir(dir_entry.as_path())?;
            files.extend_from_slice(&sub_dir_files);
        }
    }
    Ok(files)
}

fn find_protos_in_dirs(paths: &[&str]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = vec![];
    for path in paths {
        files.extend(find_protos_in_dir(&proto_path(path))?);
    }
    Ok(files)
}

fn with_additional_protos(mut protos: Vec<String>, additional_paths: &[&str]) -> Vec<String> {
    for path in additional_paths {
        protos.push(proto_path(path).display().to_string());
    }
    protos
}

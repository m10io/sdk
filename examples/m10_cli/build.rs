use vergen::{BuildBuilder, Emitter, RustcBuilder};
use vergen_git2::Git2Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Build info
    let build = BuildBuilder::default().build_timestamp(true).build()?;

    // 2) Git info
    let git = Git2Builder::default()
        .branch(true)
        .commit_timestamp(true)
        .sha(true)
        .build()?;

    // 3) Rustc info
    let rustc = RustcBuilder::all_rustc()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&git)?
        .add_instructions(&rustc)?
        .emit()?;

    // 4) Short SHA (7 chars) for display in version string.
    // Priority: VERGEN_GIT_SHA env override (set in CI workflow) → git command → "unknown".
    let sha = std::env::var("VERGEN_GIT_SHA")
        .ok()
        .or_else(|| {
            std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| "unknown".to_string());
    let short_sha: String = sha.chars().take(7).collect();
    println!("cargo:rustc-env=GIT_SHA_SHORT={short_sha}");

    Ok(())
}

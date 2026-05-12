use clap::Parser;
use m10_sdk::error::M10Error;
use verbose::LogLevel;

mod collections;
mod commands;
mod config;
mod context;
mod dyn_signer;
mod tls;
mod utils;
mod verbose;

#[cfg(feature = "internal")]
const LONG_VERSION: &str = const_str::concat!(
    env!("CARGO_PKG_VERSION"),
    " (commit ",
    env!("GIT_SHA_SHORT"),
    ", built ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")\n",
    "Commit Date:      ",
    env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
    "\n",
    "Git Branch:       ",
    env!("VERGEN_GIT_BRANCH"),
    "\n",
    "Rustc Version:    ",
    env!("VERGEN_RUSTC_SEMVER"),
    "\n",
);

#[cfg(not(feature = "internal"))]
const LONG_VERSION: &str = const_str::concat!(
    env!("CARGO_PKG_VERSION"),
    " (commit ",
    env!("GIT_SHA_SHORT"),
    ", built ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")\n",
);

#[derive(Clone, Parser, Debug)]
#[clap(
    about,
    version = env!("CARGO_PKG_VERSION"), // Keep the short version simple
    long_version = LONG_VERSION // Use the detailed version string here
)]
pub(crate) struct Opts {
    /// Server address
    #[arg(global = true, short, long)]
    pub(crate) server: Option<String>,
    /// Disable TLS (e.g. for localhost connection)
    #[arg(global = true, long)]
    pub(crate) no_tls: bool,
    /// Choose http connection instead of gRPC
    #[arg(global = true, long)]
    pub(crate) http: bool,
    /// Load key from file (pkcs8)
    #[arg(
        global = true,
        short,
        long,
        conflicts_with_all = &["vault_addr", "vault_token", "vault_key_name"]
    )]
    pub(crate) key_file: Option<String>,
    /// Force digest signing mode for Ed25519 key file (only valid value: "ed25519ph").
    #[arg(
        global = true,
        long,
        requires = "key_file",
        conflicts_with_all = &["vault_addr", "vault_token", "vault_key_name"]
    )]
    pub(crate) key_algorithm: Option<String>,
    /// Use a profile from a config file for global options
    #[arg(global = true, short, long)]
    pub(crate) profile: Option<String>,
    /// Set context ID. Parsed as Hex
    #[arg(global = true, short, long)]
    pub(crate) context_id: Option<String>,
    /// Vault Transit server address
    #[arg(global = true, long, requires_all = &["vault_token", "vault_key_name"])]
    pub(crate) vault_addr: Option<String>,
    /// Vault token for authentication
    #[arg(global = true, long)]
    pub(crate) vault_token: Option<String>,
    /// Vault key name to use
    #[arg(global = true, long)]
    pub(crate) vault_key_name: Option<String>,
    /// Optional mount point for Vault Transit (default: "transit")
    #[arg(global = true, long)]
    pub(crate) vault_mount: Option<String>,
    /// Vault enterprise namespace
    #[arg(global = true, long)]
    pub(crate) vault_namespace: Option<String>,
    /// Vault algorithm: "ed25519", "ed25519ph", or "p256" (default: "ed25519")
    #[arg(global = true, long)]
    pub(crate) vault_algorithm: Option<String>,
    /// Path to additional CA certificate PEM file (added to native roots)
    #[arg(global = true, long)]
    pub(crate) ca_cert: Option<String>,
    #[command(subcommand)]
    cmd: commands::Commands,
    /// Set log level (default: no logging)
    #[arg(global = true, long, value_enum)]
    pub(crate) verbose: Option<LogLevel>,
}

fn error_to_message(err: &anyhow::Error) -> String {
    if let Some(error) = err.downcast_ref::<M10Error>() {
        return error.get_message();
    };

    err.to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let _ = rustls::crypto::ring::default_provider().install_default();
    let context = context::Context::new_from_options(&opts).await?;
    if let Err(err) = opts.cmd.run(&context).await {
        eprintln!("error: {}", error_to_message(&err));
        std::process::exit(1);
    }

    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Opts::command().debug_assert()
}

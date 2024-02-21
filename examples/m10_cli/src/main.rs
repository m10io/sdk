use core::str::FromStr;

use clap::Parser;

mod collections;
mod commands;
mod config;
mod context;
mod utils;

#[derive(Clone, Debug)]
struct HexStr(Vec<u8>);

impl FromStr for HexStr {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> anyhow::Result<Self> {
        Ok(Self(hex::decode(value)?))
    }
}

#[derive(Clone, Parser, Debug)]
#[clap(about)]
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
    #[arg(global = true, short, long)]
    pub(crate) key_file: Option<String>,
    /// Use a profile from a config file for global options
    #[arg(global = true, short, long)]
    pub(crate) profile: Option<String>,
    /// Set context ID. Parsed as Hex
    #[clap(global = true, short, long)]
    pub(crate) context_id: Option<HexStr>,
    #[command(subcommand)]
    cmd: commands::Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let context = context::Context::new_from_options(&opts)?;
    opts.cmd.run(&context).await?;
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Opts::command().debug_assert()
}

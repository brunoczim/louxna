use std::process::ExitCode;

use clap::Parser;
use louxna::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let cli = Cli::parse();
    let status = cli.run().await?;
    Ok(ExitCode::from(status as u8))
}

use clap::Parser;
use louxna::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run().await?;
    Ok(())
}

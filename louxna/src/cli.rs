use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::{id::package::PkgName, package::PkgConfig, runtime::Runtime};

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[clap(long = "root", default_value = ".")]
    pub root: PathBuf,
    #[clap(subcommand)]
    pub cmd: Command,
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<()> {
        self.cmd.run(self.root).await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Run(RunCommand),
    Init(InitCommand),
}

impl Command {
    pub async fn run(self, root: PathBuf) -> anyhow::Result<()> {
        match self {
            Self::Run(cmd) => cmd.run(root).await,
            Self::Init(cmd) => cmd.run(root).await,
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct RunCommand {}

impl RunCommand {
    pub async fn run(self, root: PathBuf) -> anyhow::Result<()> {
        let runtime = Runtime::new(root).await?;
        runtime.run()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Args)]
pub struct InitCommand {
    #[clap(short = 'n', long = "name")]
    pub name: Box<PkgName>,
}

impl InitCommand {
    pub async fn run(self, root: PathBuf) -> anyhow::Result<()> {
        let pkg = PkgConfig { name: self.name };
        pkg.save(root).await?;
        Ok(())
    }
}

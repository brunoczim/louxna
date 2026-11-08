use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::{
    id::package::{PkgName, PkgNamespace},
    package::PkgConfig,
    runtime::Runtime,
};

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[clap(long = "root", default_value = ".")]
    pub root: PathBuf,
    #[clap(subcommand)]
    pub cmd: Command,
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<i32> {
        self.cmd.run(self.root).await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Run(RunCommand),
    Init(InitCommand),
}

impl Command {
    pub async fn run(self, root: PathBuf) -> anyhow::Result<i32> {
        match self {
            Self::Run(cmd) => cmd.run(root).await,
            Self::Init(cmd) => cmd.run(root).await,
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct RunCommand {
    pub args: Vec<String>,
}

impl RunCommand {
    pub async fn run(self, root: PathBuf) -> anyhow::Result<i32> {
        let runtime = Runtime::load(root).await?;
        let status = runtime.run_as_binary(self.args)?;
        Ok(status)
    }
}

#[derive(Debug, Clone, Args)]
pub struct InitCommand {
    #[clap(short = 'n', long = "name")]
    pub name: Box<PkgName>,
    #[clap(short = 's', long = "namespace")]
    pub namespace: Box<PkgNamespace>,
}

impl InitCommand {
    pub async fn run(self, root: PathBuf) -> anyhow::Result<i32> {
        let pkg = PkgConfig {
            name: self.name,
            namespace: self.namespace,
            isolated_modules: false,
        };
        pkg.save(root).await?;
        Ok(0)
    }
}

pub(crate) mod fetch;

use anyhow::Result;
use bojexec_core::fs::structure::FsConfig;
use clap::Parser;
use fetch::FetchCommand;

#[async_trait::async_trait]
pub(crate) trait Command {
    async fn execute(&self, config: &FsConfig) -> Result<()>;
}

#[derive(Parser)]
#[command(name = "bojexec")]
#[command(about = "BOJ execution CLI tool")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

impl Cli {
    pub(crate) async fn execute(&self, config: &FsConfig) -> Result<()> {
        self.command.execute(config).await
    }
}

#[derive(Parser)]
pub(crate) enum Commands {
    Fetch(FetchCommand),
}

#[async_trait::async_trait]
impl Command for Commands {
    async fn execute(&self, config: &FsConfig) -> Result<()> {
        match self {
            Commands::Fetch(cmd) => fetch::fetch(config, cmd.id).await,
        }
    }
}

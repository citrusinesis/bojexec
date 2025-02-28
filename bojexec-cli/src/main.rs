mod command;

use anyhow::Result;
use bojexec_core::fs::structure::{DirectoryStructure, FsConfig};
use clap::Parser;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = command::Cli::parse();

    let config = FsConfig {
        problems_base: PathBuf::from("problems"),
        solutions_base: Some(PathBuf::from("solutions")),
        structure: DirectoryStructure::Separate,
        problem_set: PathBuf::from("baekjoon"),
    };

    cli.execute(&config).await
}

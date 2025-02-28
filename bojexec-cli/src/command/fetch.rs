use anyhow::Result;
use bojexec_core::{
    crawl::get_baekjoon_problem,
    fs::{save_problem, structure::FsConfig},
};
use clap::Parser;
use std::fs;

#[derive(Parser)]
pub(crate) struct FetchCommand {
    #[arg(short, long)]
    pub(crate) id: u32,
}

pub(crate) async fn fetch(config: &FsConfig, id: u32) -> Result<()> {
    fs::create_dir_all(&config.problems_base)?;
    if let Some(solutions_base) = &config.solutions_base {
        fs::create_dir_all(solutions_base)?;
    }

    let problem = get_baekjoon_problem(id).await.unwrap();
    save_problem(config, &problem)?;
    println!("Problem {} fetched and saved successfully", id);
    Ok(())
}

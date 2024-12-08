use crate::cli::config::show::ConfigCommand;
use crate::cli::vapid::generate;
use clap::{Parser, Subcommand};

use crate::cli::search::SearchCommand;

#[derive(Debug, Parser)]
#[command(name = "notectl", about = "A CLI tool for managing misskey")]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "About webpush notification")]
    Webpush(generate::VapidCommand),
    #[command(about = "About your misskey configuration")]
    Config(ConfigCommand),
    #[command(about = "About Meilisearch")]
    Search(SearchCommand),
}

pub async fn exec() {
    let args = Cli::parse();
    match args.cmd {
        Commands::Webpush(cmd) => {
            cmd.exec().unwrap();
        }
        Commands::Config(cmd) => {
            cmd.exec().unwrap();
        }
        Commands::Search(cmd) => {
            cmd.exec().await.unwrap();
        }
    }
}

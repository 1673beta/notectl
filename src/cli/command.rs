use crate::cli::config::show::ConfigCommand;
use crate::cli::vapid::generate;
use clap::{Parser, Subcommand};

use crate::cli::search::SearchCommand;
use crate::cli::remote::RemoteCommand;

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
    #[command(about = "About remote server")]
    Remote(RemoteCommand),
}

pub async fn exec() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.cmd {
        Commands::Webpush(cmd) => {
            if let Err(e) = cmd.exec() {
                eprintln!("{}", e);
            }
        }
        Commands::Config(cmd) => {
            if let Err(e) = cmd.exec() {
                eprintln!("{}", e);
            }
        }
        Commands::Search(cmd) => {
            if let Err(e) = cmd.exec().await {
                eprintln!("{}", e);
            }
        }
        Commands::Remote(cmd) => {
            if let Err(e) = cmd.exec().await {
                eprintln!("{}", e);
            }
        }
    }
    Ok(())
}
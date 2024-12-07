use crate::cli::config::show::ConfigCommand;
use crate::cli::vapid::generate;
use clap::{Parser, Subcommand};

use crate::cli::search::SearchCommand;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Webpush(generate::VapidCommand),
    Config(ConfigCommand),
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
            cmd.exec().unwrap();
        }
    }
}

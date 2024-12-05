use crate::cli::config::show::ConfigCommand;
use crate::cli::vapid::generate;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Webpush(generate::VapidCommand),
    Config(ConfigCommand),
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
    }
}

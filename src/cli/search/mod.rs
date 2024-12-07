pub mod deploy;
pub mod drop;
pub mod get;
pub mod list;
pub mod redeploy;

use clap::{Parser, Subcommand};
use list::list;

#[derive(Debug, Parser)]
#[command(name = "search")]
pub struct SearchCommand {
    #[command(subcommand)]
    pub subcmd: SearchSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SearchSubCommand {
    List {
        #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
        config_path: String,
    },
}

impl SearchCommand {
    pub fn exec(&self) ->Result<(), Box<dyn std::error::Error>> {
        match &self.subcmd {
            SearchSubCommand::List { config_path } => {
                let _ = list(config_path);
            }
        }
        Ok(())
    }
}
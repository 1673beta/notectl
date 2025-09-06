pub mod deploy;
pub mod drop;
pub mod get;
pub mod health;
pub mod list;
pub mod redeploy;

use clap::{Parser, Subcommand};
use deploy::deploy;
use drop::drop;
use health::health;
use list::list;

#[derive(Debug, Parser)]
#[command(name = "search")]
pub struct SearchCommand {
  #[command(subcommand)]
  pub subcmd: SearchSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SearchSubCommand {
  List,
  Health,
  Drop,
  Deploy {
    #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
    config_path: String,
  },
}

impl SearchCommand {
  pub async fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.subcmd {
      SearchSubCommand::List => {
        list().await?;
      }
      SearchSubCommand::Health => {
        health().await?;
      }
      SearchSubCommand::Drop => {
        drop().await?;
      }
      SearchSubCommand::Deploy { config_path } => {
        deploy(config_path).await?;
      }
    }
    Ok(())
  }
}

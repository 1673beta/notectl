pub mod delete;

use clap::{Parser, Subcommand};
use delete::delete;

#[derive(Debug, Parser)]
#[command(name = "note")]
pub struct NoteCommand {
  #[command(subcommand)]
  pub subcmd: NoteSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum NoteSubCommand {
  #[command(about = "Delete notes from remote server. Needs to specify days before to delete.")]
  Delete {
    #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
    config_path: String,
    #[arg(short = 'H', long = "host")]
    host: Option<String>,
    #[arg(short = 'd', long = "days")]
    days: u64,
  },
}

impl NoteCommand {
  pub async fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.subcmd {
      NoteSubCommand::Delete {
        config_path,
        host,
        days,
      } => {
        delete(config_path, host.as_deref(), *days).await?;
      }
    }
    Ok(())
  }
}

use crate::cli::config::show::ConfigCommand;
use crate::cli::vapid::WebpushCommand;
use clap::{ColorChoice, Parser, Subcommand};

use crate::cli::id::IdCommand;
use crate::cli::note::NoteCommand;
use crate::cli::remote::RemoteCommand;
use crate::cli::search::SearchCommand;
use crate::cli::user::UserCommand;

#[derive(Debug, Parser)]
#[command(name = "notectl", about = "A CLI tool for managing misskey", color = ColorChoice::Always, styles = super::style::style())]
pub struct Cli {
  #[clap(subcommand)]
  pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(about = "About webpush notification")]
  Webpush(WebpushCommand),
  #[command(about = "About your misskey configuration")]
  Config(ConfigCommand),
  #[command(about = "About Meilisearch")]
  Search(SearchCommand),
  #[command(about = "About remote server")]
  Remote(RemoteCommand),
  #[command(about = "About id")]
  Id(IdCommand),
  #[command(about = "About user")]
  User(UserCommand),
  #[command(about = "About note")]
  Note(NoteCommand),
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
    Commands::Id(cmd) => {
      cmd.exec();
    }
    Commands::User(cmd) => {
      if let Err(e) = cmd.exec().await {
        eprintln!("{}", e);
      }
    }
    Commands::Note(cmd) => {
      if let Err(e) = cmd.exec().await {
        eprintln!("{}", e);
      }
    }
  }
  Ok(())
}

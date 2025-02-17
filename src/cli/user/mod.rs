pub mod cull;
pub mod delete;
pub mod prune;

use clap::{Parser, Subcommand, ArgGroup};
use cull::cull;
use delete::delete;
use prune::prune;

#[derive(Debug, Parser)]
#[command(name = "user")]
pub struct UserCommand {
  #[command(subcommand)]
  pub subcmd: UserSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
  Cull {
    #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
    config_path: String,
  },
  #[command(group(
    ArgGroup::new("user_delete")
      .required(true)
      .args(&["id", "username"])
  ),
    about = "Delete user account(s)",
    long_about = "Delete remote user account(s) by ID or username. Need specify either ID or username.")]
  Delete {
    #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
    config_path: String,
    #[arg(short = 'i', long = "id", value_delimiter = ',', help = "Id of user you want to delete. You can specify multiple IDs with comma. e.g. -i a4ct2ps000,a3tghk0000")]
    id: Option<Vec<String>>,
    #[arg(short = 'u', long = "username", value_delimiter = ',', help = "Username of user you want to delete. You can specify multiple usernames with comma. e.g. -u alice,bob")]
    username: Option<Vec<String>>,
  },
  Prune {
    #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
    config_path: String,
    #[arg(short = 'H', long = "host")]
    host: Option<String>,
    #[arg(short = 'n', long = "no-note", action = clap::ArgAction::SetTrue)]
    note: bool,
    #[arg(short = 'f', long = "no-follow", action = clap::ArgAction::SetTrue)]
    follow: bool,
  },
}

impl UserCommand {
  pub async fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.subcmd {
      UserSubCommand::Cull { config_path } => {
        cull(config_path).await?;
      }
      UserSubCommand::Delete { config_path, id, username } => {
        let id_ref = id.as_ref().map(|v| {
          v.iter().map(|s| s.as_str()).collect::<Vec<&str>>()
        });
        let username_ref = username.as_ref().map(|v| {
          v.iter().map(|s| s.as_str()).collect::<Vec<&str>>()
        });
        delete(config_path, id_ref, username_ref).await?;
      }
      UserSubCommand::Prune {
        config_path,
        host,
        note,
        follow,
      } => {
        let refs = host.as_deref();
        prune(config_path, refs, *note, *follow).await?;
      }
    }
    Ok(())
  }
}

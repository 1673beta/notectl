pub mod cull;
pub mod delete;
pub mod prune;

use clap::{Parser, Subcommand};
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
    Delete {
        #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
        config_path: String,
        #[arg(short = 'i', long = "id", value_delimiter = ',')]
        id: Vec<String>,
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
            UserSubCommand::Delete { config_path, id } => {
                let refs = id.iter().map(|s| s.as_str()).collect();
                delete(config_path, refs).await?;
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

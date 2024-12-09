pub mod gone;
pub mod suspend;
pub mod unsuspend;

// Note: サーバーのサスペンド状態は、
// none | manuallySuspended | goneSuspended | autoSuspendedForNotResponding
// の4つ

use clap::{Parser, Subcommand};
use gone::gone;
use suspend::suspend;
use unsuspend::unsuspend;

#[derive(Debug, Parser)]
#[command(name = "remote")]
pub struct RemoteCommand {
    #[command(subcommand)]
    pub subcmd: RemoteSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum RemoteSubCommand {
    Gone {
        #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
        config_path: String,
        #[arg(short = 'u', long = "url")]
        url: String,
    },
    Suspend {
        #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
        config_path: String,
        #[arg(short = 'u', long = "url")]
        url: String,
    },
    Unsuspend {
        #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
        config_path: String,
        #[arg(short = 'u', long = "url")]
        url: String,
    },
}

impl RemoteCommand {
    pub async fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcmd {
            RemoteSubCommand::Gone { config_path, url } => {
                gone(config_path, url).await?;
            }
            RemoteSubCommand::Suspend { config_path, url } => {
                suspend(config_path, url).await?;
            }
            RemoteSubCommand::Unsuspend { config_path, url } => {
                unsuspend(config_path, url).await?;
            }
        }
        Ok(())
    }
}

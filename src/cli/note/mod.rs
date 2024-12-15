pub mod delete;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "note")]
pub struct NoteCommand {
    #[command(subcommand)]
    pub subcmd: NoteSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum NoteSubCommand {
    Delete {
        #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
        config_path: String,
        #[arg(short = 'r', long = "reply", action = clap::ArgAction::SetTrue)]
        reply: bool,
        #[arg(short = 'q', long = "quote", action = clap::ArgAction::SetTrue)]
        quote: bool,
        #[arg(short = 'H', long = "host")]
        host: Option<String>,
        #[arg(short = 'd', long = "days", default_value = "90")]
        days: u64,
    },
}

impl NoteCommand {
    pub async fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcmd {
            NoteSubCommand::Delete {
                config_path,
                reply,
                quote,
                host,
                days,
            } => {
                println!(
                    "config_path: {}, reply: {}, quote: {}, days: {}, host: {:?}",
                    config_path, reply, quote, days, host
                )
            }
        }
        Ok(())
    }
}

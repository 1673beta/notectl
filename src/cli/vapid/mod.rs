pub mod generate;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct WebpushCommand {
    #[command(subcommand)]
    pub subcmd: WebpushSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum WebpushSubCommand {
    #[command(about = "Generate VAPID keys")]
    Generate,
}

impl WebpushCommand {
    pub fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcmd {
            WebpushSubCommand::Generate => {
                crate::cli::vapid::generate::gen()?;
            }
        }
        Ok(())
    }
}

use crate::config::{MisskeyConfig, load_config};
use clap::{Subcommand, Parser};

#[derive(Debug, Parser)]
#[command(name = "config")]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub subcmd: ConfigSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    Show,
}

impl ConfigCommand {
    pub fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcmd {
            ConfigSubCommand::Show => {
                let config: MisskeyConfig = load_config()?;
                println!("{:?}", config);
            }
        }
        Ok(())
    }
}
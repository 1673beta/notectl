use crate::config::{load_config, print_config, MisskeyConfig};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "config")]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub subcmd: ConfigSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    Show {
        #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
        config_path: String,
    },
}

impl ConfigCommand {
    pub fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcmd {
            ConfigSubCommand::Show { config_path} => {
                let config: MisskeyConfig = load_config(&config_path)?;
                let _ = print_config(&config);
            }
        }
        Ok(())
    }
}

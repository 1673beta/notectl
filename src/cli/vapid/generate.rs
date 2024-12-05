use crate::util::vapid;
use clap::{Subcommand, Parser, Arg};

#[derive(Debug, Parser)]
#[command(name = "webpush")]
pub struct VapidCommand {
    #[command(subcommand)]
    pub subcmd: VapidSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum VapidSubCommand {
    Generate,
}

impl VapidCommand {
    pub fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcmd {
            VapidSubCommand::Generate => {
                let key = vapid::generate()?;
                println!("Private Key: {}", key.private_key);
                println!("Public Key: {}", key.public_key);
            }
        }
        Ok(())
    }
}
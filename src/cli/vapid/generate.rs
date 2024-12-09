use crate::util::vapid;
use clap::{Parser, Subcommand};
use nu_ansi_term::Color::Red;

#[derive(Debug, Parser)]
#[command(name = "webpush")]
pub struct VapidCommand {
    #[command(subcommand)]
    pub subcmd: VapidSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum VapidSubCommand {
    #[command(about = "Generate VAPID keys")]
    Generate,
}

impl VapidCommand {
    pub fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcmd {
            VapidSubCommand::Generate => {
                let key = vapid::generate()?;
                println!("{}",Red.paint("Please copy the following keys and paste them into Service Worker Settings in control panel."));
                println!("Private Key: {}", key.private_key);
                println!("Public Key: {}", key.public_key);
            }
        }
        Ok(())
    }
}

use crate::util::vapid;
use clap::{Parser, Subcommand};

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
        let style = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));
        println!("{style}{}{style:#}","Please copy the following keys and paste them into Service Worker Settings in control panel.");
        println!("Private Key: {}", key.private_key);
        println!("Public Key: {}", key.public_key);
      }
    }
    Ok(())
  }
}

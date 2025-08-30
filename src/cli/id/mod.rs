pub mod gen;
pub mod parse;

use clap::{Parser, Subcommand};
use gen::gen;
use parse::parse;

use crate::configs::server::IdMethod;

#[derive(Debug, Parser)]
#[command(name = "id")]
pub struct IdCommand {
  #[command(subcommand)]
  pub subcmd: IdSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum IdSubCommand {
  Parse {
    #[arg(short = 'f', long = "format")]
    id_type: IdMethod,
    #[arg(short = 'i', long = "id")]
    id: String,
  },
  Gen {
    #[arg(short = 'f', long = "format")]
    id_type: IdMethod,
  },
}

impl IdCommand {
  pub fn exec(&self) {
    match &self.subcmd {
      IdSubCommand::Parse { id_type, id } => {
        println!("{}", parse(id, *id_type))
      }
      IdSubCommand::Gen { id_type } => {
        println!("{}", gen(*id_type))
      }
    }
  }
}

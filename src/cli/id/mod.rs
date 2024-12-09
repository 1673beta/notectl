pub mod parse;

use clap::{Parser, Subcommand};
use parse::parse;

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
        id_type: String,
        #[arg(short = 'i', long = "id")]
        id: String,
    }
}

impl IdCommand {
    pub fn exec(&self) {
        match &self.subcmd {
            IdSubCommand::Parse { id_type, id } => {
                println!("{}", parse(id, id_type))
            }
        }
    }
}
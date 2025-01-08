pub mod gen;
pub mod parse;

use clap::{Parser, Subcommand, ValueEnum};
use gen::gen;
use parse::parse;

#[derive(Debug, Parser)]
#[command(name = "id")]
pub struct IdCommand {
    #[command(subcommand)]
    pub subcmd: IdSubCommand,
}

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum IdType {
    Aid,
    Aidx,
    Meid,
    ObjectId,
    Ulid,
}

#[derive(Debug, Subcommand)]
pub enum IdSubCommand {
    Parse {
        #[arg(short = 'f', long = "format")]
        id_type: String,
        #[arg(short = 'i', long = "id")]
        id: String,
    },
    Gen {
        #[arg(short = 'f', long = "format")]
        id_type: IdType,
    },
}

impl IdCommand {
    pub fn exec(&self) {
        match &self.subcmd {
            IdSubCommand::Parse { id_type, id } => {
                println!("{}", parse(id, id_type))
            }
            IdSubCommand::Gen { id_type } => {
                println!("{}", gen(*id_type))
            }
        }
    }
}

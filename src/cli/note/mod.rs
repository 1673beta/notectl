pub mod delete;

use crate::entities::sea_orm_active_enums::NoteVisibilityEnum;
use clap::{Parser, Subcommand};
use delete::delete;

#[derive(Debug, Parser)]
#[command(name = "note")]
pub struct NoteCommand {
  #[command(subcommand)]
  pub subcmd: NoteSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum NoteSubCommand {
  #[command(about = "Delete notes from remote server. Needs to specify days before to delete.")]
  Delete {
    #[arg(short = 'c', long = "config", default_value = ".config/default.yml")]
    config_path: String,
    #[arg(short = 'H', long = "host")]
    host: Option<String>,
    #[arg(short = 'd', long = "days")]
    days: u64,
    #[arg(short = 'v', long = "visibility", value_delimiter = ',')]
    visibility: Option<Vec<NoteVisibilityEnum>>,
    #[arg(long = "no-interaction", conflicts_with_all = ["no_reaction", "no_reply", "no_renote", "no_clipped"], action = clap::ArgAction::SetTrue)]
    no_interaction: bool,
    #[arg(long = "no-reaction", action = clap::ArgAction::SetTrue, conflicts_with = "no_interaction")]
    no_reaction: bool,
    #[arg(long = "no-reply", action = clap::ArgAction::SetTrue, conflicts_with = "no_interaction")]
    no_reply: bool,
    #[arg(long = "no-renote", action = clap::ArgAction::SetTrue, conflicts_with = "no_interaction")]
    no_renote: bool,
    #[arg(long = "no-clipped", action = clap::ArgAction::SetTrue, conflicts_with = "no_interaction")]
    no_clipped: bool,
  },
}

impl NoteCommand {
  pub async fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.subcmd {
      NoteSubCommand::Delete {
        config_path,
        host,
        days,
        visibility,
        no_interaction,
        no_reaction,
        no_reply,
        no_renote,
        no_clipped,
      } => {
        let no_reaction_input = *no_interaction || *no_reaction;
        let no_reply_input = *no_interaction || *no_reply;
        let no_renote_input = *no_interaction || *no_renote;
        let no_clipped_input = *no_interaction || *no_clipped;
        delete(
          config_path,
          host.as_deref(),
          *days,
          visibility.clone(),
          no_reaction_input,
          no_reply_input,
          no_renote_input,
          no_clipped_input,
        )
        .await?;
      }
    }
    Ok(())
  }
}

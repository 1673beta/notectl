mod cli;
mod config; // TODO: 消す
mod configs; // TODO: rename
mod consts;
mod db;
mod entities;
mod services;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  if let Err(e) = cli::command::exec().await {
    eprintln!("{}", e);
  }

  Ok(())
}

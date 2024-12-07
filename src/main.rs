mod cli;
mod config;
mod db;
mod entities;
mod util;

use anyhow::Ok;

#[tokio::main]
async fn main() {
    let _ = cli::command::exec().await;

    Ok(()).expect("Failed to execute the program");
}

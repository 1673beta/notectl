mod config;
mod cli;
mod db;
mod entities;
mod util;

#[tokio::main]
async fn main() {
    let _ = cli::command::exec().await;
}

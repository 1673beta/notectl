mod cli;
mod config;
mod db;
mod entities;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = cli::command::exec().await {
        eprintln!("{}", e);
    }

    Ok(())
}

mod config;
mod cli;
mod db;
mod entities;
mod util;

#[tokio::main]
async fn main() {
    let id = util::id::aid::parse("a1dry0dc00");
    println!("id: {:?}", id);
}

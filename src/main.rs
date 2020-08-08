mod watching;
mod models;

#[macro_use] extern crate log;
#[macro_use] extern crate serde;
use tokio;
use watching::create_and_start_watchers;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();
    create_and_start_watchers().await?;
    Ok(())
}

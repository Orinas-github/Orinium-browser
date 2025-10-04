use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let _args: Vec<String> = env::args().collect::<Vec<String>>();
    env_logger::init();

    Ok(())
}

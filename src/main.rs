use std::env;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    env_logger::init();

    Ok(())
}

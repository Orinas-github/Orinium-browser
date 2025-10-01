use std::env;
// use std::path::Path;
use anyhow::{Context, Result};

// use platform::io;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    env_logger::init();

    Ok(())
}

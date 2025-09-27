use std::env;
// use std::path::Path;
use anyhow::{Context, Result};

mod engine;
mod platform;
use engine::html::parser;
use platform::network;
// use platform::io;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    env_logger::init();

    Ok(())
}

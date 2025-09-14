mod platform;
mod engine;
use engine::html::tokenizer::{Tokenizer};

#[tokio::main]
async fn main() {
    println!("Hello, Orinium Browser!");

    let html = "<!DOCTYPE html><html><head><title>Test</title></head><body><h1>Hello, world!</h1></body></html>";
    let mut tokenizer = Tokenizer::new(html);
    while let Some(token) = tokenizer.next_token() {
        println!("{:?}", token);
    }
}

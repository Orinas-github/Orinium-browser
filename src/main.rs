mod platform;
mod engine;
use engine::html::tokenizer::{Tokenizer};

#[tokio::main]
async fn main() {
    println!("Hello, Orinium Browser!");

    let html = r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd"><html><head><title>Test</title></head><body><h1>Hello, world!</h1></body></html>"#;
    print!("Parsing HTML: {}\n", html);
    let mut tokenizer = Tokenizer::new(html);
    while let Some(token) = tokenizer.next_token() {
        println!("{:?}", token);
    }
}

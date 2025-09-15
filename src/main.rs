mod platform;
mod engine;
use engine::html::tokenizer::{Tokenizer};

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Hello, Orinium Browser!");

    let html = r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>TEST</title>
</head>
<body>
    <h1 id="main-title">TEST</h1>
    <p id="intro">This is a paragraph using HTML 4.01 Strict Doctype.</p>
</body>
</html>
"#;
    print!("Parsing HTML: {}\n", html);
    let mut tokenizer = Tokenizer::new(html);
    while let Some(token) = tokenizer.next_token() {
        println!("{:?}", token);
    }
}

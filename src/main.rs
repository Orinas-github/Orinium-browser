use std::env;
use std::path::Path;
use anyhow::{Result, Context};

mod platform;
mod engine;
use engine::html::parser;
use platform::network;
use platform::io;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    env_logger::init();

    let url_or_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "https://example.com".to_string()
    };

    let html = if url_or_path.starts_with("http://") || url_or_path.starts_with("https://") {
        println!("Getting...[{}]", url_or_path);
        let net = network::NetworkCore::new()?;
        let bytes = net.fetch(&url_or_path).await?.body;
        String::from_utf8(bytes).context("Warn: Resoulse was'nt encoded by UTF-8")?
    } else if Path::new(&url_or_path).exists() {
        println!("loading local file[{}]", url_or_path);
        let bytes = io::load_local_file(&url_or_path).await?;
        String::from_utf8(bytes).context("Warn: Resoulse was'nt encoded by UTF-8")?
    } else {
        let html = r#"<!DOCTYPE html>
<html lang="ja">
<head>
    <title>Orinium Browser DOM Test</title>
    <!-- コメント -->
</head>
<body>
    <p>This is a <b>test page</b> for DOM module debuging.</p>
    <div>
        <p>Nested <span>span text</span></p>
        <img src="image.png">
        <br />
        <input type="text" value="Hello" />
        <p>Unclosed paragraph
    </div>
    <footer>Footer content</footer>
</body>
</html>
"#;
        println!("Using the default test html code...");
        html.to_string()
    };

    println!("Parsing HTML: {}\n", html);
    let mut parser = parser::Parser::new(&html);
    let dom = parser.parse();
    parser::print_dom_tree(&dom,"" , true);
    Ok(())
}

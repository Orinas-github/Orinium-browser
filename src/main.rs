use std::env;
use std::path::Path;
use anyhow::{Result, Context};

mod platform;
mod engine;
use engine::html::parser;
use futures::future::ok;
use platform::network;

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
        network::load_html_page(&url_or_path).await?
    } else if Path::new(&url_or_path).exists() {
        println!("loading local file[{}]", url_or_path);
        let bytes = network::load_local_file(&url_or_path).await?;
        String::from_utf8(bytes).context("Warn: dont load UTF-8")?
    } else {
        let html = r#"<!DOCTYPE html>
<html lang="ja">
<head>
    <title>Orinium Browser テストページ</title>
    <!-- コメント -->
</head>
<body>
    <h1>Orinium Browser へようこそ</h1>
    <p>これは <b>テスト</b> ページです。</p>
    <div>
        <p>ネストされた <span>span テキスト</span></p>
        <img src="image.png">
        <br />
        <input type="text" value="テキスト入力" />
        <p>閉じていない段落
    </div>
    <footer>フッターコンテンツ</footer>
</body>
</html>
"#;
        println!("use default html code...");
        html.to_string()
    };

    println!("parsing html code...");
    let mut parser = parser::Parser::new(&html);
    let dom = parser.parse();
    parser::print_dom_tree(&dom,"" , true);
    Ok(())
}

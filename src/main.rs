use std::env;

mod platform;
mod engine;
use engine::html::parser;

#[tokio::main]
async fn main() {
    #[allow(unused)]
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    env_logger::init();
    println!("Hello, Orinium Browser!");

    let html = r#"<!DOCTYPE html>
<html lang="ja">
<head>
    <title>Test</title>
    <!-- コメント -->
</head>
<body>
    <p>Hello <b>World</b></p>
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

    print!("Parsing HTML: {}\n", html);
    let mut parser = parser::Parser::new(html);
    let dom = parser.parse();
    parser::print_dom_tree(&dom,"" , true);
}

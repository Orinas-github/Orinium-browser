use anyhow::Result;
use orinium_browser::engine::html::parser;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

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

    html.to_string();

    log::debug!("Parsing HTML: {html}\n");
    let mut parser = parser::Parser::new(&html);
    let dom = parser.parse();
    parser::print_dom_tree(&dom, &[]);
    Ok(())
}

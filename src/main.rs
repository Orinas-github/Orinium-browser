#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use components::html_dom::HTMLRenderer as html;
use components::network::Net;
use components::javascript::JSEngine;

mod components;

pub fn main() {

}

// html::render(&result_to_string(
//                net.file_get_relative_path("pages/test/testpage.html"),
//            ))
//            .join("\n");

fn result_to_string(result: Result<String, Box<dyn Error>>) -> String {
    match result {
        Ok(string) => string,
        Err(_) => "".to_string(),
    }
}

/*
fn test() -> Vec<String> {
    let js_engine = JSEngine::new();
    // let gui = uisystem;
    // モジュールを初期化
    let htmlcode = r#"<!DOCTYPE html>
<html lang="ja">
    <h1>hello</h1>
    <p><span>goodbye</span></p>
</html>"#;

    // JSを実行
    // js_engine.execute("console.log('Hello, World!');");

    // HTMLを取得、レンダリング、描画
    // html::render(net::fetch("https://example.com").as_str());
    // /*gui.display(*/html::render(&htmlcode)/*)*/;
    return html::render(&htmlcode);
}
*/

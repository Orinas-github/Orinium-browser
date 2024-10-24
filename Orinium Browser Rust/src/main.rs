use javascript::JSEngine;
use network::Fetch as net;
use renderer::HTMLRenderer as html;
use ui::GUI;

mod javascript;
mod network;
mod renderer;
mod ui;

fn main() {
    // 各モジュールを初期化
    let js_engine = JSEngine::new();
    let gui = GUI;

    // JSを実行
    js_engine.execute("console.log('Hello, World!');");

    // HTMLを取得、レンダリング
    html::render(net::fetch("https://example.com").as_str());

    // 描画
    gui.display();
}
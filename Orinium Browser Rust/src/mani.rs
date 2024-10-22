mod javascript;
mod network;
mod render;
mod ui;

use javascript::JSEngine;
use network::Fetch;
use render::HTMLRenderer;
use ui::GUI;

fn main() {
    // 各モジュールを初期化
    let js_engine = JSEngine::new();
    let fetcher = Fetch;
    let renderer = HTMLRenderer;
    let gui = GUI;

    // JSを実行
    js_engine.execute("console.log('Hello, World!');");

    // HTMLを取得、レンダリング
    renderer.render(fetcher.fetch("https://example.com"));

    // 描画
    gui.display();
}
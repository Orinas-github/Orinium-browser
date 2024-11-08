use javascript::JSEngine;
use network::Fetch as net;
use renderer::HTMLRenderer as html;
use ui::GUI as uisystem;

use eframe::{egui::*};

mod javascript;
mod network;
mod renderer;
mod ui;

fn main() {
    // 各モジュールを初期化
    let js_engine = JSEngine::new();
    let gui = uisystem;
    let htmlcode = r#"<!DOCTYPE html>
<html lang="ja">
    <h1>hello</h1>
    <p><span>goodbye</span></p>
</html>"#;

    // JSを実行
    js_engine.execute("console.log('Hello, World!');");

    // HTMLを取得、レンダリング、描画
    // html::render(net::fetch("https://example.com").as_str());
    gui.display(html::render(&htmlcode));

    let options = eframe::NativeOptions::default();
    eframe::run_native("Orinium", options, Box::new(|cc| Box::new(ui::Orinium::new(cc))));
}
<<<<<<< HEAD
=======
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
>>>>>>> 850d6e8411f9468b46e8ca9812a62a4531b30889

use javascript::JSEngine;
use network::Fetch as net;
use renderer::HTMLRenderer as html;
use ui::GUI as uisystem;

mod javascript;
mod network;
mod renderer;
mod ui;

<<<<<<< HEAD


fn main() {

=======
#[derive(Default)]
struct App {
    window: Option<Window>,
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
>>>>>>> 850d6e8411f9468b46e8ca9812a62a4531b30889
}

fn test() {
    let js_engine = JSEngine::new();
    let gui = uisystem;
    // モジュールを初期化
    let htmlcode = r#"<!DOCTYPE html>
<html lang="ja">
    <h1>hello</h1>
    <p><span>goodbye</span></p>
</html>"#;

    // JSを実行
    js_engine.execute("console.log('Hello, World!');");

    // HTMLを取得、レンダリング、描画
    // html::render(net::fetch("https://example.com").as_str());
    /*gui.display(*/html::render(&htmlcode)/*)*/;
}

use javascript::JSEngine;
use network::Net as net;
use renderer::HTMLRenderer as html;
// use ui::GUI as uisystem;

mod javascript;
mod network;
mod renderer;
mod ui;

use iced::widget::{column, container, row, text, button};
use iced::{Fill, Element};

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Display,
}

#[derive(Default)]
struct Counter {
    value: u64,
    maintxt: String,
}

pub fn main() -> iced::Result {
    iced::run("Test page", update, view)
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::Display => {
            counter.maintxt = test().join("\n");
        }
    }
}

fn view(counter: &Counter) -> Element<Message> {
    container(
        column![
            "Top",
            row!["Left", "Right"].spacing(10),
            "Bottom",
            text(counter.value).size(20),
            button("Increment").on_press(Message::Increment),
            text(counter.maintxt.clone()).size(20),
            button("reload").on_press(Message::Display)
        ]
        .spacing(10)

    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

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
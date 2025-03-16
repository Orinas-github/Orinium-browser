use std::error::Error;

use javascript::JSEngine;
use network::Net;
use renderer::HTMLRenderer as html;
// use ui::GUI as uisystem;

mod javascript;
mod network;
mod renderer;
mod ui;

use iced::widget::{column, row, text, button, scrollable, vertical_space};
use iced::Element;

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Display,
}

#[derive(Default)]
struct Testpage { 
    value: u64,
    maintxt: String,
}

pub fn main() -> iced::Result {
    iced::run("Test page", update, view)
}

fn update(testpage: &mut Testpage, message: Message) {
    let net = Net::new();
    match message {
        Message::Increment => testpage.value += 1,
        Message::Display => {
            testpage.maintxt = html::render(&result_to_string(net.file_get_relative_path("pages/test/testpage.html"))).join("\n");
        }
    }
}

fn view(testpage: &Testpage) -> Element<Message> {
    scrollable(
        column![
            row!["  ",text(testpage.maintxt.clone()).size(20),].spacing(10),
            button("reload").on_press(Message::Display),
            vertical_space().height(30),
        ]
        .spacing(10)

    )
    .into()
}

fn result_to_string(result: Result<String, Box<dyn Error>>) -> String {
    match result {
        Ok(string) => string,
        Err(error) => "".to_string()
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
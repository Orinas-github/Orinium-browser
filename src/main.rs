use std::error::Error;

use network::Net;
use renderer::HTMLRenderer as html;
// use ui::GUI as uisystem;

mod javascript;
mod network;
mod renderer;
mod ui;

use iced::widget::{button, column, row, scrollable, text, vertical_space};
use iced::Element;

#[derive(Debug, Clone)]
enum Message {
    Display,
}

#[derive(Default)]
struct Testpage {
    maintxt: String,
}

/*
fn main() {
    // Glutinの初期化
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // Iced の初期化
    let mut iced_application = Application::new(());
    let mut iced_renderer =  iced::Renderer::with_theme(Theme::Light, &display, Size::new(800.0, 600.0));

    // イベントループ
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
            glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
            _ => (),
            },
            _ => (),
        };
    });
}
*/

pub fn main() -> iced::Result {
    iced::run("Test page", update, view)
}

fn update(testpage: &mut Testpage, message: Message) {
    let net = Net::new();
    match message {
        Message::Display => {
            testpage.maintxt = html::render(&result_to_string(
                net.file_get_relative_path("pages/test/testpage.html"),
            ))
            .join("\n");
        }
    }
}

fn view(testpage: &Testpage) -> Element<Message> {
    scrollable(
        column![
            row!["  ", text(testpage.maintxt.clone()).size(20),].spacing(10),
            button("reload").on_press(Message::Display),
            vertical_space().height(30),
        ]
        .spacing(10),
    )
    .into()
}

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

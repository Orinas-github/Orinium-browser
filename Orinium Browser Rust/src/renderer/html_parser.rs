mod javascript;
mod network;

use javascript::JSEngine;
use network::Fetch;

pub struct HTMLRenderer;

impl HTMLRenderer {
    pub fn render(&self, html: &str) {
        // HTMLをレンダリングするためのロジック
        println!("Rendering HTML: {}", html);
    }
}
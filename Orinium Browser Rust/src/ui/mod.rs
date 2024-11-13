use bevy::prelude::*;

pub struct GUI;

impl GUI {
    pub fn display(&self, data: Vec<String>) {
        // GUIを表示するロジック
        println!("Displaying the GUI!");
    }

    fn draw(node: &Node, target: &glium::Frame) {
        if node.istext {
            // テキストを描画するロジックを入れる
        } else {
            // タグに応じた描画を行う
            // 簡易的な四角形を描画したり、テキストを描画するロジックを入れる
        }
    
        // 子ノードも描画
        for child in &node.children {
            draw(child, target);
        }
    }
}
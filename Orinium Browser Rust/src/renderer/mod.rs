use std::collections::HashMap;
use crate::javascript::JSEngine;
use crate::network::Fetch;

pub struct HTMLRenderer;

fn get_nth_char(s: &str, n: usize) -> Option<String> {
    s.chars().nth(n).to_string() // n番目の文字をStringで取得
}

impl HTMLRenderer {


    struct Tagdata {
        fpc: i64
        lpc: i64
        tag: String,
        attr: String,
        //  attr は attribute の略
    }

    pub fn render(html: &str) {
        // HTMLをレンダリングするためのロジック
        println!("Rendering HTML: {}", html);
        // 初期化
        setup();

        while html_pc > html.len(){
            // タグの抽出
            if get_nth_char(html, html_pc) == String::from("<"){
                if get_nth_char(html, html_pc) != String::from("/") and get_nth_char(html, html_pc) != String::from(" ") and get_nth_char(html, html_pc) != String::from("!"){
                    html_pc += 1;
                    while get_nth_char(html, html_pc) != String::from(" ") and get_nth_char(html, html_pc) != String::from(">"){
                        html_tagname = html_tagname + get_nth_char(html, html_pc);
                        // 作業用フラグ
                    }
                }
            }
            html_pc += 1;
        }

    }

    fn setup(){
        // HTMLレンダリングに必要な初期化や設定
        println!("Setting up HTML renderer...");

        pub let mut html_data = HashMap::new
        pub let mut html_guidata = HashMap::new

        pub let mut html_pc i32 = 0;
        let mut html_tagname = String::new();

        println!("dode!");
    }
}
use std::collections::HashMap;
use crate::javascript::JSEngine;
use crate::network::Fetch;

pub struct HTMLRenderer;

struct Tagdata {
    fpc: i64,
    lpc: i64,
    tag: String,
    attr: Vec<String>,
    //  attr は attribute の略
}

enum Node {
    Element { tag: String, children: Vec<Node> },
    Text(String),
}

fn get_nth_string(s: &str, n: usize) -> String {
    s.chars().nth(n).unwrap_or_default().to_string() // n番目の文字をStringで取得
}

fn compare(code: &str, n: usize, txt: &str) -> bool {
    get_nth_string(code, n) == String::from(txt)
}

impl HTMLRenderer {

    pub fn render(html: &str) {
        // HTMLをレンダリングするためのロジック
        println!("Rendering HTML: {}", html);
        // HTMLレンダリングに必要な初期化や設定
        println!("Setting up HTML renderer...");

        // let mut html_data = HashMap::new;
        // let mut html_guidata = HashMap::new;

        let mut html_tag_first: Vec<usize> = Vec::new();
        let mut html_tag_last: Vec<usize> = Vec::new();
        let mut html_tags: Vec<String> = Vec::new();
        let mut html_tagattrs: Vec<String> = Vec::new();

        let mut tag_num = 0;

        let mut html_pc: usize = 0;
        let mut html_tagname = String::new();
        let mut html_tagattr = String::new();

        println!("done!");
        println!("Parse tags...");
        
        // 作業用フラグ



    }
}
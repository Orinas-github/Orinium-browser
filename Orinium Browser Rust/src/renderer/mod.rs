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
        while html_pc < html.chars().count() {
            // タグの抽出
            if compare(html, html_pc,"<") {
                if !compare(html, html_pc + 1, "/") && !compare(html, html_pc + 1, " ") && !compare(html, html_pc + 1, "!") {
                    html_tagname = String::new();
                    html_pc += 1;
                    while !compare(html, html_pc, " ") && !compare(html, html_pc, ">") {
                        html_tagname = html_tagname + &get_nth_string(html, html_pc);
                        html_pc += 1;
                    }
                    html_tag_first.push(html_pc + 1);
                    html_tags.push(html_tagname.clone());
                    if compare(html, html_pc, " ") {
                        html_tagattr = String::new();
                        html_pc += 1;
                        while !compare(html, html_pc, ">") {
                            html_tagattr = html_tagattr + &get_nth_string(html, html_pc);
                            html_pc += 1;
                        }
                        html_tagattrs.push(html_tagattr);
                        *html_tag_first.last_mut().unwrap() = html_pc + 1;
                    } else {
                        html_tagattrs.push(String::new());
                    }
                }
            }
            html_pc += 1;
        }

        println!("done!");
        println!("{}", html_tags.join(", "));
        // 要素を見つける
        for i in 0..html_tags.len() {

            // 作業用フラグ
        }



    }
}
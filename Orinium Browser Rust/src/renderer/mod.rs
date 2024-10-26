use std::collections::HashMap;
use crate::javascript::JSEngine;
use crate::network::Fetch;

pub struct HTMLRenderer;

fn get_nth_string(s: &str, n: usize) -> Option<String> {
    s.chars().nth(n).to_string() // n番目の文字をStringで取得
}

impl HTMLRenderer {


    struct Tagdata {
        fpc: i64
        lpc: i64
        tag: String,
        attr: Vec,
        //  attr は attribute の略
    }

    pub fn render(html: &str) {
        // HTMLをレンダリングするためのロジック
        println!("Rendering HTML: {}", html);
        // 初期化
        setup();

        println!("Parse tags...");
        while html_pc > html.chars().count(){
            // タグの抽出
            if get_nth_string(html, html_pc) == String::from("<"){
                if get_nth_string(html, html_pc) != String::from("/") and get_nth_string(html, html_pc) != String::from(" ") and get_nth_string(html, html_pc) != String::from("!") {
                    html_tagname = "";
                    html_pc += 1;
                    while get_nth_string(html, html_pc) != String::from(" ") and get_nth_string(html, html_pc) != String::from(">"){
                        html_tagname = html_tagname + get_nth_string(html, html_pc);
                        html_pc += 1;
                    }
                    html_ftag.push(html_pc + 1);
                    html_tag.push(html_tagname);
                    if get_nth_string(html, html_pc) != String::from(" ") {
                        html_tagattr = "";
                        html_pc += 1;
                        while get_nth_string(html, html_pc) != String::from(">") {
                            html_tagattr = html_tagattr + get_nth_string(html, html_pc);
                            html_pc += 1;
                        }
                        html_tagattr.push(html_tagattr);
                        html_ftag[html_ftag.len()] = html_pc + 1;
                    } else {
                        html_tagattr = ""
                    }
                }
            }
            html_pc += 1;
        }
        println!("30%");

        // 要素を見つける
        for i in html_tag.len() {
            // 作業用フラグ
        }



    }

    fn setup(){
        // HTMLレンダリングに必要な初期化や設定
        println!("Setting up HTML renderer...");

        pub let mut html_data = HashMap::new
        pub let mut html_guidata = HashMap::new

        let mut html_ftag = Vec::new();
        let mut html_tagname = Vec::new();
        let mut html_tagattr = Vec::new();

        let mut tag_num = 0;

        pub let mut html_pc i32 = 0;
        let mut html_tagname = String::new();
        let mut html_tagattr = String::new();

        println!("dode!");
    }
}
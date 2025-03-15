use std::collections::HashMap;
use crate::javascript::JSEngine;
// use crate::network::Net;

pub struct HTMLRenderer;

/*
struct Tagdata {
    fpc: i64,
    lpc: i64,
    tag: String,
    attr: Vec<String>,
    //  attr は attribute の略
}
*/

/*
enum Node_test {
    Element {
        tag: String,
        attr: Vec<(String, String)>, // 属性名と属性値
        children: Vec<Node_test>,
    },
    Text(String)
}
*/

#[derive(Debug)]
struct Node {
    tag: String,
    id: usize,
    element: String,
    layer: usize,
    parent: String,
    istext: bool
}

/*
enum GUI {
    Element {

    }
}
*/


fn get_nth_string(s: &str, n: usize) -> String {
    s.chars().nth(n).unwrap_or_default().to_string() // n番目の文字をStringで取得
}

fn compare(code: &str, n: usize, txt: &str) -> bool {
    get_nth_string(code, n) == String::from(txt)
}

impl HTMLRenderer {

    pub fn render(html: &str) -> Vec<String> {
        let (tags, attrs, elements, parsed) = Self::parser(html);
        elements
    }

    fn parser(html: &str) -> (Vec<String>, Vec<String>, Vec<String>, Vec<Node>) {
        // HTMLをレンダリングするためのロジック
        // println!("Rendering HTML: {}", html);
        // HTMLレンダリングに必要な初期化や設定
        // println!("Setting up HTML renderer...");

        // let mut html_data = HashMap::new;
        // let mut html_guidata = HashMap::new;

        let mut html_tag_first: Vec<usize> = Vec::new();
        let mut html_tag_last: Vec<usize> = Vec::new();
        // let mut html_tag_id: Vec<usize> = Vec::new();
        let mut html_tags: Vec<String> = Vec::new();
        let mut html_tagattrs: Vec<String> = Vec::new();

        let mut html_layers: Vec<usize> = Vec::new();
        let mut html_layer: usize = 0;
        // let mut html_layer_counter: Vec<Vec> = Vec::new();
        let mut html_parent: Vec<String> = Vec::new();

        // let mut tag_num = 0;

        let mut html_pc: usize = 0;
        let mut html_tagname;
        let mut html_tagattr;

        let mut html_elements: Vec<String> = Vec::new();
        let mut html_elements_bool: Vec<bool> = Vec::new(); 

        let mut parsed_html: Vec<Node> = Vec::new();

        println!("Parsing html...");
        
        while html_pc < html.chars().count() {
            // タグの抽出,要素の取得
            if compare(html, html_pc,"<") {
                if !compare(html, html_pc + 1, " ") && !compare(html, html_pc + 1, "!") {
                    //タグ関連
                    if !compare(html, html_pc + 1, "/") {
                        // 開始タグ
                        html_layer += 1;
                        html_layers.push(html_layer.clone());
                        html_elements_bool.push(true);
                        html_elements.push(String::new());

                        html_tagname = String::new();
                        html_pc += 1;
                        while !compare(html, html_pc, " ") && !compare(html, html_pc, ">") {
                            html_tagname = html_tagname + &get_nth_string(html, html_pc);
                            html_pc += 1;
                        }

                        html_tag_first.push(html_pc + 1);
                        html_tag_last.push(html_pc + 1);
                        html_tags.push(html_tagname.clone());
                        html_parent.push(html_tagname.clone());

                        if compare(html, html_pc, " ") {
                            html_tagattr = String::new();
                            html_pc += 1;
                            while !compare(html, html_pc, ">") {
                                html_tagattr = html_tagattr + &get_nth_string(html, html_pc);                                        html_pc += 1;
                            }
                            html_tagattrs.push(html_tagattr);
                            *html_tag_first.last_mut().unwrap() = html_pc + 1;
                        } else {
                            html_tagattrs.push(String::new());
                        }

                        while !compare(html, html_pc, "<") {
                            html_pc -= 1;
                        }

                        for i in 0..html_elements_bool.len()-1 {
                            if html_elements_bool[i] {
                                html_elements[i] = html_elements[i].clone() + "<";
                            }
                        }


                    } else {
                        // 終了タグ
                        html_layer -= 1;
                        html_pc += 2;
                        html_tagname = String::new();
                        while !compare(html, html_pc, ">") {
                            html_tagname = html_tagname + &get_nth_string(html, html_pc);
                            html_pc += 1;
                        }
                        while !compare(html, html_pc, "<") {
                            html_pc -= 1;
                        }
                        html_pc -= 1;

                        html_parent.pop(); // 最後を削除

                        for i in (0..html_elements_bool.len()).rev() {
                            if html_elements_bool[i] == true {
                                if html_tagname == html_tags[i] {
                                    html_tag_last[i] = html_pc;
                                    html_elements_bool[i] = false;
                                    parsed_html.push(
                                        Node {
                                            tag: html_tags[i].clone(),
                                            id: i,
                                            element: html_elements[i].clone(),
                                            layer: html_layers[i],
                                            parent: html_parent.last().unwrap_or(&String::new()).clone(),
                                            istext: vec!["b","i","u","s","sub","sup","em","strong","dfn","address","blockquote","q","code","center","pre","h1","h2","h3","h4","h5","h6","button","a"].iter().map(|s| s.to_string()).collect::<Vec<_>>().contains(&html_tags[i])
                                        }
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
                if html_elements_bool.len() != 0 {
                    loop {
                        html_pc += 1;
                        for i in 0..html_elements_bool.len()-1 {
                            if html_elements_bool[i] {
                                html_elements[i] = html_elements[i].clone() + &get_nth_string(html, html_pc);
                            }
                        }
    
                        if compare(html, html_pc, ">") {
                            break;
                        }
                    }
                }

            } else {
                // 要素取得
                for i in 0..html_elements_bool.len() {
                    if html_elements_bool[i] {
                        html_elements[i] = html_elements[i].clone() + &get_nth_string(html, html_pc);
                    }
                }
            }
            html_pc += 1;
        }
        /*
        println!("{:?}", html_elements);
        println!("{:?}", html_elements_bool);
        println!("{:?}", html_tags);
        println!("{:?}", html_layers);
        println!("{:?}", html_tagattrs);
        println!("{:?}", parsed_html);
        */
        // println!("Done!");

        return (html_tags, html_tagattrs, html_elements, parsed_html);
    }
}
mod istext;

/// `get_nth_string` 関数は、指定された文字列 `s` の `n` 番目の文字を取得し、
/// それを `String` 型として返します。
///
/// # 引数
/// - `s`: 対象の文字列。
/// - `n`: 取得したい文字のインデックス（0始まり）。
///
/// # 戻り値
/// - `String`: 指定されたインデックスの文字を `String` 型で返します。
///   インデックスが範囲外の場合は空の文字列を返します。
fn get_nth_string(s: &str, n: usize) -> String {
    s.chars().nth(n).unwrap_or_default().to_string() // n番目の文字をStringで取得
}

/// `compare` 関数は、指定された文字列 `code` の `n` 番目の文字が
/// 指定された文字列 `txt` と一致するかどうかを判定します。
///
/// # 引数
/// - `code`: 対象の文字列。
/// - `n`: 判定したい文字のインデックス（0始まり）。
/// - `txt`: 比較対象の文字列。
///
/// # 戻り値
/// - `bool`: 一致する場合は `true`、それ以外は `false` を返します。
fn compare(code: &str, n: usize, txt: &str) -> bool {
    get_nth_string(code, n) == String::from(txt)
}

/// `HTMLRenderer` 構造体は、HTML を解析し、
/// テキスト要素を抽出するための機能を提供します。
pub struct HTMLRenderer;

/// `Node` 構造体は、HTML のノードを表現するためのデータ構造です。
/// 各ノードはタグ名、ID、要素、階層、親ノード、子ノード、
/// テキストノードかどうか、表示されるかどうかの情報を持ちます。
#[derive(Debug)]
#[allow(unused)]
struct Node {
    /// ノードのタグ名 (例: "div", "span")。
    tag: String,
    /// ノードの一意の識別子。
    id: usize,
    /// ノードの内容または要素 (例: テキストや属性値)。
    element: String,
    /// ノードの階層レベル (ルートが 0)。
    layer: usize,
    /// 親ノードの識別子。
    parent: String,
    /// 子ノードの識別子のリスト。
    children: Vec<String>,
    /// ノードがテキストノードかどうかを示すフラグ。
    istext: bool,
    /// ノードが表示されるかどうかを示すフラグ。
    isdisplay: bool,
}

impl HTMLRenderer {
    /// `render` 関数は、指定された HTML を解析し、
    /// テキスト要素を抽出して返します。
    ///
    /// # 引数
    /// - `html`: 解析対象の HTML 文字列。
    ///
    /// # 戻り値
    /// - `Vec<String>`: 抽出されたテキスト要素のリスト。
    pub fn render(html: &str) -> Vec<String> {
        let (_tags, _attrs, _elements, parsed) = Self::parser(html);
        let mut text: Vec<String> = Vec::new();
        /*
        for i in 0..parsed.len() {
            let mut tag_name = parsed[i].tag;
            if tag_name == "nojavascript" {
                break;
            }
        }
        */
        for i in 0..parsed.len() {
            if parsed[i].istext {
                text.push(parsed[i].element.clone());
            }
        }
        text
    }

    /// `parser` 関数は、指定された HTML を解析し、
    /// タグ、属性、要素、および解析されたノード情報を返します。
    ///
    /// # 引数
    /// - `html`: 解析対象の HTML 文字列。
    ///
    /// # 戻り値
    /// - `(Vec<String>, Vec<String>, Vec<String>, Vec<Node>)`:
    ///   タグ、属性、要素、および解析されたノード情報を含むタプル。
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
        let mut html_parent_i: usize = 0;

        // let mut tag_num = 0;

        let mut html_pc: usize = 0;
        let mut html_tagname;
        let mut html_tagattr;

        let mut html_elements: Vec<String> = Vec::new();
        let mut html_elements_bool: Vec<bool> = Vec::new();

        let mut parsed_html: Vec<Node> = Vec::new();

        while html_pc < html.chars().count() {
            // タグの抽出,要素の取得
            if compare(html, html_pc, "<") {
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
                        html_parent_i = parsed_html.len() as usize;

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

                        while !compare(html, html_pc, "<") {
                            html_pc -= 1;
                        }

                        for i in 0..html_elements_bool.len() - 1 {
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

                        for id in (0..html_elements_bool.len()).rev() {
                            if html_elements_bool[id] == true {
                                if html_tagname == html_tags[id] {
                                    html_tag_last[id] = html_pc;
                                    html_elements_bool[id] = false;

                                    let istext = istext::is_text(&html_tags, id);

                                    let parent: String =
                                        html_parent.last().unwrap_or(&String::new()).clone();

                                    let node = Node {
                                        tag: html_tags[id].clone(),
                                        id,
                                        element: html_elements[id].clone(),
                                        layer: html_layers[id],
                                        parent: parent.clone(),
                                        children: Vec::<String>::new(),
                                        istext,
                                        isdisplay: true,
                                    };

                                    parsed_html.push(node);

                                    parsed_html[html_parent_i]
                                        .children
                                        .push(html_tags[id].clone()); // Children 追加

                                    break;
                                }
                            }
                        }
                    }
                }
                if html_elements_bool.len() != 0 {
                    loop {
                        html_pc += 1;

                        for i in 0..html_elements_bool.len() - 1 {
                            if html_elements_bool[i] {
                                html_elements[i] =
                                    html_elements[i].clone() + &get_nth_string(html, html_pc);
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
                        html_elements[i] =
                            html_elements[i].clone() + &get_nth_string(html, html_pc);
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

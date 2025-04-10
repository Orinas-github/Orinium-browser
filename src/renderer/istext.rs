/// 指定されたHTMLタグがテキスト要素かどうかを判定する関数。
///
/// # 引数
/// - `html_tags`: HTMLタグのリストを表す文字列スライス。
/// - `id`: 判定対象のタグのインデックス。
///
/// # 戻り値
/// - `true`: 指定されたタグがテキスト要素の場合。
/// - `false`: 指定されたタグがテキスト要素でない場合。
///
/// # 注意
/// - `id`が`html_tags`の範囲外の場合、この関数はパニックする可能性があります。
///
/// # 補足
/// - テキスト要素として判定されるタグは、`istext`配列に定義されています。
/// - この配列は将来的に拡張される可能性があるため、別ファイルに分離されています。
pub fn is_text(html_tags: &[String], id: usize) -> bool {
    let istext = &[
        "b",
        "i",
        "u",
        "s",
        "sub",
        "sup",
        "em",
        "strong",
        "dfn",
        "address",
        "blockquote",
        "q",
        "code",
        "center",
        "pre",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "button",
    ];

    istext
        .iter()
        .map(|tag| tag.to_string())
        .collect::<Vec<String>>()
        .contains(&html_tags[id])
}

pub fn is_text(html_tags: &[String], id: usize) -> bool {

    // 多分絶対今後増えると思うから別ファイルにしとかないとつらいことになる
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

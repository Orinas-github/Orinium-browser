use crate::engine::html::tokenizer::{Attribute, Token, Tokenizer};
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
pub enum NodeType {
    Document,
    Element {
        tag_name: String,
        attributes: Vec<Attribute>,
    },
    Text(String),
    Comment(String),
    Doctype {
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
    },
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

pub type NodeRef = Rc<RefCell<Node>>;

pub struct Parser<'a> {
    tokenizer: crate::engine::html::tokenizer::Tokenizer<'a>,
    stack: Vec<NodeRef>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let document = Rc::new(RefCell::new(Node {
            node_type: NodeType::Document,
            children: vec![],
            parent: None,
        }));

        Self {
            tokenizer: Tokenizer::new(input),
            stack: vec![document],
        }
    }

    pub fn parse(&mut self) -> NodeRef {
        while let Some(token) = self.tokenizer.next_token() {
            //println!("---");
            //println!("Processing token: {token:?}");
            match token {
                Token::StartTag { .. } => self.handle_start_tag(token),
                Token::EndTag { .. } => self.handle_end_tag(token),
                Token::Doctype { .. } => self.handle_doctype(token),
                Token::Comment(_) => self.handle_comment(token),
                Token::Text(_) => self.handle_text(token),
            }
            //print_dom_tree(&self.stack.last().unwrap(), &[]);
        }

        Rc::clone(&self.stack[0])
    }

    fn handle_start_tag(&mut self, token: Token) {
        if let Token::StartTag {
            name,
            attributes,
            self_closing,
        } = token
        {
            let parent = Rc::clone(self.stack.last().unwrap());
            if self.check_start_tag_with_invalid_nesting(&name, &parent) {
                todo!("Found invalid nesting for <{name}>. Auto-closing the previous tag.");
                //self.handle_end_tag();
            }
            let new_node = Rc::new(RefCell::new(Node {
                node_type: NodeType::Element {
                    tag_name: name,
                    attributes,
                },
                children: vec![],
                parent: Some(Rc::clone(&parent)),
            }));

            parent.borrow_mut().children.push(Rc::clone(&new_node));

            // Self-closing タグは stack に push しない
            if !self_closing {
                self.stack.push(new_node);
            }
        }
    }

    fn handle_end_tag(&mut self, token: Token) {
        if let Token::EndTag { name } = token {
            while let Some(top) = self.stack.pop() {
                if let NodeType::Element { tag_name, .. } = &top.borrow().node_type {
                    if tag_name == &name {
                        break;
                    }
                }
            }
        }
    }

    fn handle_text(&mut self, token: Token) {
        if let Token::Text(data) = token {
            let parent = Rc::clone(self.stack.last().unwrap());
            // 親ノードが pre, textarea, script, style でない場合、空白改行を無視する
            if let Some(parent_node) = &parent.borrow().parent {
                let parent_node_borrow = parent_node.borrow();
                if let NodeType::Element { tag_name, .. } = &parent_node_borrow.node_type {
                    if !matches!(tag_name.as_str(), "pre" | "textarea" | "script" | "style")
                        && data.trim().is_empty()
                    {
                        return;
                    }
                } else if data.trim().is_empty() {
                    return;
                }
            } else if data.trim().is_empty() {
                return;
            }
            let text_node = Rc::new(RefCell::new(Node {
                node_type: NodeType::Text(data),
                children: vec![],
                parent: Some(Rc::clone(&parent)),
            }));
            parent.borrow_mut().children.push(text_node);
        }
    }

    fn handle_comment(&mut self, token: Token) {
        if let Token::Comment(data) = token {
            let parent = Rc::clone(self.stack.last().unwrap());
            let comment_node = Rc::new(RefCell::new(Node {
                node_type: NodeType::Comment(data),
                children: vec![],
                parent: Some(Rc::clone(&parent)),
            }));
            parent.borrow_mut().children.push(comment_node);
        }
    }

    fn handle_doctype(&mut self, token: Token) {
        if let Token::Doctype {
            name,
            public_id,
            system_id,
            ..
        } = token
        {
            let parent = Rc::clone(self.stack.last().unwrap());
            let doctype_node = Rc::new(RefCell::new(Node {
                node_type: NodeType::Doctype {
                    name,
                    public_id,
                    system_id,
                },
                children: vec![],
                parent: Some(Rc::clone(&parent)),
            }));
            parent.borrow_mut().children.push(doctype_node);
        }
    }

    fn check_start_tag_with_invalid_nesting(&self, name: &String, parent: &NodeRef) -> bool {
        if let NodeType::Element { tag_name, .. } = &parent.borrow().node_type {
            // <p> の中に <p> が来た場合、前の <p> を閉じる
            if tag_name == "p" && name == "p" {
                return true;
            }
            // <li> の中に <li> が来た場合、前の <li> を閉じる
            if tag_name == "li" && name == "li" {
                return true;
            }
            // <dt> の中に <dt> または <dd> が来た場合、前の <dt> を閉じる
            if tag_name == "dt" && (name == "dt" || name == "dd") {
                return true;
            }
            // <dd> の中に <dt> または <dd> が来た場合、前の <dd> を閉じる
            if tag_name == "dd" && (name == "dt" || name == "dd") {
                return true;
            }
            // <option> の中に <option> が来た場合、前の <option> を閉じる
            if tag_name == "option" && name == "option" {
                return true;
            }
        }
    false
    }
}

/// 再帰表示用の DOM デバッグ関数（祖先情報付きで罫線を正確に描画）
pub fn print_dom_tree(node: &NodeRef, ancestors_last: &[bool]) {
    let n = node.borrow();

    // ├── か └── を決める（自身の最後かどうかは ancestors_last の最後で判断）
    let is_last = *ancestors_last.last().unwrap_or(&true);
    let connector = if ancestors_last.is_empty() {
        ""
    } else if is_last {
        "└── "
    } else {
        "├── "
    };

    // 現在の prefix を構築
    let mut prefix = String::new();
    for &ancestor_last in &ancestors_last[..ancestors_last.len().saturating_sub(1)] {
        prefix.push_str(if ancestor_last { "    " } else { "│   " });
    }

    // ノード情報の表示
    match &n.node_type {
        NodeType::Document => println!("{prefix}{connector}Document"),
        NodeType::Element {
            tag_name,
            attributes,
        } => {
            let attrs_str = if attributes.is_empty() {
                "".to_string()
            } else {
                let attrs_list = attributes
                    .iter()
                    .map(|attr| format!("{}=\"{}\"", attr.name, attr.value))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(" [{attrs_list}]")
            };
            println!("{prefix}{connector}Element: {tag_name}{attrs_str}");
        }
        NodeType::Text(data) => {
            let trimmed = data.trim();
            if !trimmed.is_empty() {
                println!("{prefix}{connector}Text: {trimmed:?}");
            }
        }
        NodeType::Comment(data) => println!("{prefix}{connector}Comment: {data:?}"),
        NodeType::Doctype {
            name,
            public_id,
            system_id,
        } => {
            println!("{prefix}{connector}Doctype: name={name:?}, public_id={public_id:?}, system_id={system_id:?}");
        }
    }

    // 子ノードを再帰
    let child_count = n.children.len();
    for (i, child) in n.children.iter().enumerate() {
        let child_is_last = i == child_count - 1;

        // ancestors_last を更新して渡す
        let mut new_ancestors = ancestors_last.to_vec();
        new_ancestors.push(child_is_last);

        print_dom_tree(child, &new_ancestors);
    }
}

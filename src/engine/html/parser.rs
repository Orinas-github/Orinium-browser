use std::rc::Rc;
use std::cell::RefCell;
use crate::engine::html::tokenizer::{Token, Tokenizer, Attribute};

#[allow(dead_code)]
#[derive(Debug)]
pub enum NodeType {
    Document,
    Element { tag_name: String, attributes: Vec<Attribute> },
    Text(String),
    Comment(String),
    Doctype { name: Option<String>, public_id: Option<String>, system_id: Option<String> },
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

#[allow(dead_code)]
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
            log::debug!("Processing token: {:?}", token);
            match token {
                Token::StartTag { .. } => self.handle_start_tag(token),
                Token::EndTag { .. } => self.handle_end_tag(token),
                Token::Doctype { .. } => self.handle_doctype(token),
                Token::Comment(_) => self.handle_comment(token),
                Token::Text(_) => self.handle_text(token),
            }
        }

        Rc::clone(&self.stack[0])
    }

    fn handle_start_tag(&mut self, token: Token) {
        if let Token::StartTag { name, attributes, self_closing } = token {
            let parent = Rc::clone(self.stack.last().unwrap());
            let new_node = Rc::new(RefCell::new(Node {
                node_type: NodeType::Element { tag_name: name, attributes },
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
                    if tag_name == &name { break; }
                }
            }
        }
    }

    fn handle_text(&mut self, token: Token) {
        if let Token::Text(data) = token {
            let parent = Rc::clone(self.stack.last().unwrap());
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
        if let Token::Doctype { name, public_id, system_id, ..} = token {
            let parent = Rc::clone(self.stack.last().unwrap());
            let doctype_node = Rc::new(RefCell::new(Node {
                node_type: NodeType::Doctype { name, public_id, system_id },
                children: vec![],
                parent: Some(Rc::clone(&parent)),
            }));
            parent.borrow_mut().children.push(doctype_node);
        }
    }

}

/// DOMTreeを見やすく表示するデバッグ用関数
/// `prefix` は親からの接続線の情報を渡す
pub fn print_dom_tree(node: &NodeRef, prefix: &str, is_last: bool) {
    let n = node.borrow();

    // connector を決める
    let connector = if prefix.is_empty() { "" } else if is_last { "└── " } else { "├── " };

    match &n.node_type {
        NodeType::Document => println!("{}{}Document", prefix, connector),
        NodeType::Element { tag_name, attributes } => {
            let attrs_str = if attributes.is_empty() {
                "".to_string()
            } else {
                let attrs_list = attributes.iter()
                    .map(|attr| format!("{}=\"{}\"", attr.name, attr.value))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(" [{}]", attrs_list)
            };
            println!("{}{}Element: {}{}", prefix, connector, tag_name, attrs_str);
        }
        NodeType::Text(data) => {
            // 空白や改行だけの Text は簡略化
            let trimmed = data.trim();
            if !trimmed.is_empty() {
                println!("{}{}Text: {:?}", prefix, connector, trimmed);
            }
        }
        NodeType::Comment(data) => println!("{}{}Comment: {:?}", prefix, connector, data),
        NodeType::Doctype { name, public_id, system_id } => {
            println!("{}{}Doctype: name={:?}, public_id={:?}, system_id={:?}", prefix, connector, name, public_id, system_id);
        }
    }

    let child_count = n.children.len();
    for (i, child) in n.children.iter().enumerate() {
        let child_is_last = i == child_count - 1;

        // 新しい prefix を作成
        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        print_dom_tree(child, &new_prefix, child_is_last);
    }
}

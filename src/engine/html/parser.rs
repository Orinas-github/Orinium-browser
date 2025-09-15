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

/// DOM ツリーをインデント付きで表示するデバッグ用関数
pub fn print_dom(node: &NodeRef, indent: usize) {
    let pad = "  ".repeat(indent);
    let n = node.borrow();

    match &n.node_type {
        NodeType::Document => println!("{}Document", pad),
        NodeType::Element { tag_name, attributes } => {
            println!("{}Element: {} attrs={:?}", pad, tag_name, attributes);
        }
        NodeType::Text(data) => println!("{}Text: {:?}", pad, data),
        NodeType::Comment(data) => println!("{}Comment: {:?}", pad, data),
        NodeType::Doctype { name, public_id, system_id } => {
            println!("{}Doctype: name={:?}, public_id={:?}, system_id={:?}", pad, name, public_id, system_id);
        }
    }

    // 子ノードを再帰的に表示
    for child in &n.children {
        print_dom(child, indent + 1);
    }
}

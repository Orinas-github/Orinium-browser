#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Doctype {
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
        force_quirks: bool,
    },
    StartTag {
        name: String,
        attributes: Vec<Attribute>,
        self_closing: bool,
    },
    EndTag {
        name: String,
    },
    Comment(String),
    Text(String),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TokenizerState {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    SelfClosingStartTag,
    CommentStartDash,
    Comment,
    CommentEndDash,
    BogusComment,
    Doctype,
}

#[allow(dead_code)]
pub struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
    token: Option<Token>,
    state: TokenizerState,
    current_token: Option<Token>,
    current_attribute: Option<Attribute>,
    buffer: String,
}

#[allow(dead_code)]
impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            pos: 0,
            token: None,
            state: TokenizerState::Data,
            current_token: None,
            current_attribute: None,
            buffer: String::new(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            self.pos += c.len_utf8();

            match self.state {
                TokenizerState::Data => self.state_data(c),
                TokenizerState::TagOpen => self.state_tag_open(c),
                TokenizerState::TagName => self.state_tag_name(c),
                //TokenizerState::BeforeAttributeName => self.state_before_attribute_name(c),
                //TokenizerState::AttributeName => self.state_attribute_name(c),
                //TokenizerState::BeforeAttributeValue => self.state_before_attribute_value(c),
                //TokenizerState::AttributeValueDoubleQuoted => self.state_attribute_value_double_quoted(c),
                //TokenizerState::AttributeValueSingleQuoted => self.state_attribute_value_single_quoted(c),
                //TokenizerState::AttributeValueUnquoted => self.state_attribute_value_unquoted(c),
                //TokenizerState::SelfClosingStartTag => self.state_self_closing_start_tag(c),
                //TokenizerState::EndTagOpen => self.state_end_tag_open(c),
                //TokenizerState::Comment | TokenizerState::CommentStartDash | TokenizerState::CommentEndDash => self.state_comment(c),
                //TokenizerState::Doctype => self.state_doctype(c),
                //TokenizerState::BogusComment => self.state_bogus_comment(c),
                _ => {
                    // 未実装の状態は無視してData状態に戻る
                    self.state = TokenizerState::Data;
                }
            }

            if let Some(token) = self.token.take() {
                return Some(token);
            }
        }

        None
    }

    fn commit_token(&mut self) {
        self.token = self.current_token.take();
    }

    fn state_data(&mut self, c: char) {
        match c {
            '<' => self.state = TokenizerState::TagOpen,
            '&' => {
                todo!(); // エスケープ処理 (未実装)
            }
            _ => {
                self.buffer.push(c);
                if let Some(Token::Text(ref mut text)) = self.current_token {
                    text.push(c);
                } else {
                    self.current_token = Some(Token::Text(c.to_string()));
                }
            }
        }
    }

    fn state_tag_open(&mut self, c: char) {
        match c {
            '/' => self.state = TokenizerState::EndTagOpen,
            '!' => {
                if c == '-' {
                    self.state = TokenizerState::CommentStartDash;
                } else if self.input[self.pos..].to_lowercase().starts_with("doctype") {
                    self.pos += 6;
                    self.state = TokenizerState::Doctype;
                } else {
                    self.state = TokenizerState::BogusComment;
                }
            }
            c if c.is_ascii_alphabetic() => { // cがアルファベットの場合
                self.state = TokenizerState::TagName;
                self.buffer.push(c);
                self.current_token = Some(Token::StartTag {
                    name: c.to_string(),
                    attributes: Vec::new(),
                    self_closing: false,
                });
            }
            _ => {
                // テキストノードとして処理
                self.buffer.push('<');
                self.buffer.push(c);
                if let Some(Token::Text(ref mut text)) = self.current_token {
                    text.push('<');
                    text.push(c);
                } else {
                    self.current_token = Some(Token::Text(format!("<{}", c)));
                }
                self.state = TokenizerState::Data;
            }
        }
    }

    fn state_tag_name(&mut self, c: char) {
        match c {
            c if c.is_whitespace() => self.state = TokenizerState::BeforeAttributeName,
            '/' => self.state = TokenizerState::SelfClosingStartTag,
            '>' => {
                self.commit_token();
                self.state = TokenizerState::Data;
            }
            c if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':' => {
                self.buffer.push(c);
                if let Some(Token::StartTag { ref mut name, .. }) = self.current_token {
                    name.push(c);
                } else if let Some(Token::EndTag { ref mut name }) = self.current_token {
                    name.push(c);
                }
            }
            _ => {// 不正な文字
                // タグの終了として処理
                self.commit_token();
                self.state = TokenizerState::Data;
            }
        }
    }
}

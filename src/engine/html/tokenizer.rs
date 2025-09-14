#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

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
    CommentStart,
    Comment,
    Doctype,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
    state: TokenizerState,
    current_token: Option<Token>,
    current_attribute: Option<Attribute>,
    buffer: String,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            pos: 0,
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
                TokenizerState::BeforeAttributeName => self.state_before_attribute_name(c),
                TokenizerState::AttributeName => self.state_attribute_name(c),
                TokenizerState::BeforeAttributeValue => self.state_before_attribute_value(c),
                TokenizerState::AttributeValueDoubleQuoted => self.state_attribute_value_double_quoted(c),
                TokenizerState::AttributeValueSingleQuoted => self.state_attribute_value_single_quoted(c),
                TokenizerState::AttributeValueUnquoted => self.state_attribute_value_unquoted(c),
                TokenizerState::AfterAttributeValueQuoted => self.state_after_attribute_value_quoted(c),
                TokenizerState::SelfClosingStartTag => self.state_self_closing_start_tag(c),
                TokenizerState::EndTagOpen => self.state_end_tag_open(c),
                TokenizerState::CommentStart | TokenizerState::Comment | TokenizerState::CommentStartDash | TokenizerState::CommentEndDash | TokenizerState::CommentEnd => self.state_comment(c),
                TokenizerState::Doctype => self.state_doctype(c),
                TokenizerState::BogusComment => self.state_bogus_comment(c),
            }

            if let Some(token) = self.current_token.take() {
                return Some(token);
            }
        }

        None
    }

    fn state_data(&mut self, c: char) {
        match c {
            '<' => self.state = TokenizerState::TagOpen,
            '&' => {
                TODO!(); // エスケープ処理 (未実装)
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
                if self.input[self.pos..].starts_with("--") {
                    self.pos += 2;
                    self.state = TokenizerState::CommentStart;
                } else if self.input[self.pos..].to_lowercase().starts_with("doctype") {
                    self.pos += 7;
                    self.state = TokenizerState::Doctype;
                } else {
                    self.state = TokenizerState::BogusComment;
                }
            }
            c if c.is_ascii_alphabetic() => {
                self.state = TokenizerState::TagName;
                self.buffer.push(c);
                self.current_token = Some(Token::StartTag {
                    name: c.to_string(),
                    attributes: Vec::new(),
                    self_closing: false,
                });
            }
            _ => {
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
}

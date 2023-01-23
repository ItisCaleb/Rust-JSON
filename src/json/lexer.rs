use std::collections::VecDeque;

use crate::json::TokenType;

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) position: usize,
    pub(crate) text: String,
}
impl Token {
    pub(crate) fn new(token_type: TokenType, text: &str, position: usize) -> Token {
        Token {
            token_type,
            position,
            text: text.to_string(),
        }
    }
}

pub(crate) struct Lexer {
    position: usize,
    input: Box<[u8]>,
    tokens: VecDeque<Token>,
    diagnostic: Vec<String>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            position: 0,
            input: input.as_bytes().into(),
            tokens: VecDeque::new(),
            diagnostic: vec![],
        }
    }

    pub fn lex(&mut self) -> (&mut VecDeque<Token>, &Vec<String>) {
        while self.position < self.input.len() && self.diagnostic.is_empty() {
            if self.current().is_ascii_whitespace() {
                self.trim_white();
                continue;
            }
            if self.current() == b'-' || self.current().is_ascii_digit() {
                self.parse_numeric();
                continue;
            }
            if self.current() == b'"' {
                self.parse_string();
                continue;
            }
            if self.current().is_ascii_alphabetic() {
                self.parse_keyword();
                continue;
            }
            self.parse_symbol();
        }
        self.tokens
            .push_back(Token::new(TokenType::Eof, "End of JSON", self.position));
        (&mut self.tokens, &self.diagnostic)
    }
    fn trim_white(&mut self) {
        while self.current().is_ascii_whitespace() {
            self.next();
        }
    }

    fn parse_numeric(&mut self) {
        let start = self.position;
        let mut float = false;
        if self.current() == b'-' {
            self.next();
        }
        if !self.current().is_ascii_digit() {
            self.tokens
                .push_back(Token::new(TokenType::Int, "", self.position));
            self.error(format!(
                "Missing number after minus sign at position {}",
                self.position
            ));
            return;
        }
        self.get_digits();
        if self.current() == b'.' {
            float = true;
            self.next();
            self.get_digits();
        }
        if self.current() == b'e' || self.current() == b'E' {
            self.next();
            if self.current() == b'-' || self.current() == b'+' {
                if self.current() == b'-' {
                    float = true;
                }
                self.next();
            }
            self.get_digits();
        }
        let num = &self.input[start..self.position];
        let num = String::from_utf8_lossy(num);
        let num_type = if float {
            TokenType::Float
        } else {
            TokenType::Int
        };
        self.tokens
            .push_back(Token::new(num_type, &num, self.position));
    }
    fn get_digits(&mut self) {
        while self.current().is_ascii_digit() {
            self.next();
        }
    }

    fn parse_string(&mut self) {
        self.next();
        //only contains literal
        let start = self.position;
        while self.current() != b'"' {
            if self.current() == b'\\' {
                self.next();
                match self.current() {
                    b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' | b'u' => self.next(),
                    c => {
                        self.error(format!(
                            "Unexpected control character '\\{}' at position {}",
                            c as char, self.position
                        ));
                        self.next()
                    }
                }
            } else if self.position >= self.input.len() {
                self.error(format!("Missing quote at position {}", self.position));
                return;
            } else {
                self.next();
            }
        }
        self.next();
        let text = &self.input[start..self.position - 1];
        let text = String::from_utf8_lossy(text);
        self.tokens
            .push_back(Token::new(TokenType::String, &text, self.position));
    }

    fn parse_keyword(&mut self) {
        let start = self.position;
        while self.current().is_ascii_alphabetic() {
            self.next();
        }

        let id = &self.input[start..self.position];
        let id = String::from_utf8_lossy(id);
        if &id == "true" || &id == "false" {
            self.tokens
                .push_back(Token::new(TokenType::Bool, &id, self.position));
            return;
        }
        if &id == "null" {
            self.tokens
                .push_back(Token::new(TokenType::Null, &id, self.position));
            return;
        }
        self.error(format! {"Unexpected word \"{}\" start from position {}",id,start});
    }

    fn parse_symbol(&mut self) {
        let token = match self.current() {
            b'[' => Token::new(TokenType::LBracket, "[", self.position),
            b']' => Token::new(TokenType::RBracket, "]", self.position),
            b'{' => Token::new(TokenType::LCurlyBracket, "{", self.position),
            b'}' => Token::new(TokenType::RCurlyBracket, "}", self.position),
            b':' => Token::new(TokenType::Colon, ":", self.position),
            b',' => Token::new(TokenType::Comma, ",", self.position),
            c => {
                self.error(format!(
                    "Unexpected symbol '{}' at position {}",
                    c as char, self.position
                ));
                Token::new(TokenType::Error, "", self.position)
            }
        };
        self.tokens.push_back(token);
        self.next();
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn current(&mut self) -> u8 {
        if self.position < self.input.len() {
            self.input[self.position]
        } else {
            b'\0'
        }
    }
    fn error(&mut self, diagnostic: String) {
        self.diagnostic.push(diagnostic);
    }
}

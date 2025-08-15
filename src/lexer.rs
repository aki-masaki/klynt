#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Fn,
    Return,
    Identifier,
    LBrace,
    RBrace,
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    lexeme: String,
    line: usize,
    column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let current_char = self.current_char();
        let mut token: Option<Token> = None;

        if self.position == self.input.len() {
            return None
        }

        if self.lookup_ahead(String::from("fn")) {
            token = Some(self.new_token(TokenKind::Fn, String::from("fn")));

            self.advance(2);
        } else if self.lookup_ahead(String::from("return")) {
            token = Some(self.new_token(TokenKind::Return, String::from("return")));

            self.advance(6);
        } else if let Some(char) = current_char {
            match char {
                '{' => {
                    token = Some(self.new_token(TokenKind::LBrace, String::from('{')));
                    self.advance(1);
                }
                '}' => {
                    token = Some(self.new_token(TokenKind::RBrace, String::from('}')));
                    self.advance(1);
                }
                ';' => {
                    token = Some(self.new_token(TokenKind::Semicolon, String::from(';')));
                    self.advance(1);
                }
                c if c.is_alphanumeric() => {
                    let mut identifier = String::from(c);
                    let mut length = 1;

                    while let Some(next_char) = self.peek(length) {
                        if !(next_char.is_alphanumeric() || next_char == '_') {
                            break;
                        }

                        identifier.push(next_char);
                        length += 1;
                    }

                    token = Some(self.new_token(TokenKind::Identifier, identifier));

                    self.advance(length);
                }
                _ => {
                    println!("Unexpected character: \"{char}\"")
                }
            };
        }

        token
    }

    pub fn advance(&mut self, n: usize) -> Option<char> {
        for _ in 0..n {
            if self.position >= self.input.len() {
                return None;
            }

            let c = self.input[self.position];

            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            self.position += 1;
        }

        self.current_char()
    }

    pub fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    pub fn peek(&self, n: usize) -> Option<char> {
        if self.position + n < self.input.len() {
            return Some(self.input[self.position + n]);
        }

        None
    }

    pub fn lookup_ahead(&self, s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();

        for (i, item) in chars.iter().enumerate() {
            if self.input[self.position + i] != *item {
                return false;
            }
        }

        true
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    pub fn new_token(&self, kind: TokenKind, lexeme: String) -> Token {
        Token {
            kind,
            lexeme,
            line: self.line,
            column: self.column,
        }
    }
}

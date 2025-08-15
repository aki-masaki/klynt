#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Fn,
    Identifier,
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

        if self.lookup_ahead(String::from("fn")) {
            token = Some(Token {
                kind: TokenKind::Fn,
                lexeme: String::from("fn"),
                line: self.line,
                column: self.column,
            });

            self.advance(2);
        } else {
            match current_char {
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

                    token = Some(Token {
                        kind: TokenKind::Identifier,
                        lexeme: identifier,
                        line: self.line,
                        column: self.column,
                    });

                    self.advance(length);
                }
                _ => {}
            };
        }

        token
    }

    pub fn advance(&mut self, n: usize) -> Option<char> {
        self.position += n;

        for i in 0..n {
            if let Some(c) = self.peek(i) {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
            }
        }

        if self.position < self.input.len() {
            return Some(self.input[self.position]);
        }

        None
    }

    pub fn current_char(&self) -> char {
        self.input[self.position]
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
        if self.current_char() == ' ' {
            self.advance(1);
        }

        while let Some(' ') = self.peek(1) {
            self.advance(1);
        }
    }
}

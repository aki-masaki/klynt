#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Fn,
    Return,
    Let,
    Set,
    Const,
    Call,
    When,
    OrWhen,
    Or,
    Obj,
    Colon,
    Comma,
    Identifier,
    Number,
    StringLiteral,
    Plus,
    Minus,
    Times,
    Divided,
    Gt,
    Lt,
    Equal,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LPar,
    RPar,
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    start_column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            start_column: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let current_char = self.current_char();

        self.start_column = self.column;

        if self.position == self.input.len() {
            return None;
        }

        if self.lookup_ahead("fn") {
            Some(self.new_token(TokenKind::Fn, "fn"))
        } else if self.lookup_ahead("ret") {
            Some(self.new_token(TokenKind::Return, "ret"))
        } else if self.lookup_ahead("let") {
            Some(self.new_token(TokenKind::Let, "let"))
        } else if self.lookup_ahead("set") {
            Some(self.new_token(TokenKind::Set, "set"))
        } else if self.lookup_ahead("const") {
            Some(self.new_token(TokenKind::Const, "const"))
        } else if self.lookup_ahead("call") {
            Some(self.new_token(TokenKind::Call, "call"))
        } else if self.lookup_ahead("when") {
            Some(self.new_token(TokenKind::When, "when"))
        } else if self.lookup_ahead("orwhen") {
            Some(self.new_token(TokenKind::OrWhen, "orwhen"))
        } else if self.lookup_ahead("or") {
            Some(self.new_token(TokenKind::Or, "or"))
        } else if let Some(char) = current_char {
            match char {
                '{' => Some(self.new_token(TokenKind::LBrace, "{")),
                '}' => Some(self.new_token(TokenKind::RBrace, "}")),
                ';' => Some(self.new_token(TokenKind::Semicolon, ";")),
                ':' => Some(self.new_token(TokenKind::Colon, ":")),
                ',' => Some(self.new_token(TokenKind::Comma, ",")),
                '+' => Some(self.new_token(TokenKind::Plus, "+")),
                '-' => Some(self.new_token(TokenKind::Minus, "-")),
                '*' => Some(self.new_token(TokenKind::Times, "*")),
                '/' => Some(self.new_token(TokenKind::Divided, "/")),
                '>' => Some(self.new_token(TokenKind::Gt, ">")),
                '<' => Some(self.new_token(TokenKind::Lt, "<")),
                '=' => Some(self.new_token(TokenKind::Equal, "=")),
                '[' => Some(self.new_token(TokenKind::LBracket, "[")),
                ']' => Some(self.new_token(TokenKind::RBracket, "]")),
                '(' => Some(self.new_token(TokenKind::LPar, "(")),
                ')' => Some(self.new_token(TokenKind::RPar, ")")),
                '"' => {
                    let mut literal = String::from("");
                    let mut length = 1;

                    while let Some(next_char) = self.peek(length) {
                        if next_char == '"' {
                            break;
                        }

                        literal.push(next_char);
                        length += 1;
                    }

                    self.advance(2);
                    Some(self.new_token(TokenKind::StringLiteral, literal.as_str()))
                }
                c if c.is_numeric() => {
                    let mut number = String::from(c);
                    let mut length = 1;

                    while let Some(next_char) = self.peek(length) {
                        if !next_char.is_numeric() {
                            break;
                        }

                        number.push(next_char);
                        length += 1;
                    }

                    Some(self.new_token(TokenKind::Number, number.as_str()))
                }
                c if c.is_alphabetic() => {
                    let mut identifier = String::from(c);
                    let mut length = 1;

                    while let Some(next_char) = self.peek(length) {
                        if !(next_char.is_alphanumeric() || next_char == '_') {
                            break;
                        }

                        identifier.push(next_char);
                        length += 1;
                    }

                    Some(self.new_token(TokenKind::Identifier, identifier.as_str()))
                }
                _ => {
                    panic!("Unexpected character: \"{char}\"");
                }
            }
        } else {
            None
        }
    }

    fn advance(&mut self, n: usize) -> Option<char> {
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

    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek(&self, n: usize) -> Option<char> {
        if self.position + n < self.input.len() {
            return Some(self.input[self.position + n]);
        }

        None
    }

    fn lookup_ahead(&self, s: &str) -> bool {
        for (i, item) in s.chars().enumerate() {
            if self.input[self.position + i] != item {
                return false;
            }
        }

        !self.input[self.position + s.len()].is_alphanumeric()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if !c.is_whitespace() {
                break;
            }

            self.advance(1);
        }
    }

    fn new_token(&mut self, kind: TokenKind, lexeme: &str) -> Token {
        self.advance(lexeme.len());

        Token {
            kind,
            lexeme: String::from(lexeme),
            line: self.line,
            column: self.start_column,
        }
    }
}

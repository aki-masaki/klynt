use crate::ast::ASTNode;
use crate::ast::Value;
use crate::ast::Variable;
use crate::lexer::Lexer;
use crate::lexer::TokenKind;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self, stop_at_rbrace: bool) -> Vec<ASTNode> {
        let mut nodes = vec![];

        while let Some(token) = self.lexer.next_token() {
            if stop_at_rbrace && token.kind == TokenKind::RBrace {
                return nodes;
            }

            match token.kind {
                TokenKind::Fn => {
                    let start = token.column;
                    let mut name = String::new();
                    let mut parameters: Vec<String> = Vec::new();
                    let mut content: Vec<ASTNode> = Vec::new();

                    if let Some(token) = self.lexer.next_token() {
                        if token.kind == TokenKind::Identifier {
                            name = token.lexeme;
                        } else {
                            panic!(
                                "Expected identifier after fn: {{line: {}, column: {}}}",
                                token.line, token.column
                            );
                        }
                    }

                    if let Some(token) = self.lexer.next_token() {
                        if token.kind == TokenKind::Colon
                            && let Some(token) = self.lexer.next_token()
                            && token.kind == TokenKind::LBrace
                        {
                            while let Some(token) = self.lexer.next_token() {
                                if token.kind == TokenKind::RBrace {
                                    break;
                                }

                                if token.kind == TokenKind::Identifier {
                                    parameters.push(token.lexeme);
                                }
                            }
                        }

                        if token.kind == TokenKind::LBrace {
                            content = self.parse(true);
                        }
                    }

                    nodes.push(ASTNode::FunctionDeclaration {
                        start,
                        name,
                        parameters,
                        content,
                    })
                }
                TokenKind::Let => {
                    let start = token.column;
                    let mut vars: Vec<Variable> = Vec::new();

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                    {
                        let mut current_name: Option<String> = None;

                        while let Some(token) = self.lexer.next_token() {
                            match token.kind {
                                TokenKind::RBrace => {
                                    break;
                                }
                                TokenKind::Identifier => {
                                    current_name = Some(token.lexeme);
                                }
                                TokenKind::Number => {
                                    if let Some(name) = current_name.take() {
                                        let value =
                                            Value::Number(token.lexeme.parse::<i16>().unwrap());

                                        vars.push(Variable(name, value));
                                    } else {
                                        panic!(
                                            "Expected variable name before value at column {}",
                                            token.column
                                        );
                                    }
                                }
                                TokenKind::StringLiteral => {
                                    if let Some(name) = current_name.take() {
                                        let value = Value::Literal(token.lexeme);

                                        vars.push(Variable(name, value));
                                    } else {
                                        panic!(
                                            "Expected variable name before value at column {}",
                                            token.column
                                        );
                                    }
                                }
                                TokenKind::Colon | TokenKind::Comma => {}
                                _ => {
                                    panic!(
                                        "Unexpected token {:?} in variable declaration",
                                        token.kind
                                    );
                                }
                            }
                        }
                    } else {
                        panic!("Expected '{{' after 'let' at column {}", token.column);
                    }

                    nodes.push(ASTNode::VariableDeclaration { start, vars })
                }
                _ => {}
            }
        }

        nodes
    }
}

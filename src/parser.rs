use crate::ast::ASTNode;
use crate::ast::Expression;
use crate::ast::Operator;
use crate::ast::Value;
use crate::ast::Variable;
use crate::lexer::Lexer;
use crate::lexer::Token;
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

                            if let Some(token) = self.lexer.next_token() {
                                if token.kind == TokenKind::LBrace {
                                    content = self.parse(true);
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

                    nodes.push(ASTNode::VariableDeclaration {
                        start,
                        vars: self.parse_vars(token),
                    })
                }
                TokenKind::Set => {
                    let start = token.column;

                    nodes.push(ASTNode::VariableSetting {
                        start,
                        vars: self.parse_vars(token),
                    })
                }
                TokenKind::Const => {
                    let start = token.column;

                    nodes.push(ASTNode::ConstDeclaration {
                        start,
                        vars: self.parse_vars(token),
                    })
                }
                TokenKind::Call => {
                    let mut name = String::new();
                    let mut parameters: Vec<Box<Expression>> = Vec::new();

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                        && let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::Identifier
                    {
                        name = token.lexeme;

                        if let Some(token) = self.lexer.next_token()
                            && token.kind == TokenKind::Colon
                            && let Some(token) = self.lexer.next_token()
                            && token.kind == TokenKind::LBrace
                        {
                            parameters.push(Box::new(self.parse_expression()));

                            while let Some(token) = self.lexer.next_token() {
                                match token.kind {
                                    TokenKind::Comma => {
                                        parameters.push(Box::new(self.parse_expression()));
                                    }
                                    _ => break,
                                }
                            }
                        }
                    }

                    nodes.push(ASTNode::Expression(Expression::FunctionCall {
                        function: name,
                        parameters,
                    }))
                }
                TokenKind::Return => {
                    let start = token.column;
                    let expression = self.parse_expression();

                    nodes.push(ASTNode::ReturnExpression { start, expression })
                }
                _ => {}
            }
        }

        nodes
    }

    fn parse_expression(&mut self) -> Expression {
        let mut expression = Expression::Value(Value::Literal(String::new()));

        if let Some(token) = self.lexer.next_token() {
            match token.kind {
                TokenKind::StringLiteral => {
                    expression = Expression::Value(Value::Literal(token.lexeme));
                }
                TokenKind::Number => {
                    expression =
                        Expression::Value(Value::Number(token.lexeme.parse::<i16>().unwrap()));
                }
                TokenKind::Identifier => expression = Expression::Identifier(token.lexeme),
                TokenKind::Plus | TokenKind::Minus => {
                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                    {}

                    let left = self.parse_expression();

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::Comma
                    {}

                    let right = self.parse_expression();

                    expression = Expression::Binary {
                        left: Box::new(left),
                        op: match token.kind {
                            TokenKind::Plus => Operator::Plus,
                            TokenKind::Minus => Operator::Minus,
                            // Impossible
                            _ => Operator::Plus,
                        },
                        right: Box::new(right),
                    }
                }
                TokenKind::Call => {
                    let mut name = String::new();
                    let mut parameters: Vec<Box<Expression>> = Vec::new();

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                        && let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::Identifier
                    {
                        name = token.lexeme;

                        if let Some(token) = self.lexer.next_token()
                            && token.kind == TokenKind::Colon
                            && let Some(token) = self.lexer.next_token()
                            && token.kind == TokenKind::LBrace
                        {
                            parameters.push(Box::new(self.parse_expression()));

                            while let Some(token) = self.lexer.next_token() {
                                match token.kind {
                                    TokenKind::Comma => {
                                        parameters.push(Box::new(self.parse_expression()));
                                    }
                                    _ => break,
                                }
                            }
                        }
                    }

                    expression = Expression::FunctionCall {
                        function: name,
                        parameters,
                    }
                }
                TokenKind::Semicolon => {
                    panic!("Expected expression at column: {}", token.column);
                }
                _ => {
                    panic!(
                        "Unexpected token: {} at column: {}",
                        token.lexeme, token.column
                    );
                }
            }
        }

        expression
    }

    fn parse_vars(&mut self, token: Token) -> Vec<Variable> {
        let mut vars: Vec<Variable> = Vec::new();

        if let Some(token) = self.lexer.next_token()
            && token.kind == TokenKind::LBrace
        {
            let mut current_name: Option<String> = None;

            while let Some(token) = self.lexer.next_token() {
                match token.kind {
                    TokenKind::RBrace => {
                        while let Some(token) = self.lexer.next_token()
                            && token.kind == TokenKind::RBrace
                        {}

                        break;
                    }
                    TokenKind::Identifier => {
                        current_name = Some(token.lexeme);
                    }
                    TokenKind::Colon => {
                        let expression = self.parse_expression();

                        if let Some(name) = current_name.take() {
                            vars.push(Variable(name, expression));
                        }
                    }
                    TokenKind::Comma => {}
                    _ => {
                        panic!("Unexpected token {:?}", token.lexeme);
                    }
                }
            }
        } else {
            panic!(
                "Expected '{{' after '{}' at column {}",
                token.lexeme, token.column
            );
        }

        vars
    }
}

use crate::ast::ASTNode;
use crate::ast::Expression;
use crate::ast::Operator;
use crate::ast::Value;
use crate::ast::Variable;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenKind;
use std::collections::HashMap;

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
                        } else if token.kind == TokenKind::LBrace {
                            content = self.parse(true);
                        }
                    }

                    nodes.push(ASTNode::FunctionDeclaration {
                        start,
                        name,
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
                TokenKind::Dollar => {
                    let mut name: Option<Expression> = None;
                    let mut parameter: Option<Expression> = None;

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                    {
                        name = Some(self.parse_expression());

                        if let Some(token) = self.lexer.next_token() {
                            if token.kind == TokenKind::Colon {
                                parameter = Some(self.parse_expression());
                            }
                        }
                    }

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::RBrace
                    {}

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::Semicolon
                    {}

                    if let Some(name) = name
                        && let Some(parameter) = parameter
                    {
                        nodes.push(ASTNode::Expression(Expression::FunctionCall {
                            function: Box::new(name),
                            parameter: Box::new(parameter),
                        }))
                    }
                }
                TokenKind::Return => {
                    let start = token.column;
                    let expression = self.parse_expression();

                    nodes.push(ASTNode::ReturnExpression { start, expression })
                }
                TokenKind::When => {
                    let start = token.column;
                    let mut expression = Expression::Identifier(String::new());
                    let mut content: Vec<ASTNode> = Vec::new();

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::Colon
                        && let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                    {
                        expression = self.parse_expression();

                        while let Some(token) = self.lexer.next_token() {
                            if token.kind == TokenKind::RBrace {
                                continue;
                            }

                            if token.kind == TokenKind::LBrace {
                                content = self.parse(true);

                                break;
                            }
                        }
                    }

                    nodes.push(ASTNode::WhenExpression {
                        start,
                        expression,
                        content,
                    })
                }
                TokenKind::OrWhen => {
                    let start = token.column;
                    let mut expression = Expression::Identifier(String::new());
                    let mut content: Vec<ASTNode> = Vec::new();

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::Colon
                        && let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                    {
                        expression = self.parse_expression();

                        while let Some(token) = self.lexer.next_token() {
                            if token.kind == TokenKind::RBrace {
                                continue;
                            }

                            if token.kind == TokenKind::LBrace {
                                content = self.parse(true);

                                break;
                            }
                        }
                    }

                    nodes.push(ASTNode::OrWhenExpression {
                        start,
                        expression,
                        content,
                    })
                }
                TokenKind::Or => {
                    let start = token.column;
                    let mut content: Vec<ASTNode> = Vec::new();

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                    {
                        content = self.parse(true);
                    }

                    nodes.push(ASTNode::OrExpression { start, content })
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
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Times
                | TokenKind::Divided
                | TokenKind::Gt
                | TokenKind::Lt
                | TokenKind::Equal => {
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
                            TokenKind::Times => Operator::Times,
                            TokenKind::Divided => Operator::Divided,
                            TokenKind::Gt => Operator::Gt,
                            TokenKind::Lt => Operator::Lt,
                            TokenKind::Equal => Operator::Equal,
                            // Impossible
                            _ => Operator::Plus,
                        },
                        right: Box::new(right),
                    }
                }
                TokenKind::Dollar => {
                    let mut name: Option<Expression> = None;
                    let mut parameter: Option<Expression> = None;

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                    {
                        name = Some(self.parse_expression());

                        if let Some(token) = self.lexer.next_token() {
                            if token.kind == TokenKind::Colon {
                                parameter = Some(self.parse_expression());
                            }
                        }
                    }

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::RBrace
                    {}

                    if let Some(name) = name
                        && let Some(parameter) = parameter
                    {
                        expression = Expression::FunctionCall {
                            function: Box::new(name),
                            parameter: Box::new(parameter),
                        }
                    }
                }
                TokenKind::LBracket => {
                    let mut items: Vec<Box<Expression>> = vec![Box::new(self.parse_expression())];

                    while let Some(token) = self.lexer.next_token() {
                        if token.kind == TokenKind::Comma {
                            items.push(Box::new(self.parse_expression()));
                        } else if token.kind == TokenKind::RBracket {
                            break;
                        }
                    }

                    expression = Expression::ArrayExpression(items);
                }
                TokenKind::LPar => {
                    let mut hash_map = HashMap::new();

                    let mut current_name: Option<String> = None;

                    while let Some(token) = self.lexer.next_token() {
                        if token.kind == TokenKind::Identifier {
                            current_name = Some(token.lexeme);
                        } else if token.kind == TokenKind::Colon {
                            let expr = self.parse_expression();

                            if let Some(name) = current_name.take() {
                                hash_map.insert(name, expr);
                            }
                        } else if token.kind == TokenKind::RPar {
                            break;
                        }
                    }

                    expression = Expression::ObjectExpression(hash_map);
                }
                TokenKind::Dot => {
                    let mut object: Option<String> = None;
                    let mut property: Option<Expression> = None;

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                    {
                        if let Some(identifier) = self.lexer.next_token() {
                            object = Some(identifier.lexeme);
                        }

                        if let Some(token) = self.lexer.next_token()
                            && token.kind == TokenKind::Comma
                        {
                            property = Some(self.parse_expression());

                            if let Some(token) = self.lexer.next_token()
                                && token.kind == TokenKind::RBrace
                            {}
                        }
                    }

                    if let Some(object) = object
                        && let Some(property) = property
                    {
                        expression = Expression::PropertyAccess {
                            object,
                            property: Box::new(property),
                        }
                    }
                }
                TokenKind::At => {
                    let mut array: Option<Expression> = None;
                    let mut index: Option<Expression> = None;

                    if let Some(token) = self.lexer.next_token()
                        && token.kind == TokenKind::LBrace
                    {
                        array = Some(self.parse_expression());

                        if let Some(token) = self.lexer.next_token()
                            && token.kind == TokenKind::Comma
                        {
                            index = Some(self.parse_expression());
                        }
                    }

                    if let Some(array) = array
                        && let Some(index) = index
                    {
                        expression = Expression::ArrayIndex {
                            array: Box::new(array),
                            index: Box::new(index),
                        }
                    }
                }
                TokenKind::Comma => {
                    return self.parse_expression();
                }
                TokenKind::Semicolon => {
                    panic!("Expected expression at column: {}", token.column);
                }
                _ => {
                    panic!(
                        "Unexpected token: {} at column: {} line: {}",
                        token.lexeme, token.column, token.line
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
                        panic!(
                            "Unexpected token {:?} at {}:{}",
                            token.lexeme, token.line, token.column
                        );
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

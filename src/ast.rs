#[derive(Debug, PartialEq)]
pub enum Value {
    Literal(String),
    Number(i16),
}

#[derive(Debug, PartialEq)]
pub struct Variable(pub String, pub Expression);

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(Value),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>
    },
    FunctionCall {
        function: String,
        parameters: Vec<Box<Expression>>
    }
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    FunctionDeclaration {
        start: usize,
        name: String,
        parameters: Vec<String>,
        content: Vec<ASTNode>,
    },
    VariableDeclaration {
        start: usize,
        vars: Vec<Variable>,
    },
    ConstDeclaration {
        start: usize,
        vars: Vec<Variable>,
    },
    VariableSetting {
        start: usize,
        vars: Vec<Variable>,
    },
    ReturnExpression {
        start: usize,
        expression: Expression,
    },
    Expression(Expression)
}

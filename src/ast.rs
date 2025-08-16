#[derive(Debug, PartialEq)]
pub enum Value {
    Literal(String),
    Number(i16)
}

#[derive(Debug, PartialEq)]
pub struct Variable(pub String, pub Value);

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    FunctionDeclaration {
        start: usize,
        name: String,
        parameters: Vec<String>,
        content: Vec<ASTNode>
    },
    VariableDeclaration {
        start: usize,
        vars: Vec<Variable>
    }
}

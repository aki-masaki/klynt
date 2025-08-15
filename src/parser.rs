use crate::ast::ASTNode;
use crate::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn next_node() -> Option<ASTNode> {
        None
    }
}

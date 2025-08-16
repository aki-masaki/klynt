use crate::transpiler::Transpiler;
use crate::parser::Parser;
use crate::lexer::Lexer;

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod transpiler;

pub fn transpile(input: String) -> String {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let transpiler = Transpiler::new(parser.parse(false));

    transpiler.transpile()
}

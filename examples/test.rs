use klynt::lexer::Lexer;
use klynt::parser::Parser;

fn main() {
    let source = String::from(
        r#"fn main {
    let {a: 25, b: 30, str: "Hello"};
    let {c: +{a, b}};

    set {a: 40};
}

fn calc:{a, b} {
    const {offset: 10};

    ret +{a, +{b, offset}};
}
"#,
    );

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    println!("{:?}", parser.parse(false));
}

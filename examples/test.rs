use klynt::lexer::Lexer;

fn main() {
    let source = String::from(r#"fn main {
    let {a: 25, b: 30, str: "Hello"};

    ret calc:{5, calc:{a, b}};
}

fn calc:{a, b} {
    ret +{a, -{b, 5}};
}"#);

    let mut lexer = Lexer::new(source);

    while let Some(token) = lexer.next_token() {
        println!("{token:?}");
    }
}


use klynt::lexer::Lexer;

fn main() {
    let source = String::from(r#"fn main {
    let {a: 25, b: 30, str: "Hello"};

    ret a + b;
}"#);

    let mut lexer = Lexer::new(source);

    while let Some(token) = lexer.next_token() {
        println!("{token:?}");
    }
}


use klynt::lexer::Lexer;

fn main() {
    let source = String::from(r#"fn main {
        return 42;
    }"#);

    let mut lexer = Lexer::new(source);

    while let Some(token) = lexer.next_token() {
        println!("{token:?}");
    }
}


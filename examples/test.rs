use klynt::lexer::Lexer;
use klynt::parser::Parser;
use klynt::transpiler::Transpiler;

fn main() {
    let source = String::from(
        r#"fn main {
    let {a: 25, b: 30, str: "Hello"};
    let {c: +{a, b}};

    let {arr: [1, 2, 3, "hi", a, +{b, 10}, call{calc:{5, 5}}]};
    let {object: (data: "hello", count: 0, inner: (text: "Hello World"))};

    let {count: .{object, count}, text: .{object, .{inner, text}}}

    set {a: 40};

    call{calc:{20, call{calc:{10, 20}}}};

    when:{>{+{a, b}, 10}} {
        set {a: 20};
    } orwhen:{<{-{a, b}, 20}} {
        set {b: 30};
    } or {
        set {c: 50};
    }
}

fn calc:{a, b} {
    const {offset: 10};

    ret +{a, -{b, offset}};
}
"#,
    );

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let transpiler = Transpiler::new(parser.parse(false));

    println!("{}", transpiler.transpile());
}

use klynt::lexer::Lexer;
use klynt::parser::Parser;
use klynt::transpiler::Transpiler;

fn main() {
    let source = String::from(
        r#"fn main {
    let {a: 25, b: 30, str: "Hello"};
    let {c: +{a, b}};

    let {arr: [1, 2, 3, "hi", a, +{b, 10}, ${calc:(a: 5, b: 5)}]};
    let {list: [1, 2, 3]};
    let {object: (data: "hello", count: 0, inner: (text: "Hello World", list: [@{list, 0}, @{list, 1}, @{list, 2}]))};

    let {count: .{object, count}, text: .{object, .{inner, text}}};

    let {num: (a: ${.{a, toString}:()}, b: ${.{b, toString}:()}, c: @{arr, 2}, d: @{.{object, .{inner, list}}, -{.{list, length}, 1}})};

    ${calc:(a: a, b: b)};

    ${.{a, toString}:()};

    when:{>{+{a, b}, 10}} {
        set {a: 20};
    } orwhen:{<{-{a, b}, 20}} {
        set {b: 30};
    } or {
        set {c: 50};
    }
}

fn calc {
    const {offset: 10};

    ret +{.{param, a}, -{.{param, b}, offset}};
}
"#,
    );

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let transpiler = Transpiler::new(parser.parse(false));

    println!("{}", transpiler.transpile());
}

extern crate lexer;
extern crate parser;

fn main() {
    let src = r#"
        (if (< (+ 1 2 3) (* 1 2 3))
            (mod 3 2)
            "hello world")"#;

    if let Ok((s, _)) = lexer::lexer::s(src) {
        if let Ok(token) = parser::parser::parse(&lexer::s::Edge::S(s)) {
            println!("{:?}", token);
        }
    }
}

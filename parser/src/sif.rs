use lexer::edge::Edge;
use lexer::node::Node;
use lexer::token::Token;
use std::slice::*;

pub fn sif(node: Node) -> Edge {
    match *node.head {
        Edge::Token(t) => match t {
            Token::Symbol(symbol) => match symbol.as_str() {
                "if" => {
                    let mut edges = node.tail.into_iter();
                    let head = edges.next().unwrap();
                    let result = match head {
                        Edge::Node(node) => condition(*node.head, node.tail.iter()),
                        Edge::Token(Token::Boolean(b)) => b,
                        n => panic!("Expected boolean, this is {:?}", n),
                    };
                    if result {
                        edges.next().unwrap()
                    } else {
                        let _ = edges.next();
                        edges.next().unwrap()
                    }
                }
                n => panic!("expected if, this is {}", n),
            },
            n => panic!("expected symbol token, this is {:?}", n),
        },
        n => panic!("expected token, this is {:?}", n),
    }
}

pub fn condition(head: Edge, tail: Iter<Edge>) -> bool {
    match head {
        Edge::Token(t) => match t {
            Token::Symbol(symbol) => match symbol.as_str() {
                "eq" | "=" => {
                    let mut e = tail.into_iter();
                    match (e.next(), e.next()) {
                        (
                            Some(Edge::Token(Token::Symbol(n))),
                            Some(Edge::Token(Token::Symbol(m))),
                        )
                        | (
                            Some(Edge::Token(Token::String(n))),
                            Some(Edge::Token(Token::String(m))),
                        ) => n == m,
                        (
                            Some(Edge::Token(Token::Boolean(n))),
                            Some(Edge::Token(Token::Boolean(m))),
                        ) => n == m,
                        (
                            Some(Edge::Token(Token::Number(n))),
                            Some(Edge::Token(Token::Number(m))),
                        ) => n == m,
                        (a, b) => panic!("expected value, this is {:?}, {:?}", a, b),
                    }
                }
                "<=" => {
                    let mut e = tail.into_iter();
                    match (e.next(), e.next()) {
                        (
                            Some(Edge::Token(Token::Number(n))),
                            Some(Edge::Token(Token::Number(m))),
                        ) => n <= m,
                        (a, b) => panic!("expected numbers, this is {:?}, {:?}", a, b),
                    }
                }
                ">=" => {
                    let mut e = tail.into_iter();
                    match (e.next(), e.next()) {
                        (
                            Some(Edge::Token(Token::Number(n))),
                            Some(Edge::Token(Token::Number(m))),
                        ) => n >= m,
                        (a, b) => panic!("expected numbers, this is {:?}, {:?}", a, b),
                    }
                }
                ">" => {
                    let mut e = tail.into_iter();
                    match (e.next(), e.next()) {
                        (
                            Some(Edge::Token(Token::Number(n))),
                            Some(Edge::Token(Token::Number(m))),
                        ) => n > m,
                        (a, b) => panic!("expected numbers, this is {:?}, {:?}", a, b),
                    }
                }
                "<" => {
                    let mut e = tail.into_iter();
                    match (e.next(), e.next()) {
                        (
                            Some(Edge::Token(Token::Number(n))),
                            Some(Edge::Token(Token::Number(m))),
                        ) => n < m,
                        (a, b) => panic!("expected numbers, this is {:?}, {:?}", a, b),
                    }
                }
                n => panic!("expected bool, this is {}", n),
            },
            n => panic!("expected symbol token, this is {:?}", n),
        },
        n => panic!("expected token, this is {:?}", n),
    }
}

#[test]
fn ts_sif() {
    use lexer::lexer::parse;
    let result = parse(
        r#"
        (if true 1 2)
        (if (eq 1 1) 3 4)
        (if (eq 1 2) 3 4)
        "#,
    )
    .map(|d| d.0)
    .unwrap();
    let mut a = result.into_iter().map(|d| sif(d));
    assert_eq!(a.next(), Some(Edge::Token(Token::Number(1))));
    assert_eq!(a.next(), Some(Edge::Token(Token::Number(3))));
    assert_eq!(a.next(), Some(Edge::Token(Token::Number(4))));
}

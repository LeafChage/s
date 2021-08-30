extern crate parser;

use parser::edge::Edge;
use parser::node::Node;
use parser::parser::parse;
use parser::token::Token;

fn main() {
    println!("hello world");
}

fn calicurate_edge(node: Node) -> isize {
    let number = |edge: Edge| -> isize {
        match edge {
            Edge::Node(n) => calicurate_edge(n),
            Edge::Token(Token::Number(n)) => n,
            Edge::Token(_) => panic!("Is this number?"),
        }
    };

    match *node.head {
        Edge::Token(t) => match t {
            Token::Symbol(symbol) => match symbol.as_str() {
                "+" => {
                    let mut i = node.tail.into_iter();
                    let head = match i.next() {
                        Some(n) => number(n),
                        None => 0,
                    };
                    return i.fold(head, |sum, val| sum + number(val));
                }
                "-" => {
                    let mut i = node.tail.into_iter();
                    let head = match i.next() {
                        Some(n) => number(n),
                        None => 0,
                    };
                    return i.fold(head, |sum, val| sum - number(val));
                }
                "*" => {
                    let mut i = node.tail.into_iter();
                    let head = match i.next() {
                        Some(n) => number(n),
                        None => 0,
                    };
                    return i.fold(head, |sum, val| sum * number(val));
                }
                "/" => {
                    let mut i = node.tail.into_iter();
                    let head = match i.next() {
                        Some(n) => number(n),
                        None => 0,
                    };
                    return i.fold(head, |sum, val| sum / number(val));
                }
                _ => panic!(""),
            },
            _ => panic!(""),
        },
        _ => panic!(""),
    }
}

#[test]
fn ts_calicurate() {
    let result = parse(
        r#"
        (- 34 -89)
        (+ 1 2 3)
        (* 22 3)
        (+ 1 2 3 (- 34 -89))
        "#,
    )
    .map(|d| d.0)
    .unwrap();
    let mut a = result.into_iter().map(|d| calicurate_edge(d));
    assert_eq!(a.next(), Some(123));
    assert_eq!(a.next(), Some(6));
    assert_eq!(a.next(), Some(66));
    assert_eq!(a.next(), Some(129));
}

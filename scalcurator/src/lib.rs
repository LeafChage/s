extern crate sparser;

use sparser::edge::Edge;
use sparser::node::Node;
use sparser::token::Token;

fn number_from_edge(edge: Edge) -> isize {
    match edge {
        Edge::Node(n) => calicurate_edge(n),
        Edge::Token(Token::Number(n)) => n,
        Edge::Token(n) => panic!("Expected is number, this is {:?}", n),
    }
}

fn edges_fold<F>(edges: Vec<Edge>, f: F) -> isize
where
    F: FnMut(isize, Edge) -> isize,
{
    let mut i = edges.into_iter();
    let head = match i.next() {
        Some(n) => number_from_edge(n),
        None => 0,
    };
    i.fold(head, f)
}

pub fn calicurate_edge(node: Node) -> isize {
    match *node.head {
        Edge::Token(t) => match t {
            Token::Symbol(symbol) => match symbol.as_str() {
                "+" => edges_fold(node.tail, |sum, val| sum + number_from_edge(val)),
                "-" => edges_fold(node.tail, |sum, val| sum - number_from_edge(val)),
                "*" => edges_fold(node.tail, |sum, val| sum * number_from_edge(val)),
                "/" => edges_fold(node.tail, |sum, val| sum / number_from_edge(val)),
                "mod" => edges_fold(node.tail, |sum, val| sum % number_from_edge(val)),
                n => panic!("expected + | - | * | / | mod |, this is {}", n),
            },
            n => panic!("expected symbol token, this is {:?}", n),
        },
        n => panic!("expected token, this is {:?}", n),
    }
}

#[test]
fn ts_calicurate() {
    use sparser::parser::parse;
    let result = parse(
        r#"
        (- 34 -89)
        (+ 1 2 3)
        (* 22 3)
        (+ 1 2 3 (- 34 -89))
        (mod 10 7)
        "#,
    )
    .map(|d| d.0)
    .unwrap();
    let mut a = result.into_iter().map(|d| calicurate_edge(d));
    assert_eq!(a.next(), Some(123));
    assert_eq!(a.next(), Some(6));
    assert_eq!(a.next(), Some(66));
    assert_eq!(a.next(), Some(129));
    assert_eq!(a.next(), Some(3));
}

use super::error::Result;
use super::search_token::SearchToken;
use lexer::edge::Edge;
use lexer::node::Node;
use lexer::token::Token;

pub fn sif(node: Node) -> Result<Edge> {
    let _ = node.symbol_with("if")?;

    let mut edges = node.tail.into_iter();
    let head = edges.next().unwrap();
    let result = match head {
        Edge::Node(node) => condition(*node.head, node.tail)?,
        Edge::Token(Token::Boolean(b)) => b,
        n => panic!("Expected boolean, this is {:?}", n),
    };

    Ok(if result {
        edges.next().unwrap()
    } else {
        let _ = edges.next();
        edges.next().unwrap()
    })
}

fn tail_matches<F>(edges: &[Edge], then: F) -> bool
where
    F: Fn(isize, isize) -> bool,
{
    match edges {
        [Edge::Token(Token::Number(n)), Edge::Token(Token::Number(m))] => then(*n, *m),
        [a, b] => panic!("expected type is number, number, this is {:?}, {:?}", a, b),
        [] | [_, _, ..] => panic!("expected type is number, number, unexpected arg counts"),
        _ => panic!("Unexpected Error"),
    }
}

/// not supported string == string
pub fn condition(head: Edge, tail: Vec<Edge>) -> Result<bool> {
    let symbol = head.symbol()?;
    match symbol {
        "eq" | "=" => Ok(tail_matches(&tail[..], |n, m| n == m)),
        ">=" => Ok(tail_matches(&tail[..], |n, m| n >= m)),
        "<=" => Ok(tail_matches(&tail[..], |n, m| n <= m)),
        ">" => Ok(tail_matches(&tail[..], |n, m| n > m)),
        "<" => Ok(tail_matches(&tail[..], |n, m| n < m)),
        n => panic!("expected bool, this is {}", n),
    }
}

#[test]
fn ts_sif() {
    use lexer::lexer::parse;
    let result = parse(
        r#"
        (if true 1 2)
        (if (eq 1 1) 3 4)
        (if (= 1 2) 3 4)
        "#,
    )
    .map(|d| d.0)
    .unwrap();

    let mut a = result.into_iter().map(|d| sif(d));
    assert_eq!(a.next().unwrap().unwrap(), Edge::Token(Token::Number(1)));
    assert_eq!(a.next().unwrap().unwrap(), Edge::Token(Token::Number(3)));
    assert_eq!(a.next().unwrap().unwrap(), Edge::Token(Token::Number(4)));
}

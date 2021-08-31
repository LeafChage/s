use super::edge::Edge;

#[derive(Debug)]
pub struct Node {
    pub head: Box<Edge>,
    pub tail: Vec<Edge>,
}

impl Node {
    pub fn new(token: Edge, tail: Vec<Edge>) -> Self {
        Node {
            head: Box::new(token),
            tail: tail,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}

#[test]
fn ts_eq() {
    use super::token::Token;
    assert_eq!(
        Node::new(Edge::Token(Token::symbol("symbol")), vec![]),
        Node::new(Edge::Token(Token::symbol("symbol")), vec![]),
    );
}

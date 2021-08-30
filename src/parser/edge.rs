use super::node::Node;
use super::token::Token;

#[derive(Debug)]
pub enum Edge {
    Token(Token),
    Node(Node),
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Edge::Token(ref a), Edge::Token(ref b)) => a == b,
            (Edge::Node(ref a), Edge::Node(ref b)) => a == b,
            _ => false,
        }
    }
}

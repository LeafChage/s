use super::token::Token;
use std::fmt;
use std::iter::Iterator;

pub enum Edge {
    Token(Token),
    S(S),
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Edge::Token(ref a), Edge::Token(ref b)) => a == b,
            (Edge::S(ref a), Edge::S(ref b)) => a == b,
            _ => false,
        }
    }
}
impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Edge::Token(t) => write!(f, "{:?}", t),
            Edge::S(s) => write!(f, "({:?})", s),
        }
    }
}

pub enum S {
    Cons(Box<Edge>, Box<S>),
    Nil,
}

impl S {
    pub fn unit(head: Edge) -> Self {
        S::Cons(Box::new(head), Box::new(S::Nil))
    }

    pub fn cons(head: Edge, tail: S) -> Self {
        S::Cons(Box::new(head), Box::new(tail))
    }

    pub fn from_vector(v: Vec<Edge>) -> Self {
        v.into_iter().rev().fold(S::Nil, |l, head| S::cons(head, l))
    }

    pub fn fold<Acc, F>(&self, init: Acc, f: F) -> Acc
    where
        F: Fn(Acc, &Edge) -> Acc,
    {
        match self {
            S::Cons(head, tail) => match tail.head() {
                &Edge::S(S::Nil) => f(init, head),
                _ => tail.fold(f(init, head), f),
            },
            S::Nil => init,
        }
    }

    // pub fn to_vector(&self) -> Vec<Edge> {
    //     to_vector()
    //     v.into_iter().fold(S::Nil, |l, head| S::cons(head, l))
    // }

    pub fn head(&self) -> &Edge {
        match &*self {
            S::Cons(head, _) => &(*head),
            S::Nil => &Edge::Token(Token::Nil),
        }
    }

    pub fn tail(&self) -> &S {
        match &*self {
            S::Cons(_, tail) => &(*tail),
            S::Nil => &S::Nil,
        }
    }
}

#[test]
fn ts_add() {
    use super::token::Token;
    assert_eq!(
        S::from_vector(vec![Edge::Token(Token::Number(1))]),
        S::unit(Edge::Token(Token::Number(1))),
    );

    assert_eq!(
        S::from_vector(vec![
            Edge::Token(Token::Number(1)),
            Edge::Token(Token::Number(2))
        ]),
        S::cons(
            Edge::Token(Token::Number(1)),
            S::unit(Edge::Token(Token::Number(2)))
        ),
    );
}

impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (S::Nil, S::Nil) => true,
            (S::Cons(ref a, ref a2), S::Cons(ref b, ref b2)) => a == b && a2 == b2,
            _ => false,
        }
    }
}

impl fmt::Debug for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Cons(a, b) => write!(f, "({:?} {:?})", a, b),
            S::Nil => write!(f, "nil"),
        }
    }
}

#[test]
fn ts_eq() {
    use super::token::Token;
    assert_eq!(
        S::unit(Edge::Token(Token::symbol("symbol"))),
        S::unit(Edge::Token(Token::symbol("symbol"))),
    );
}

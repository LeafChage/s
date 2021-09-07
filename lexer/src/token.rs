use std::fmt;

pub enum Token {
    Symbol(String),
    String(String),
    Number(isize),
    Boolean(bool),
    Nil,
}

impl Clone for Token {
    fn clone(&self) -> Token {
        match self {
            &Token::Symbol(ref n) => Token::symbol(n),
            &Token::String(ref n) => Token::string(n),
            &Token::Number(ref n) => Token::Number(*n),
            &Token::Boolean(ref n) => Token::Boolean(*n),
            &Token::Nil => Token::Nil,
        }
    }
}

impl Token {
    pub fn symbol(s: impl Into<String>) -> Self {
        Token::Symbol(s.into())
    }
    pub fn string(s: impl Into<String>) -> Self {
        Token::String(s.into())
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Symbol(ref a), Token::Symbol(ref b)) => a == b,
            (Token::String(ref a), Token::String(ref b)) => a == b,
            (Token::Number(ref a), Token::Number(ref b)) => a == b,
            (Token::Boolean(ref a), Token::Boolean(ref b)) => a == b,
            _ => false,
        }
    }
}

#[test]
fn ts_eq() {
    assert_eq!(Token::symbol("a"), Token::symbol("a"));
    assert_eq!(Token::string("a"), Token::string("a"));
    assert_eq!(Token::Number(1), Token::Number(1));
    assert_eq!(Token::Boolean(true), Token::Boolean(true));
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Symbol(s) => write!(f, ":{}", s),
            Token::String(s) => write!(f, "\"{:?}\"", s),
            Token::Number(n) => write!(f, "{:?}", n),
            Token::Boolean(b) => write!(f, "{:?}", b),
            Token::Nil => write!(f, "nil"),
        }
    }
}

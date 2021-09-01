#[derive(Debug)]
pub enum Token {
    Symbol(String),
    String(String),
    Number(isize),
    Boolean(bool),
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

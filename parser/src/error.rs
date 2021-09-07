use std::fmt;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new(msg: String) -> Self {
        return ParseError { msg };
    }

    pub fn expected_token(d: impl fmt::Debug) -> Self {
        return ParseError {
            msg: format!("expected token, this is {:?}", d),
        };
    }

    pub fn expected_boolean_token(d: impl fmt::Debug) -> Self {
        return ParseError {
            msg: format!("expected Token::Boolean, this is {:?}", d),
        };
    }

    pub fn expected_list(d: impl fmt::Debug) -> Self {
        return ParseError {
            msg: format!("expected edge, this is {:?}", d),
        };
    }

    pub fn unexpected_symbol(symbol: &str, d: impl fmt::Debug) -> Self {
        return ParseError {
            msg: format!("expected symbol is {}, this is {:?}", symbol, d),
        };
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

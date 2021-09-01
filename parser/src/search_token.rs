use super::error::{ParseError, Result};
use lexer::edge::Edge;
use lexer::node::Node;
use lexer::token::Token;

pub trait SearchToken {
    fn token(&self) -> Result<&Token>;
    fn symbol(&self) -> Result<&str>;
    fn symbol_with(&self, expect: &str) -> Result<Token>;
}

impl SearchToken for Node {
    fn token(&self) -> Result<&Token> {
        match &*self.head {
            Edge::Token(t) => Ok(t),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn symbol(&self) -> Result<&str> {
        let token = self.token()?;
        match token {
            Token::Symbol(s) => Ok(s),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn symbol_with(&self, expect: &str) -> Result<Token> {
        let token = self.token()?;
        match token {
            Token::Symbol(s) => {
                if s == expect {
                    Ok(Token::symbol(s))
                } else {
                    Err(ParseError::unexpected_symbol(expect, s))
                }
            }
            n => Err(ParseError::expected_token(n)),
        }
    }
}

impl SearchToken for Edge {
    fn token(&self) -> Result<&Token> {
        match self {
            Edge::Token(t) => Ok(t),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn symbol(&self) -> Result<&str> {
        let token = self.token()?;
        match token {
            Token::Symbol(s) => Ok(s),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn symbol_with(&self, expect: &str) -> Result<Token> {
        let token = self.token()?;
        match token {
            Token::Symbol(s) => {
                if s == expect {
                    Ok(Token::symbol(s))
                } else {
                    Err(ParseError::unexpected_symbol(expect, s))
                }
            }
            n => Err(ParseError::expected_token(n)),
        }
    }
}

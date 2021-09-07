use super::error::{ParseError, Result};
use lexer::s::{Edge, S};
use lexer::token::Token;

pub trait SearchToken {
    fn head_token(&self) -> Result<&Token>;
    fn symbol(&self) -> Result<String>;
    fn number(&self) -> Result<isize>;
    fn boolean(&self) -> Result<bool>;
    fn string(&self) -> Result<String>;
    fn symbol_with(&self, expect: &str) -> Result<Token>;
}

impl SearchToken for S {
    fn head_token(&self) -> Result<&Token> {
        match self {
            S::Cons(edge, _) => edge.head_token(),
            n => Err(ParseError::expected_list(n)),
        }
    }

    fn symbol(&self) -> Result<String> {
        let token = self.head_token()?;
        match token {
            Token::Symbol(s) => Ok(String::from(s)),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn number(&self) -> Result<isize> {
        let token = self.head_token()?;
        match token {
            Token::Number(s) => Ok(s.clone()),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn boolean(&self) -> Result<bool> {
        let token = self.head_token()?;
        match token {
            Token::Boolean(s) => Ok(s.clone()),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn string(&self) -> Result<String> {
        let token = self.head_token()?;
        match token {
            Token::String(s) => Ok(String::from(s)),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn symbol_with(&self, expect: &str) -> Result<Token> {
        let token = self.head_token()?;
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
    fn head_token(&self) -> Result<&Token> {
        match self {
            Edge::Token(ref t) => Ok(t),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn symbol(&self) -> Result<String> {
        let token = self.head_token()?;
        match token {
            Token::Symbol(s) => Ok(String::from(s)),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn number(&self) -> Result<isize> {
        let token = self.head_token()?;
        match token {
            Token::Number(s) => Ok(s.clone()),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn boolean(&self) -> Result<bool> {
        let token = self.head_token()?;
        match token {
            Token::Boolean(s) => Ok(s.clone()),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn string(&self) -> Result<String> {
        let token = self.head_token()?;
        match token {
            Token::String(s) => Ok(String::from(s)),
            n => Err(ParseError::expected_token(n)),
        }
    }

    fn symbol_with(&self, expect: &str) -> Result<Token> {
        let token = self.head_token()?;
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

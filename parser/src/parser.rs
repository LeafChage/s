use super::error::{ParseError, Result};
use lexer::s::Edge;
use lexer::token::Token;

pub fn parse(e: &Edge) -> Result<Token> {
    match e {
        &Edge::S(ref s) => {
            let head = s.head();
            match head {
                &Edge::S(_) => parse(head),
                &Edge::Token(Token::Symbol(ref symbol)) => match symbol.as_str() {
                    "if" => super::sif::evaluate(s),
                    // "for" => super::sif::evaluate(s),
                    "+" | "-" | "*" | "/" | "%" => super::math::evaluate(s),
                    "=" | "eq" | ">" | "<" | ">=" | "<=" => super::boolean::evaluate(s),
                    n => Err(ParseError::unexpected_symbol("", n)),
                },
                &Edge::Token(ref n) => Ok(n.clone()),
            }
        }
        &Edge::Token(ref n) => Ok(n.clone()),
    }
}

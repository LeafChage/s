use super::error::{ParseError, Result};
use super::parser::parse;
use super::search_token::SearchToken;
use lexer::s::{Edge, S};
use lexer::token::Token;

pub fn evaluate(s: &S) -> Result<Token> {
    let symbol = s.symbol()?;

    let first = s.tail().head();
    let first = match first {
        Edge::S(_) => parse(first).unwrap_or(Token::Nil),
        Edge::Token(t) => t.clone(),
    };

    let second = s.tail().tail().head();
    let second = match second {
        Edge::S(_) => parse(second).unwrap_or(Token::Nil),
        Edge::Token(t) => t.clone(),
    };

    let result = match symbol.as_str() {
        "eq" | "=" => match (&first, &second) {
            (Token::String(a), Token::String(b)) => Ok(a == b),
            (Token::Symbol(a), Token::Symbol(b)) => Ok(a == b),
            (Token::Number(a), Token::Number(b)) => Ok(a == b),
            (Token::Boolean(a), Token::Boolean(b)) => Ok(a == b),
            _ => panic!(),
        },
        ">=" => match (&first, &second) {
            (Token::Number(a), Token::Number(b)) => Ok(a >= b),
            _ => panic!(),
        },
        "<=" => match (&first, &second) {
            (Token::Number(a), Token::Number(b)) => Ok(a <= b),
            _ => panic!(),
        },
        ">" => match (&first, &second) {
            (Token::Number(a), Token::Number(b)) => Ok(a > b),
            _ => panic!(),
        },
        "<" => match (&first, &second) {
            (Token::Number(a), Token::Number(b)) => Ok(a < b),
            _ => panic!(),
        },
        n => Err(ParseError::unexpected_symbol(
            "eq | = | >= | <= | <  | > ",
            n,
        )),
    }
    .map(|b| Token::Boolean(b));

    println!(
        "[{:15}] {:?} {} {:?} => {:?}",
        "Parser BOOLEAN",
        &first,
        symbol.as_str(),
        &second,
        result
    );
    result
}

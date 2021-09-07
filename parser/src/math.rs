use super::error::{ParseError, Result};
use super::search_token::SearchToken;
use lexer::s::{Edge, S};
use lexer::token::Token;

fn number_from(edge: &Edge) -> isize {
    match edge {
        &Edge::Token(Token::Number(n)) => n,
        &Edge::Token(ref n) => panic!("Expected is number, this is {:?}", n),
        &Edge::S(S::Nil) => panic!("Expected is number, this is nil"),
        &Edge::S(_) => match super::parser::parse(edge) {
            Ok(Token::Number(n)) => n,
            _ => panic!(),
        },
    }
}

pub fn evaluate(s: &S) -> Result<Token> {
    let symbol = s.symbol()?;
    let tail = s.tail();

    let result = match symbol.as_str() {
        "+" => Ok(tail
            .tail()
            .fold(number_from(tail.head()), |sum, v| sum + number_from(v))),
        "-" => Ok(tail
            .tail()
            .fold(number_from(tail.head()), |sum, v| sum - number_from(v))),
        "*" => Ok(tail
            .tail()
            .fold(number_from(tail.head()), |sum, v| sum * number_from(v))),
        "/" => Ok(tail
            .tail()
            .fold(number_from(tail.head()), |sum, v| sum / number_from(v))),
        "%" => Ok(tail
            .tail()
            .fold(number_from(tail.head()), |sum, v| sum % number_from(v))),
        n => Err(ParseError::new(format!(
            "expected + | - | * | / | mod |, this is {}",
            n
        ))),
    }
    .map(|v| Token::Number(v));

    println!(
        "[{:15}] ({} {:?}) => {:?}",
        "Parser Math",
        symbol,
        tail.tail(),
        result
    );

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use lexer::lexer::s;

    #[test]
    fn ts_calicurate() {
        let s = s(r#"(- 34 -89)"#).unwrap();
        assert_eq!(evaluate(&(s.0)), Ok(Token::Number(123)));
    }
    #[test]
    fn ts_calicurate2() {
        let s = s(r#"(+ 1 2 3)"#).unwrap();
        assert_eq!(evaluate(&(s.0)), Ok(Token::Number(6)));
    }
    #[test]
    fn ts_calicurate3() {
        let s = s(r#"(* 22 3)"#).unwrap();
        assert_eq!(evaluate(&(s.0)), Ok(Token::Number(66)));
    }
    #[test]
    fn ts_calicurate4() {
        let s = s(r#"(+ 1 2 3 (- 34 -89))"#).unwrap();
        assert_eq!(evaluate(&(s.0)), Ok(Token::Number(129)));
    }
    #[test]
    fn ts_calicurate5() {
        let s = s(r#"(% 10 7)"#).unwrap();
        assert_eq!(evaluate(&(s.0)), Ok(Token::Number(3)));
    }
}

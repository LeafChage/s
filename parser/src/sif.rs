use super::error::{ParseError, Result};
use super::search_token::SearchToken;
use lexer::s::{Edge, S};
use lexer::token::Token;

pub fn evaluate(s: &S) -> Result<Token> {
    let _ = s.symbol_with("if")?;
    let b = s.tail().head();
    let first = s.tail().tail().head();
    let second = s.tail().tail().tail().head();
    sif(b, first, second)
}

fn sif<'a>(condition: &'a Edge, if_true: &'a Edge, if_false: &'a Edge) -> Result<Token> {
    let b = super::parser::parse(condition)?;
    match b {
        Token::Boolean(b) => {
            let result = if b { if_true } else { if_false };
            println!(
                "[{:15}] if({:?}) {:?} else {:?}  => {:?}",
                "Parser IF", b, if_true, if_false, result,
            );

            super::parser::parse(result)
        }
        n => Err(ParseError::expected_boolean_token(n)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lexer::lexer::s;

    #[test]
    fn ts_sif() {
        let s = s(r#"(if (= 1 1) 1 2)"#).unwrap();
        assert_eq!(evaluate(&(s.0)), Ok(Token::Number(1)));
    }

    #[test]
    fn ts_sif2() {
        let s = s(r#"(if (eq 1 1) 1 2)"#).unwrap();
        assert_eq!(evaluate(&(s.0)), Ok(Token::Number(1)));
    }

    #[test]
    fn ts_sif3() {
        let s = s(r#"(if (>= 1 3) 1 2)"#).unwrap();
        assert_eq!(evaluate(&(s.0)), Ok(Token::Number(2)));
    }

    #[test]
    fn ts_sif4() {
        let s = s(r#"(if (< 1 3) 1 2)"#).unwrap();
        assert_eq!(evaluate(&(s.0)), Ok(Token::Number(1)));
    }
}

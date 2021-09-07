use super::s::{Edge, S};
use super::token::Token;
use combine::parser::char::{alpha_num, digit, letter, newline, space, string as string_parse};
use combine::{
    attempt, between, choice, look_ahead, many, many1, satisfy, skip_many, token as token_parse,
    EasyParser, Parser, Stream,
};
use std::string::*;
use std::vec::*;

parser! {
    fn devide_space_newline[Input, P](parser: P)(Input) -> P::Output
        where [
        Input: Stream<Token = char>,
        P: Parser<Input>
        ]
    {
        skip_many(space().or(newline())).
            with(parser).
            skip(many::<Vec<_>, _, _>(space().or(newline())))
    }
}
#[test]
fn ts_devide_space_newline() {
    let mut p = devide_space_newline(many::<Vec<char>, _, _>(digit()));
    assert_eq!(p.parse(" 123 ").map(|d| d.0), Ok(vec!['1', '2', '3']));
}

parser! {
    fn special_symbol[Input]()(Input) -> char
        where [
        Input: Stream<Token = char>,
        ]
    {
        token_parse('+')
            .or(token_parse('='))
            .or(token_parse('-'))
            .or(token_parse('*'))
            .or(token_parse('/'))
            .or(token_parse('_'))
            .or(token_parse('?'))
            .or(token_parse('&'))
            .or(token_parse('%'))
            .or(token_parse('$'))
            .or(token_parse('#'))
            .or(token_parse('>'))
            .or(token_parse('<'))
    }
}

parser! {
    fn symbol[Input]()(Input) -> Token
        where [
        Input: Stream<Token = char>,
        ]
    {
        look_ahead(
            attempt(special_symbol().and(letter()).map(|_| ()))
            .or(look_ahead(letter()).map(|_| ()))
            .or(special_symbol().map(|_| ()))
        ) .with(many::<Vec<_>, _, _>(
            alpha_num()
            .or(special_symbol())))
        .map(|s| {
            Token::Symbol(s.iter().collect::<String>())
        })
    }
}

#[test]
fn ts_symbol() {
    assert_eq!(
        symbol().easy_parse("hello world").map(|d| d.0),
        Ok(Token::symbol("hello"))
    );

    assert_eq!(
        symbol().easy_parse("a123").map(|d| d.0),
        Ok(Token::symbol("a123"))
    );

    assert_ne!(
        symbol().easy_parse("123").map(|d| d.0),
        Ok(Token::symbol("123"))
    );
}

parser! {
    fn string[Input]()(Input) -> Token
        where [
        Input: Stream<Token = char>,
        ]
    {
        between(
            token_parse('"'),
            token_parse('"'),
            many::<Vec<_>, _, _>(satisfy(|c| c != '"'))
        ).map(|c| {
            Token::String(c.iter().collect::<String>())
        })
    }
}
#[test]
fn ts_string() {
    assert_eq!(
        string().easy_parse(r#""hello world""#).map(|d| d.0),
        Ok(Token::string("hello world"))
    );
}

parser! {
    fn boolean[Input]()(Input) -> Token
        where [
        Input: Stream<Token = char>,
        ]
    {
        attempt(string_parse("true")).or(string_parse("false"))
            .map(|b|
                match b {
                    "true" => Token::Boolean(true),
                    "false" => Token::Boolean(false),
                    _ => Token::Boolean(false),
                }
            )
    }
}
#[test]
fn ts_boolean() {
    assert_eq!(
        boolean().easy_parse("true").map(|d| d.0),
        Ok(Token::Boolean(true))
    );
    assert_eq!(
        boolean().easy_parse("false").map(|d| d.0),
        Ok(Token::Boolean(false))
    );
}

parser! {
    /// not support decimal
    fn numeric[Input]()(Input) -> Token
        where [
        Input: Stream<Token = char>,
        ]
    {
        choice((
                token_parse('-').and(many1(digit())),
                token_parse('+').and(many1(digit())),
                many1(digit()).map(|v| ('+', v)),
        )).map(|(sign, nums): (_, Vec<char>)| {
            let num = nums.iter()
                .collect::<String>()
                .parse::<isize>()
                .unwrap();
            match sign {
                '+' => Token::Number(num),
                '-' => Token::Number(-num),
                _ => Token::Number(num),
            }
        })
    }
}
#[test]
fn ts_numeric() {
    assert_eq!(
        numeric().easy_parse("123").map(|d| d.0),
        Ok(Token::Number(123))
    );
    assert_eq!(
        numeric().easy_parse("-123").map(|d| d.0),
        Ok(Token::Number(-123))
    );
    assert_eq!(
        numeric().easy_parse("-1024").map(|d| d.0),
        Ok(Token::Number(-1024))
    );
}

parser! {
    fn token[Input]()(Input) -> Edge
        where [
        Input: Stream<Token = char>,
        ]
    {
        choice((
                attempt(numeric()),
                attempt(string()),
                attempt(boolean()),
                symbol(),
        )).map(|t| Edge::Token(t))
    }
}

parser! {
    fn atom[Input]()(Input) -> Edge
        where [
        Input: Stream<Token = char>,
        ]
    {
        devide_space_newline(
            choice((
                    token(),
                    bracket().map(|n| Edge::S(n)),
            ))
        )
    }
}

parser! {
    fn bracket[Input]()(Input) -> S
        where [
        Input: Stream<Token = char>,
        ]
    {
        devide_space_newline(
            between(
                token_parse('('),
                token_parse(')'),
                atom().
                and(many::<Vec<_>, _, _>(atom()).map(|tails| S::from_vector(tails))).
                map(|(head, tail)| S::cons(head, tail)).map(|s| {
                    println!("[{:15}] {:?}", "Lexer", s);
                    s
                })
            )
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ts_bracket1() {
        assert_eq!(
            bracket().easy_parse("(a1 (b1 b2 b3))").map(|d| d.0),
            Ok(S::cons(
                Edge::Token(Token::symbol("a1")),
                S::unit(Edge::S(S::cons(
                    Edge::Token(Token::symbol("b1")),
                    S::cons(
                        Edge::Token(Token::symbol("b2")),
                        S::unit(Edge::Token(Token::symbol("b3")))
                    )
                )))
            ))
        );
    }

    #[test]
    fn ts_bracket2() {
        assert_eq!(
            bracket()
                .easy_parse(
                    "
            (a1 (b1 b2 b3)
                ((d1 d2) c2) a4)
            "
                )
                .map(|d| d.0),
            Ok(S::cons(
                Edge::Token(Token::symbol("a1")),
                S::cons(
                    Edge::S(S::cons(
                        Edge::Token(Token::symbol("b1")),
                        S::cons(
                            Edge::Token(Token::symbol("b2")),
                            S::unit(Edge::Token(Token::symbol("b3")))
                        )
                    )),
                    S::cons(
                        Edge::S(S::cons(
                            Edge::S(S::cons(
                                Edge::Token(Token::symbol("d1")),
                                S::unit(Edge::Token(Token::symbol("d2")))
                            )),
                            S::unit(Edge::Token(Token::symbol("c2")))
                        )),
                        S::unit(Edge::Token(Token::symbol("a4")))
                    )
                )
            ))
        );
    }

    #[test]
    fn ts_bracket3() {
        assert_eq!(
            bracket().easy_parse(" (+ 1 2 3 (- 34 -89)) ").map(|d| d.0),
            Ok(S::cons(
                Edge::Token(Token::symbol("+")),
                S::cons(
                    Edge::Token(Token::Number(1)),
                    S::cons(
                        Edge::Token(Token::Number(2)),
                        S::cons(
                            Edge::Token(Token::Number(3)),
                            S::unit(Edge::S(S::cons(
                                Edge::Token(Token::symbol("-")),
                                S::cons(
                                    Edge::Token(Token::Number(34)),
                                    S::unit(Edge::Token(Token::Number(-89)))
                                ),
                            )))
                        )
                    )
                )
            ))
        );
    }
}

pub fn s(
    src: &str,
) -> Result<(S, &str), combine::easy::Errors<char, &str, combine::stream::PointerOffset<str>>> {
    bracket().easy_parse(src)
}

pub fn parse(
    src: &str,
) -> Result<(Vec<S>, &str), combine::easy::Errors<char, &str, combine::stream::PointerOffset<str>>>
{
    many::<Vec<S>, _, _>(bracket()).easy_parse(src)
}

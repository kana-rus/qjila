mod reader;

use reader::{Reader};
use std::{
    vec::IntoIter as Stream,
    iter::Peekable,
    borrow::Cow,
    format as f,
};


pub type TokenStream = Peekable<Stream<(Location, Token)>>;

pub struct Location {
    pub line:   usize,
    pub column: usize,
} impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { line, column } = self;
        f.write_str(&f!("[{line}:{column}]", ))
    }
}

pub enum Token {
    Ident  (String),
    Literal(Lit),
    
    _enum,
    _model,
    _generator,
    _datasource,

    At,
    Eq,
    Colon,
    Question,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
} impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Token::Ident(name) => name,

            Token::Literal(Lit::Str(s))     => &f!("\"{s}\""),
            Token::Literal(Lit::Bool(b))    => if *b {"true"} else {"false"},
            Token::Literal(Lit::Integer(i)) => &i.to_string(),
            Token::Literal(Lit::Decimal(d)) => &d.to_string(),

            Token::_enum       => "enum",
            Token::_model      => "model",
            Token::_generator  => "generator",
            Token::_datasource => "datasource",

            Token::At           => "@",
            Token::Eq           => "=",
            Token::Colon        => ":",
            Token::Question     => "?",
            Token::ParenOpen    => "(",
            Token::ParenClose   => ")",
            Token::BraceOpen    => "{",
            Token::BraceClose   => "}",
            Token::BracketOpen  => "[",
            Token::BracketClose => "]",
        })
    }
}

pub enum Lit {
    Str    (String),
    Bool   (bool),
    Integer(i128),
    Decimal(f64),
}


pub fn tokenize(file_path: &str) -> Result<TokenStream, Cow<'static, str>> {
    let mut r = Reader::new(file_path)?;

    let mut tokens = Vec::new();
    loop {
        r.skip_whitespace();

        let Some(b)  = r.peek() else {return Ok(tokens.into_iter().peekable())};
        let location = Location {
            line:   r.line().to_owned(),
            column: r.column().to_owned(),
        };

        match b {
            b'@' => {r.consume(1); tokens.push((location, Token::At))}
            b'=' => {r.consume(1); tokens.push((location, Token::Eq))}
            b':' => {r.consume(1); tokens.push((location, Token::Colon))}
            b'?' => {r.consume(1); tokens.push((location, Token::Question))}
            b'(' => {r.consume(1); tokens.push((location, Token::ParenOpen))}
            b')' => {r.consume(1); tokens.push((location, Token::ParenClose))}
            b'{' => {r.consume(1); tokens.push((location, Token::BraceOpen))}
            b'}' => {r.consume(1); tokens.push((location, Token::BraceClose))}
            b'[' => {r.consume(1); tokens.push((location, Token::BracketOpen))}
            b']' => {r.consume(1); tokens.push((location, Token::BracketClose))}

            b'e' => match r.parse_keyword("enum") {
                Ok(_)  => tokens.push((location, Token::_enum)),
                Err(_) => tokens.push((location, Token::Ident(r.parse_ident()?))),
            }
            b'm' => match r.parse_keyword("model") {
                Ok(_)  => tokens.push((location, Token::_model)),
                Err(_) => tokens.push((location, Token::Ident(r.parse_ident()?)))
            }
            b'g' => match r.parse_keyword("generator") {
                Ok(_)  => tokens.push((location, Token::_model)),
                Err(_) => tokens.push((location, Token::Ident(r.parse_ident()?)))
            }
            b'd' => match r.parse_keyword("datasource") {
                Ok(_)  => tokens.push((location, Token::_model)),
                Err(_) => tokens.push((location, Token::Ident(r.parse_ident()?)))
            }

            b't' => match r.parse_keyword("true") {
                Ok(_)  => tokens.push((location, Token::Literal(Lit::Bool(true)))),
                Err(_) => tokens.push((location, Token::Ident(r.parse_ident()?)))
            }
            b'f' => match r.parse_keyword("false") {
                Ok(_)  => tokens.push((location, Token::Literal(Lit::Bool(false)))),
                Err(_) => tokens.push((location, Token::Ident(r.parse_ident()?)))
            }
            b'"' => {
                let literal = r.parse_string_literal()?;
                tokens.push((location, Token::Literal(Lit::Str(literal))))
            }
            b'0'..=b'9' => {
                let integer = r.parse_integer_literal()?;
                if r.parse_keyword(".").is_ok() {
                    tokens.push((location, Token::Literal(Lit::Integer(integer))));
                    continue
                }
                
                let (not_negative, mut abs) = (integer >= 0, integer.abs() as f64);

                let mut min_degit = 1.0_f64;
                while let Some(d) = r.pop_if(|b| b.is_ascii_digit()) {
                    min_degit /= 10.0;
                    abs += (d as f64) / min_degit
                }
                if min_degit == 1.0 {
                    return Err(Cow::Owned(f!("{location} Unexpectedly end of float literal: `{integer}.`")))
                }

                tokens.push((location, Token::Literal(Lit::Decimal(
                    if not_negative { abs } else { - abs }
                ))))
            }
            _ => {
                let unknown_token = String::from_utf8_lossy(r.read_while(|b| !b.is_ascii_whitespace()));
                return Err(Cow::Owned(f!("{location} Unnkown token: `{unknown_token}`")))
            }
        }
    }
}

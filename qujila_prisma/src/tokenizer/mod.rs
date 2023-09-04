mod reader;

use reader::{Reader};
use std::{
    vec::IntoIter as Stream,
    iter::Peekable,
    borrow::Cow,
    format as f,
    fs,
};


pub type TokenStream = Peekable<Stream<(Location, Token)>>;

pub struct Location {
    pub file:   String,
    pub line:   usize,
    pub column: usize,
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
}

pub enum Lit {
    Str    (String),
    Bool   (bool),
    Integer(i128),
    Float  (f64),
}


pub fn tokenize(file: &str) -> Result<TokenStream, Cow<'static, str>> {
    let mut r = Reader::new(file)?;

    let mut tokens = Vec::new();
    loop {
        r.skip_whitespace();

        let Some(b)  = r.peek() else {return Ok(tokens.into_iter().peekable())};
        let location = Location {
            file:   r.file().to_owned(),
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
                match r.parse_keyword(".") {
                    Err(_) => tokens.push((location, Token::Literal(Lit::Integer(integer)))),
                    Ok(_)  => {
                        let fraction = r.pa;
                    }
                }
            }
        }
    }
}

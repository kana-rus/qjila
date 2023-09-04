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
        match r.try_peek()? {
            b'@' => {r.consume(1); tokens.push(Token::At)}
            b'=' => {r.consume(1); tokens.push(Token::Eq)}
            b':' => {r.consume(1); tokens.push(Token::Colon)}
            b'?' => {r.consume(1); tokens.push(Token::Question)}
            b'(' => {r.consume(1); tokens.push(Token::ParenOpen)}
            b')' => {r.consume(1); tokens.push(Token::ParenClose)}
            b'{' => {r.consume(1); tokens.push(Token::BraceOpen)}
            b'}' => {r.consume(1); tokens.push(Token::BraceClose)}
            b'[' => {r.consume(1); tokens.push(Token::BracketOpen)}
            b']' => {r.consume(1); tokens.push(Token::BracketClose)}

            b'e' => match r.parse_keyword("enum") {
                Ok(_)  => tokens.push(Token::_enum),
                Err(_) => tokens.push(Token::Ident(r.parse_ident()?)),
            }
        }
    }

    todo!()
}

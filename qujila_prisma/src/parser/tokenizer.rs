use super::reader::{Reader};
use std::{
    vec::IntoIter as Stream,
    iter::Peekable,
    path::PathBuf,
    borrow::Cow,
    format as f,
};


pub struct TokenStream(
    Peekable<Stream<(Location, Token)>>
); impl Iterator for TokenStream {
    type Item = <Peekable<Stream<(Location, Token)>> as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
} impl TokenStream {
    pub fn peek(&self) -> Option<&(Location, Token)> {
        self.0.peek()
    }
    pub fn try_consume(&mut self, expected: Token) -> Result<(), Cow<'static, str>> {
        let (loc, t) = self.peek().ok_or_else(|| Cow::Owned(f!("Unexpected end of input: Expetced `{expected}`")))?;
        if t == &expected {
            let _ = self.next(); Ok(())
        } else {
            Err(loc.Msg(f!("Expected `{expected}` but found `{t}`")))
        }
    }
    pub fn try_pop_ident(&mut self) -> Result<&Ident, Cow<'static, str>> {
        let (loc, t) = self.peek().ok_or_else(|| Cow::Owned(f!("Unexpected end of input: Expetced an identifier")))?;
        match t {
            Token::Ident(ident) => Ok(ident),
            another => Err(loc.Msg(f!("Expected an identifier but found `{t}`")))
        }
    }
}

pub struct Location {
    pub line:   usize,
    pub column: usize,
} impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { line, column } = self;
        f.write_str(&f!("[{line}:{column}]", ))
    }
} impl Location {
    pub fn Msg(&self, msg: impl AsRef<str>) -> Cow<'static, str> {
        Cow::Owned(f!("[{self}] {}", msg.as_ref()))
    }
}

#[derive(PartialEq)]
pub enum Token {
    Ident  (Ident),
    Literal(Lit),
    Keyword(Keyword),

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
        match self {
            Token::Ident(Ident { name }) => f.write_str(name),

            Token::Literal(Lit::Str(s))     => f.write_str(&f!("\"{s}\"")),
            Token::Literal(Lit::Bool(b))    => f.write_str(if *b {"true"} else {"false"}),
            Token::Literal(Lit::Integer(i)) => f.write_str(&i.to_string()),
            Token::Literal(Lit::Decimal(d)) => f.write_str(&d.to_string()),

            Token::Keyword(Keyword::_enum)       => f.write_str("enum"),
            Token::Keyword(Keyword::_model)      => f.write_str("model"),
            Token::Keyword(Keyword::_generator)  => f.write_str("generator"),
            Token::Keyword(Keyword::_datasource) => f.write_str("datasource"),

            Token::At           => f.write_str("@"),
            Token::Eq           => f.write_str("="),
            Token::Colon        => f.write_str(":"),
            Token::Question     => f.write_str("?"),
            Token::ParenOpen    => f.write_str("("),
            Token::ParenClose   => f.write_str(")"),
            Token::BraceOpen    => f.write_str("{"),
            Token::BraceClose   => f.write_str("}"),
            Token::BracketOpen  => f.write_str("["),
            Token::BracketClose => f.write_str("]"),
        }
    }
}

#[derive(PartialEq)]
pub struct Ident {
    pub name: String,
}

#[derive(PartialEq)]
pub enum Lit {
    Str    (String),
    Bool   (bool),
    Integer(i128),
    Decimal(f64),
}

#[derive(PartialEq)]
pub enum Keyword {
    _enum,
    _model,
    _generator,
    _datasource,
}


pub fn tokenize(file: PathBuf) -> Result<TokenStream, Cow<'static, str>> {
    let mut r = Reader::new(file)?;

    let mut tokens = Vec::new();
    loop {
        r.skip_whitespace();

        let location = Location { line: r.line(), column: r.column()};
        let push     = |token: Token| tokens.push((location, token));
        
        let Some(b) = r.peek() else {return Ok(TokenStream(tokens.into_iter().peekable()))};
        match b {
            b'@' => {r.consume(1); push(Token::At)}
            b'=' => {r.consume(1); push(Token::Eq)}
            b':' => {r.consume(1); push(Token::Colon)}
            b'?' => {r.consume(1); push(Token::Question)}
            b'(' => {r.consume(1); push(Token::ParenOpen)}
            b')' => {r.consume(1); push(Token::ParenClose)}
            b'{' => {r.consume(1); push(Token::BraceOpen)}
            b'}' => {r.consume(1); push(Token::BraceClose)}
            b'[' => {r.consume(1); push(Token::BracketOpen)}
            b']' => {r.consume(1); push(Token::BracketClose)}

            b'e' | b'm' | b'g' | b'd' => match r.parse_oneof_keywords(["enum", "model", "generator", "datasource"]) {
                Err(_) => push(Token::Ident(Ident { name: r.parse_ident()? })),
                Ok(0) => push(Token::Keyword(Keyword::_enum)),
                Ok(1) => push(Token::Keyword(Keyword::_model)),
                Ok(2) => push(Token::Keyword(Keyword::_generator)),
                Ok(3) => push(Token::Keyword(Keyword::_datasource)),
            }

            b't' | b'f' => match r.parse_oneof_keywords(["false", "true"]) {
                Err(_) => push(Token::Ident(Ident { name: r.parse_ident()? })),
                Ok(0)  => push(Token::Literal(Lit::Bool(false))),
                Ok(1)  => push(Token::Literal(Lit::Bool(true))),
            }
            b'f' => match r.parse_keyword("false") {
                Ok(_)  => push(Token::Literal(Lit::Bool(false))),
                Err(_) => push(Token::Ident(Ident { name: r.parse_ident()? })),
            }
            b'"' => {
                let literal = r.parse_string_literal()?;
                push(Token::Literal(Lit::Str(literal)))
            }
            b'0'..=b'9' => {
                let integer = r.parse_integer_literal()?;
                if r.parse_keyword(".").is_ok() {
                    push(Token::Literal(Lit::Integer(integer)));
                    continue
                }
                
                let (not_negative, mut abs) = (integer >= 0, integer.abs() as f64);

                let mut min_degit = 1.0_f64;
                while let Some(d) = r.pop_if(|b| b.is_ascii_digit()) {
                    min_degit /= 10.0;
                    abs += (d as f64) / min_degit
                }
                if min_degit == 1.0 {
                    return Err(Cow::Owned(f!("Unexpectedly end of float literal: `{integer}.`")))
                }

                push(Token::Literal(Lit::Decimal(
                    if not_negative { abs } else { - abs }
                )))
            }
            _ => {
                let unknown_token = String::from_utf8_lossy(r.read_while(|b| !b.is_ascii_whitespace()));
                return Err(Cow::Owned(f!("Unnkown token: `{unknown_token}`")))
            }
        }
    }
}

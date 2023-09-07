use super::reader::{Reader};
use std::{
    vec::IntoIter as Stream,
    iter::Peekable,
    path::PathBuf,
    borrow::Cow,
    format as f,
};


pub struct TokenStream {
    pub current: Location,
    tokens:      Peekable<Stream<(Location, Token)>>
} impl Iterator for TokenStream {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let (loc, t) = self.tokens.next()?;
        self.current = loc;
        Some(t)
    }
} impl TokenStream {
    fn new(tokens: Vec<(Location, Token)>) -> Self {
        Self {
            current: Location { line: 1, column: 0 },
            tokens:  tokens.into_iter().peekable(),
        }
    }

} impl TokenStream {
    pub fn peek(&self) -> Option<&(Location, Token)> {
        self.tokens.peek()
    }
    pub fn try_peek(&self) -> Result<&(Location/* of token you peeked */, Token), Cow<'static, str>> {
        self.peek().ok_or_else(|| self.current.Msg("Unexpectedly input ends with this"))
    }
    pub fn try_consume(&mut self, expected: Token) -> Result<&Location, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        if t == &expected {
            let _ = self.next(); Ok(&self.current)
        } else {
            Err(loc.Msg(f!("Expected `{expected}` but found `{t}`")))
        }
    }
    pub fn try_consume_ident(&mut self, expected_ident: impl AsRef<str>) -> Result<&Location, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Ident(Ident { name }) if &**name == expected_ident.as_ref() => Ok(loc),
            another => Err(loc.Msg(f!("Expected an identifier `{}` but found `{another}`", expected_ident.as_ref())))
        }
    }
    pub fn try_pop_ident(&mut self) -> Result<&Ident, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Ident(ident) => {self.next(); Ok(ident)}
            another => Err(loc.Msg(f!("Expected an identifier but found `{another}`")))
        }
    }
    pub fn try_pop_string_literal(&mut self) -> Result<String, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Literal(Lit::Str(s)) => {self.next(); Ok(s.to_string())}
            other => Err(loc.Msg(f!("Expected a string literal but found `{other}`")))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.peek().is_none()
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
        Cow::Owned(f!("{self} {}", msg.as_ref()))
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
        
        let Some(b) = r.peek() else {return Ok(TokenStream::new(tokens))};
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

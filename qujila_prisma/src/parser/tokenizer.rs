use super::reader::{Reader};
use std::{
    hint::unreachable_unchecked,
    vec::IntoIter as Stream,
    iter::Peekable,
    path::PathBuf,
    borrow::Cow,
    format as f,
};


pub struct TokenStream {
    pub current: Location,
    tokens:      Peekable<Stream<(Location, Token)>>
}

impl TokenStream {
    fn new(tokens: Vec<(Location, Token)>) -> Self {
        Self {
            current: Location { line: 1, column: 0 },
            tokens:  tokens.into_iter().peekable(),
        }
    }
    fn pop_unchecked(&mut self) -> (Location, Token) {
        unsafe {self.pop().unwrap_unchecked()}
    }
}

impl TokenStream {
    pub fn pop(&mut self) -> Option<(Location, Token)> {
        let (loc, t) = self.tokens.next()?;
        self.current = loc;
        Some((loc, t))
    }
    pub fn pop_if(&mut self, condition: impl Fn(&Token)->bool) -> Option<(Location, Token)> {
        self.peek().is_some_and(|(_, t)| condition(t)).then(|| self.pop_unchecked())
    }
    pub fn try_pop(&mut self) -> Result<(Location, Token), Cow<'static, str>> {
        if self.peek().is_some() {
            Ok(self.pop_unchecked())
        } else {
            Err(self.current.Msg("Unexpectedly end of input"))
        }
    }
    pub fn try_pop_ident(&mut self) -> Result<String, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Ident(_) => {let (_, Token::Ident(ident)) = self.pop_unchecked() else {unsafe {unreachable_unchecked()}}; Ok(ident)}
            another => Err(loc.Msg(f!("Expected an identifier but found `{another}`")))
        }
    }

    pub fn try_pop_integer_litreral(&mut self) -> Result<i128, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Literal(Lit::Integer(_))  => {let (_, Token::Literal(Lit::Integer(i))) = self.pop_unchecked() else {unsafe {unreachable_unchecked()}}; Ok(i)}
            other => Err(loc.Msg(f!("Expected an integer literal but found `{other}`")))
        }
    }
    pub fn try_pop_string_literal(&mut self) -> Result<String, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Literal(Lit::Str(_)) => {let (_, Token::Literal(Lit::Str(s))) = self.pop_unchecked() else {unsafe {unreachable_unchecked()}}; Ok(s)}
            other => Err(loc.Msg(f!("Expected a string literal but found `{other}`")))
        }
    }
    pub fn try_pop_decimal_literal(&mut self) -> Result<f64, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Literal(Lit::Decimal(_)) => {let (_, Token::Literal(Lit::Decimal(d))) = self.pop_unchecked() else {unsafe {unreachable_unchecked()}}; Ok(d)}
            other => Err(loc.Msg(f!("Expected a decimal literal but found `{other}`")))
        }
    }
    pub fn try_pop_boolean_literal(&mut self) -> Result<bool, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Literal(Lit::Bool(_)) => {let (_, Token::Literal(Lit::Bool(b))) = self.pop_unchecked() else {unsafe {unreachable_unchecked()}}; Ok(b)}
            other => Err(loc.Msg(f!("Expected `true` or `false` but found `{other}`")))
        }
    }

    pub fn parse_csv<T>(&mut self, parse: impl Fn(&mut Self)->Result<T, Cow<'static, str>>) -> Result<Vec<T>, Cow<'static, str>> {
        let mut csv = vec![];
        if let Ok(first) = parse(self) {
            csv.push(first)
        }
        while self.try_consume(Token::Comma).is_ok() {
            csv.push(parse(self)?)
        }
        {/* allow trailing comma */ self.try_consume(Token::Comma).ok(); }
        Ok(csv)
    }

    pub fn peek(&mut self) -> Option<&(Location, Token)> {
        self.tokens.peek()
    }
    pub fn try_peek(&mut self) -> Result<&(Location/* of token you peeked */, Token), Cow<'static, str>> {
        let err = self.current.Msg("Unexpectedly input ends with this");
        self.peek().ok_or_else(|| err)
    }
    pub fn next_is(&mut self, expected: Token) -> bool {
        match self.peek() {
            None => false,
            Some((_, t)) => t == &expected
        }
    }

    pub fn try_consume(&mut self, expected: Token) -> Result<(), Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        if t == &expected {
            self.pop_unchecked(); Ok(())
        } else {
            Err(loc.Msg(f!("Expected `{expected}` but found `{t}`")))
        }
    }
    pub fn try_consume_ident(&mut self, expected_ident: impl AsRef<str>) -> Result<(), Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Ident(ident) if &**ident == expected_ident.as_ref() => {self.pop_unchecked(); Ok(())},
            another => Err(loc.Msg(f!("Expected an identifier `{}` but found `{another}`", expected_ident.as_ref())))
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.tokens.peek().is_none()
    }
}

#[derive(Clone, Copy)]
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
    Ident  (String),
    Literal(Lit),
    Keyword(Keyword),

    At,
    Eq,
    At2,
    Colon,
    Comma,
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
            Token::Ident(s) => f.write_str(s),

            Token::Literal(Lit::Str(s))     => f.write_str(&f!("\"{s}\"")),
            Token::Literal(Lit::Bool(b))    => f.write_str(if *b {"true"} else {"false"}),
            Token::Literal(Lit::Integer(i)) => f.write_str(&i.to_string()),
            Token::Literal(Lit::Decimal(d)) => f.write_str(&d.to_string()),

            Token::Keyword(Keyword::_enum)       => f.write_str("enum"),
            Token::Keyword(Keyword::_model)      => f.write_str("model"),
            Token::Keyword(Keyword::_generator)  => f.write_str("generator"),
            Token::Keyword(Keyword::_datasource) => f.write_str("datasource"),

            Token::At           => f.write_str("@"),
            Token::At2          => f.write_str("@@"),
            Token::Eq           => f.write_str("="),
            Token::Colon        => f.write_str(":"),
            Token::Comma        => f.write_str(","),
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
pub struct Ident(String);

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
        let mut push = |token: Token| tokens.push((location, token));
        
        let Some(b) = r.peek() else {return Ok(TokenStream::new(tokens))};
        match b {
            b'=' => {r.consume(1); push(Token::Eq)}
            b':' => {r.consume(1); push(Token::Colon)}
            b',' => {r.consume(1); push(Token::Comma)}
            b'?' => {r.consume(1); push(Token::Question)}
            b'(' => {r.consume(1); push(Token::ParenOpen)}
            b')' => {r.consume(1); push(Token::ParenClose)}
            b'{' => {r.consume(1); push(Token::BraceOpen)}
            b'}' => {r.consume(1); push(Token::BraceClose)}
            b'[' => {r.consume(1); push(Token::BracketOpen)}
            b']' => {r.consume(1); push(Token::BracketClose)}

            b'@' => match r.peek() {
                None       => {r.consume(1); push(Token::At); return Ok(TokenStream::new(tokens))}
                Some(b'@') => {r.consume(2); push(Token::At2)}
                Some(_)    => {r.consume(1); push(Token::At)}
            }

            b'e' | b'm' | b'g' | b'd' => match r.parse_oneof_keywords(["enum", "model", "generator", "datasource"]) {
                Ok(0)  => push(Token::Keyword(Keyword::_enum)),
                Ok(1)  => push(Token::Keyword(Keyword::_model)),
                Ok(2)  => push(Token::Keyword(Keyword::_generator)),
                Ok(3)  => push(Token::Keyword(Keyword::_datasource)),
                Ok(_)  => unsafe {unreachable_unchecked()}
                Err(_) => push(Token::Ident(r.parse_ident()?)),
            }

            b't' | b'f' => match r.parse_oneof_keywords(["false", "true"]) {
                Ok(0)  => push(Token::Literal(Lit::Bool(false))),
                Ok(1)  => push(Token::Literal(Lit::Bool(true))),
                Ok(_)  => unsafe {unreachable_unchecked()}
                Err(_) => push(Token::Ident(r.parse_ident()?)),
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

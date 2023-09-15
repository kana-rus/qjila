use super::reader::{Reader};
use std::{
    path::PathBuf,
    borrow::Cow,
    format as f,
};
fn __unreachable__() -> ! {
    unsafe {::std::hint::unreachable_unchecked()}
}


#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct TokenStream {
    pub current: Location,

    /// reversed tokens
    tokens:      Vec<(Location, Token)>
}

impl TokenStream {
    pub(super/* for test */) fn new(mut tokens: Vec<(Location, Token)>) -> Self {
        tokens.reverse();
        Self {
            current: Location { line: 1, column: 1 },
            tokens,
        }
    }

    pub fn pop(&mut self) -> Option<(Location, Token)> {
        let (loc, t) = self.tokens.pop()?;
        self.current = loc;
        Some((loc, t))
    }
    fn pop_unchecked(&mut self) -> (Location, Token) {
        unsafe {self.pop().unwrap_unchecked()}
    }

    pub fn peek(&self) -> Option<&(Location, Token)> {
        self.tokens.last()
    }
}

impl TokenStream {
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
            Token::Ident(_) => {let (_, Token::Ident(ident)) = self.pop_unchecked() else {__unreachable__()}; Ok(ident)}
            another => Err(loc.Msg(f!("Expected an identifier but found `{another}`")))
        }
    }

    pub fn try_pop_integer_litreral(&mut self) -> Result<i128, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Literal(Lit::Integer(_))  => {let (_, Token::Literal(Lit::Integer(i))) = self.pop_unchecked() else {__unreachable__()}; Ok(i)}
            other => Err(loc.Msg(f!("Expected an integer literal but found `{other}`")))
        }
    }
    pub fn try_pop_string_literal(&mut self) -> Result<String, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Literal(Lit::Str(_)) => {let (_, Token::Literal(Lit::Str(s))) = self.pop_unchecked() else {__unreachable__()}; Ok(s)}
            other => Err(loc.Msg(f!("Expected a string literal but found `{other}`")))
        }
    }
    pub fn try_pop_decimal_literal(&mut self) -> Result<f64, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Literal(Lit::Decimal(_)) => {let (_, Token::Literal(Lit::Decimal(d))) = self.pop_unchecked() else {__unreachable__()}; Ok(d)}
            other => Err(loc.Msg(f!("Expected a decimal literal but found `{other}`")))
        }
    }
    pub fn try_pop_boolean_literal(&mut self) -> Result<bool, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        match t {
            Token::Literal(Lit::Bool(_)) => {let (_, Token::Literal(Lit::Bool(b))) = self.pop_unchecked() else {__unreachable__()}; Ok(b)}
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

    pub fn try_peek(&self) -> Result<&(Location/* of token you peeked */, Token), Cow<'static, str>> {
        let err = self.current.Msg("Unexpectedly input ends with this");
        self.peek().ok_or_else(|| err)
    }
    pub fn next_is(&self, expected: Token) -> bool {
        match self.peek() {
            None => false,
            Some((_, t)) => t == &expected
        }
    }

    pub fn try_consume(&mut self, expected: Token) -> Result<Location, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        let loc = loc.clone();
        if t == &expected {
            self.pop_unchecked(); Ok(loc)
        } else {
            Err(loc.Msg(f!("Expected `{expected}` but found `{t}`")))
        }
    }
    pub fn try_consume_ident(&mut self, expected_ident: impl AsRef<str>) -> Result<Location, Cow<'static, str>> {
        let (loc, t) = self.try_peek()?;
        let loc = loc.clone();
        match t {
            Token::Ident(ident) if &**ident == expected_ident.as_ref() => {self.pop_unchecked(); Ok(loc)},
            another => Err(loc.Msg(f!("Expected an identifier `{}` but found `{another}`", expected_ident.as_ref())))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.peek().is_none()
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Debug))]
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
#[cfg_attr(test, derive(Debug))]
pub enum Token {
    Ident  (String),
    Literal(Lit),
    Keyword(Keyword),

    Eq,
    At,
    At2,
    Dot,
    Colon,
    Comma,
    Question,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,

    DocComment(String),
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

            Token::Eq           => f.write_str("="),
            Token::At           => f.write_str("@"),
            Token::At2          => f.write_str("@@"),
            Token::Dot          => f.write_str("."),
            Token::Colon        => f.write_str(":"),
            Token::Comma        => f.write_str(","),
            Token::Question     => f.write_str("?"),
            Token::ParenOpen    => f.write_str("("),
            Token::ParenClose   => f.write_str(")"),
            Token::BraceOpen    => f.write_str("{"),
            Token::BraceClose   => f.write_str("}"),
            Token::BracketOpen  => f.write_str("["),
            Token::BracketClose => f.write_str("]"),

            Token::DocComment(comment) => f.write_str(&f!("comment(( {comment} ))"))
        }
    }
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Lit {
    Str    (String),
    Bool   (bool),
    Integer(i128),
    Decimal(f64),
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Keyword {
    _enum,
    _model,
    _generator,
    _datasource,
}


pub fn tokenize_file(file: PathBuf) -> Result<TokenStream, Cow<'static, str>> {
    tokenize(Reader::file(file)?)
}

pub(crate) fn tokenize(mut r: Reader) -> Result<TokenStream, Cow<'static, str>> {
    let mut tokens = Vec::new();
    loop {
        r.skip_whitespace();

        let location = Location { line: r.line(), column: r.column()};
        let mut push = |token: Token| tokens.push((location, token));
        
        let Some(b) = r.peek() else {return Ok(TokenStream::new(tokens))};
        match b {
            b'=' => {r.consume(1); push(Token::Eq)}
            b'.' => {r.consume(1); push(Token::Dot)}
            b':' => {r.consume(1); push(Token::Colon)}
            b',' => {r.consume(1); push(Token::Comma)}
            b'?' => {r.consume(1); push(Token::Question)}
            b'(' => {r.consume(1); push(Token::ParenOpen)}
            b')' => {r.consume(1); push(Token::ParenClose)}
            b'{' => {r.consume(1); push(Token::BraceOpen)}
            b'}' => {r.consume(1); push(Token::BraceClose)}
            b'[' => {r.consume(1); push(Token::BracketOpen)}
            b']' => {r.consume(1); push(Token::BracketClose)}

            b'@' => match r.peek2() {
                None       => {r.consume(1); push(Token::At); return Ok(TokenStream::new(tokens))}
                Some(b'@') => {r.consume(2); push(Token::At2)}
                Some(_)    => {r.consume(1); push(Token::At)}
            }

            b'/' => match (r.peek2(), r.peek3()) {
                (Some(b'/'), Some(b'/')) => {
                    r.consume(3); r.read_while(|b| b == &b' ');

                    let doc_comment = r.read_while(|b| b != &b'\n');
                    push(Token::DocComment(doc_comment))
                }
                (Some(b'/'), _) => {
                    r.consume(2); r.read_while(|b| b == &b' ');
                    
                    r.read_while(|b| b != &b'\n');
                }
                (_, _) => return Err(Cow::Owned(f!("{location} Crazy `/`")))
            }

            b'e' | b'm' | b'g' | b'd' => match r.parse_oneof_keywords(["enum", "model", "generator", "datasource"]) {
                Ok(0)  => push(Token::Keyword(Keyword::_enum)),
                Ok(1)  => push(Token::Keyword(Keyword::_model)),
                Ok(2)  => push(Token::Keyword(Keyword::_generator)),
                Ok(3)  => push(Token::Keyword(Keyword::_datasource)),
                Ok(_)  => __unreachable__(),
                Err(_) => push(Token::Ident(r.parse_ident()?)),
            }

            b'f' | b't' => match r.parse_oneof_keywords(["false", "true"]) {
                Ok(0)  => push(Token::Literal(Lit::Bool(false))),
                Ok(1)  => push(Token::Literal(Lit::Bool(true))),
                Ok(_)  => __unreachable__(),
                Err(_) => push(Token::Ident(r.parse_ident()?)),
            }
            b'"' => {
                let literal = r.parse_string_literal()?;
                push(Token::Literal(Lit::Str(literal)))
            }
            b'0'..=b'9' => {
                let integer = r.parse_integer_literal()?;
                if r.parse_keyword(".").is_err() {
                    push(Token::Literal(Lit::Integer(integer)));
                    continue
                }
                
                let (not_negative, mut abs) = (integer >= 0, integer.abs() as f64);

                let mut min_degit = 1.0_f64;
                while let Some(d) = r.pop_if(|b| b.is_ascii_digit()) {
                    min_degit /= 10.0;
                    abs += (d as f64) * min_degit
                }
                if min_degit == 1.0 {
                    return Err(Cow::Owned(f!("Unexpectedly end of float literal: `{integer}.`")))
                }

                push(Token::Literal(Lit::Decimal(
                    if not_negative { abs } else { - abs }
                )))
            }
            _ => push(Token::Ident(r.parse_ident()?))
        }
    }
}

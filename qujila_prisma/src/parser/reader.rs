use std::{borrow::Cow, fs, format as f, path::PathBuf};


pub struct Reader {
    content:        Vec<u8>,
    current_idx:    usize,
    current_line:   usize,
    current_column: usize,
}

impl Reader {
    pub fn new(path: PathBuf) -> Result<Self, Cow<'static, str>> {
        let content = fs::read(path.as_path()).map_err(|e| Cow::Owned(f!("Can't read file `{}`: {e}", path.display())))?;
        Ok(Self {
            content,
            current_idx:    0,
            current_line:   1,
            current_column: 1,
        })
    }
    pub fn Msg(&self, message: impl AsRef<str>) -> Cow<'static, str> {
        Cow::Owned(f!("[{}:{}] {}", self.line(), self.column(), message.as_ref()))
    }
}

impl Reader {
    #[inline(always)] fn remained(&self) -> &[u8] {
        &self.content[self.current_idx..]
    }

    #[inline(always)] pub fn line(&self) -> usize {
        self.current_line
    }
    #[inline(always)] pub fn column(&self) -> usize {
        self.current_column
    }

    pub fn read(&mut self, max_bytes: usize) -> &[u8] {
        let start_idx = self.current_idx;

        let remained = self.remained();
        let add_idx  = max_bytes.min(remained.len());

        let mut line   = self.current_line.clone();
        let mut column = self.current_column.clone();

        for b in &remained[..add_idx] {
            if &b'\n' != b {
                column += 1
            } else {
                line += 1; column = 1
            }
        }

        self.current_idx += add_idx;
        self.current_line   = line;
        self.current_column = column;

        &self.content[start_idx..(start_idx + add_idx)]
    }
    pub fn consume(&mut self, max_bytes: usize) {
        let _ = self.read(max_bytes);
    }
    pub fn pop_if(&mut self, condition: impl Fn(&u8)->bool) -> Option<u8> {
        let value = self.peek()?;
        condition(value).then_some(*value)
    }
    pub fn read_while(&mut self, condition: impl Fn(&u8)->bool) -> &[u8] {
        let mut until = 0;
        while self.remained().get(until).is_some_and(&condition) {
            until += 1
        }
        self.read(until)
    }

    #[inline] pub fn peek(&self) -> Option<&u8> {
        self.remained().first()
    }
    pub fn try_peek(&self) -> Result<&u8, Cow<'static, str>> {
        self.peek().ok_or_else(|| Cow::Borrowed("Unexpectedly end of input"))
    }
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn skip_whitespace(&mut self) {
        while self.remained().first().is_some_and(|b| b.is_ascii_whitespace()) {
            self.consume(1)
        }
    }
}

impl Reader {
    pub fn parse_keyword(&mut self, keyword: &'static str) -> Result<(), Cow<'static, str>> {
        self.remained().starts_with(keyword.as_bytes())
            .then(|| self.consume(keyword.len()))
            .ok_or_else(|| self.Msg(f!("Expected keyword `{keyword}` but not found")))
    }
    pub fn parse_oneof_keywords<const N: usize>(&mut self, keywords: [&'static str; N]) -> Result<usize, Cow<'static, str>> {
        for i in 0..keywords.len() {
            if self.remained().starts_with(&keywords[i].as_bytes()) {
                self.consume(keywords[i].len());
                return Ok(i)
            }
        }
        Err(self.Msg(f!("Expected oneof {} but none matched", keywords.map(|kw| f!("`{kw}`")).join(", "))))
    }
    pub fn parse_ident(&mut self) -> Result<String, Cow<'static, str>> {
        let mut ident_len = 0;
        while !self.remained()[ident_len].is_ascii_whitespace() {
            ident_len += 1
        }
        if ident_len == 0 {return Err(self.Msg("Expected an ident but not found"))}

        self.consume(ident_len);
        Ok(unsafe { String::from_utf8_unchecked(self.remained()[..ident_len].to_vec()) })
    }
    pub fn parse_string_literal(&mut self) -> Result<String, Cow<'static, str>> {
        self.parse_keyword("\"")?;
        let mut literal_bytes = Vec::new();
        while self.remained().first().is_some_and(|b| &b'"' != b) {
            literal_bytes.push(self.remained()[0]);
            self.consume(1)
        }
        self.parse_keyword("\"")?;

        Ok(unsafe { String::from_utf8_unchecked(literal_bytes) })
    }
    pub fn parse_positive_integer_literal(&mut self) -> Result<u64, Cow<'static, str>> {
        let mut integer = 0;

        let mut degit   = 0;
        loop {
            let b = self.remained().first()
                .ok_or_else(|| self.Msg("Expected an integer but not found"))?;
            match b {
                b'0'..=b'9' => {integer = integer * 10 + (*b - b'0') as u64; degit += 1}
                _ => break,
            }
        }
        if degit == 0 {return Err(self.Msg("Expected an integer but not found"))}

        self.consume(degit);
        Ok(integer)
    }
    pub fn parse_integer_literal(&mut self) -> Result<i128, Cow<'static, str>> {
        let negetive = self.parse_keyword("-").is_ok();
        let absolute = self.parse_positive_integer_literal()? as i128;
        
        Ok(if negetive { -absolute } else {absolute})
    }
}

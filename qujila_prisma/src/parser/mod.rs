use std::{borrow::Cow, fs, format as f};



pub(crate) struct Parser {
    file_path:    String,
    content:      Vec<u8>,
    current_idx:  usize,
    current_line: usize,
}

impl Parser {
    
    #[inline(always)] fn remained(&self) -> &[u8] {
        &self.content[self.current_idx..]
    }
    fn read(&mut self, n_bytes: usize) {
        let remained = self.remained();

        let     add_idx  = n_bytes.min(remained.len());
        let mut add_line = 0;

        let mut was_eol = false;
        for b in &remained[..add_idx] {
            if was_eol {add_line += 1}
            was_eol = &b'\n' == b
        }

        self.current_idx  += add_idx;
        self.current_line += add_line;
    }
    fn skip_whitespace(&mut self) {
        while self.remained().first().is_some_and(|b| b.is_ascii_whitespace()) {
            self.read(1)
        }
    }
}

impl Parser {
    pub fn new(path: &str) -> Result<Self, Cow<'static, str>> {
        let content = fs::read(path).map_err(|e| Cow::Owned(f!("Can't read file `{path}`: {e}")))?;
        Ok(Self { content, file_path: f!("{path}"), current_idx: 0, current_line: 1 })
    }
}

impl Parser {
    pub fn parse_keyword(&mut self, keyword: &'static str) -> Result<(), Cow<'static, str>> {
        self.skip_whitespace();
        self.remained().starts_with(keyword.as_bytes())
            .then(|| self.read(keyword.len()))
            .ok_or_else(|| Cow::Owned(
                f!("[{}::line{}] Expected a keyword `{keyword}` but not found",
                &self.file_path,
                &self.current_line
            )))
    }
    pub fn parse_oneof_keywords<const N: usize>(&mut self, keywords: [&'static str; N]) -> Result<usize, Cow<'static, str>> {
        self.skip_whitespace();
        for i in 0..keywords.len() {
            if self.remained().starts_with(&keywords[i].as_bytes()) {
                self.read(keywords[i].len());
                return Ok(i)
            }
        }
        Err(Cow::Owned(f!("[{}::line{}] Expected oneof {} but none matched",
            &self.file_path,
            &self.current_line,
            keywords.map(|kw| f!("`{kw}`")).join(", ")
        )))
    }
    pub fn parse_ident(&mut self) -> Result<String, Cow<'static, str>> {
        self.skip_whitespace();

        let mut ident_len = 0;
        while !self.remained()[ident_len].is_ascii_whitespace() {
            ident_len += 1
        }

        if ident_len == 0 {return Err(Cow::Owned(f!(
            "[{}::line{}] Expected an ident but not found",
            &self.file_path,
            &self.current_line,
        )))}

        self.read(ident_len);
        Ok(unsafe { String::from_utf8_unchecked(self.remained()[..ident_len].to_vec()) })
    }
    pub fn parse_string_literal(&mut self) -> Result<String, Cow<'static, str>> {
        self.skip_whitespace();

        self.parse_keyword("\"")?;
        let mut literal_bytes = Vec::new();
        while self.remained().first().is_some_and(|b| &b'"' != b) {
            literal_bytes.push(self.remained()[0]);
            self.read(1)
        }
        self.parse_keyword("\"")?;

        Ok(unsafe { String::from_utf8_unchecked(literal_bytes) })
    }
    pub fn parse_int_literal(&mut self) -> Result<i32, Cow<'static, str>> {
        self.skip_whitespace();

        let mut int   = 0;
        let mut degit = 0;
        loop {
            let b = self.remained().first().ok_or_else(|| Cow::Owned(f!(
                "[{}::line{}] Expected an Int value but not found",
                &self.file_path,
                &self.current_line,
            )))?;
            match b {
                b'0'..=b'9' => {int += (*b - b'0') as i32; degit += 1}
                _ => break,
            }
        }

        if degit == 0 {return Err(Cow::Owned(f!(
            "[{}::line{}] Expected an Int value but not found",
            &self.file_path,
            &self.current_line,
        )))}

        self.read(degit);
        Ok(int)
    }
    pub fn parse_bigint_literal(&mut self) -> Result<i64, Cow<'static, str>> {
        self.skip_whitespace();

        let mut int   = 0;
        let mut degit = 0;
        loop {
            let b = self.remained().first().ok_or_else(|| Cow::Owned(f!(
                "[{}::line{}] Expected an Int value but not found",
                &self.file_path,
                &self.current_line,
            )))?;
            match b {
                b'0'..=b'9' => {int += (*b - b'0') as i64; degit += 1}
                _ => break,
            }
        }

        if degit == 0 {return Err(Cow::Owned(f!(
            "[{}::line{}] Expected an Int value but not found",
            &self.file_path,
            &self.current_line,
        )))}

        self.read(degit);
        Ok(int)
    }
}

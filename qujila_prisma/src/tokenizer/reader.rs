use std::{borrow::Cow, fs, format as f};


pub(crate) struct Reader {
    file_path:      String,
    content:        Vec<u8>,
    current_idx:    usize,
    current_line:   usize,
    current_column: usize,
}

impl Reader {
    pub fn new(path: &str) -> Result<Self, Cow<'static, str>> {
        let content = fs::read(path).map_err(|e| Cow::Owned(f!("Can't read file `{path}`: {e}")))?;
        Ok(Self { content, file_path: f!("{path}"),
            current_idx:    0,
            current_line:   1,
            current_column: 1,
        })
    }
}

impl Reader {
    #[inline(always)] fn remained(&self) -> &[u8] {
        &self.content[self.current_idx..]
    }

    pub fn read(&mut self, max_bytes: usize) -> &[u8] {
        let remained = self.remained();
        let add_idx  = max_bytes.min(remained.len());
        let read_bytes = &remained[..add_idx];

        let mut line   = self.current_line.clone();
        let mut column = self.current_column.clone();

        let mut was_newline = false;
        for b in read_bytes {
            column += 1;
            if was_newline {line += 1; column = 1}
            was_newline = &b'\n' == b
        }

        self.current_idx += add_idx;
        self.current_line   = line;
        self.current_column = column;

        read_bytes
    }
    pub fn consume(&mut self, max_bytes: usize) {
        let _ = self.read(max_bytes);
    }

    #[inline] pub fn peek(&self) -> Option<&u8> {
        self.remained().first()
    }
    pub fn try_peek(&self) -> Result<&u8, Cow<'static, str>> {
        self.peek().ok_or_else(|| Cow::Borrowed("Unexpectedly end of input"))
    }

    pub fn skip_whitespace(&mut self) {
        while self.remained().first().is_some_and(|b| b.is_ascii_whitespace()) {
            self.consume(1)
        }
    }
}

impl Reader {
}

impl Reader {
    pub fn parse_keyword(&mut self, keyword: &'static str) -> Result<(), Cow<'static, str>> {
        self.skip_whitespace();
        self.remained().starts_with(keyword.as_bytes())
            .then(|| self.consume(keyword.len()))
            .ok_or_else(|| Cow::Owned(
                f!("[{}:{}:{}] Expected a keyword `{keyword}` but not found",
                &self.file_path,
                &self.current_line,
                &self.current_column,
            )))
    }
    pub fn parse_oneof_keywords<const N: usize>(&mut self, keywords: [&'static str; N]) -> Result<usize, Cow<'static, str>> {
        self.skip_whitespace();
        for i in 0..keywords.len() {
            if self.remained().starts_with(&keywords[i].as_bytes()) {
                self.consume(keywords[i].len());
                return Ok(i)
            }
        }
        Err(Cow::Owned(f!("[{}:{}:{}] Expected oneof {} but none matched",
            &self.file_path,
            &self.current_line,
            &self.current_column,
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
            "[{}:{}:{}] Expected an ident but not found",
            &self.file_path,
            &self.current_line,
            &self.current_column,
        )))}

        self.consume(ident_len);
        Ok(unsafe { String::from_utf8_unchecked(self.remained()[..ident_len].to_vec()) })
    }
    pub fn parse_string_literal(&mut self) -> Result<String, Cow<'static, str>> {
        self.skip_whitespace();

        self.parse_keyword("\"")?;
        let mut literal_bytes = Vec::new();
        while self.remained().first().is_some_and(|b| &b'"' != b) {
            literal_bytes.push(self.remained()[0]);
            self.consume(1)
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
                "[{}:{}:{}] Expected an Int value but not found",
                &self.file_path,
                &self.current_line,
                &self.current_column,
            )))?;
            match b {
                b'0'..=b'9' => {int += (*b - b'0') as i32; degit += 1}
                _ => break,
            }
        }

        if degit == 0 {return Err(Cow::Owned(f!(
            "[{}:{}:{}] Expected an Int value but not found",
            &self.file_path,
            &self.current_line,
            &self.current_column,
        )))}

        self.consume(degit);
        Ok(int)
    }
    pub fn parse_bigint_literal(&mut self) -> Result<i64, Cow<'static, str>> {
        self.skip_whitespace();

        let mut int   = 0;
        let mut degit = 0;
        loop {
            let b = self.remained().first().ok_or_else(|| Cow::Owned(f!(
                "[{}:{}:{}] Expected an Int value but not found",
                &self.file_path,
                &self.current_line,
                &self.current_column,
            )))?;
            match b {
                b'0'..=b'9' => {int += (*b - b'0') as i64; degit += 1}
                _ => break,
            }
        }

        if degit == 0 {return Err(Cow::Owned(f!(
            "[{}:{}:{}] Expected an Int value but not found",
            &self.file_path,
            &self.current_line,
            &self.current_column,
        )))}

        self.consume(degit);
        Ok(int)
    }
}

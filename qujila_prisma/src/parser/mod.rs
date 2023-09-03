use std::{borrow::Cow, fs, format as f};



pub(crate) struct Parser {
    file_path:    String,
    content:      Vec<u8>,
    current_idx:  usize,
    current_line: usize,
}

impl Parser {
    pub(crate) fn new(path: &str) -> Result<Self, Cow<'static, str>> {
        let content = fs::read(path).map_err(|e| Cow::Owned(f!("Can't read file `{path}`: {e}")))?;
        Ok(Self { content, file_path: f!("{path}"), current_idx: 0, current_line: 1 })
    }
    #[inline] fn remained(&self) -> &[u8] {
        &self.content[self.current_idx..]
    }
    fn read(&mut self, n_bytes: usize) {
        let remained = self.remained();

        let     add_idx  = n_bytes.min(remained.len());
        let mut add_line = 0;

        let mut is_eol = false;
        for b in &remained[..add_idx] {
            if is_eol {add_line += 1}
            is_eol = &b'\n' == b
        }

        self.current_idx  += add_idx;
        self.current_line += add_line;
    }
}

impl Parser {
    pub(crate) fn skip_whitespace(&mut self) {
        while self.remained().first().is_some_and(|b| b.is_ascii_whitespace()) {
            self.read(1)
        }
    }
    pub(crate) fn parse_keyword(&mut self, keyword: &'static str) -> Result<(), Cow<'static, str>> {
        self.remained().starts_with(keyword.as_bytes())
            .then(|| self.read(keyword.len()))
            .ok_or_else(|| Cow::Owned(
                f!("{}:line{}: Expected a keyword `{keyword}` but not found",
                &self.file_path,
                &self.current_line
            )))
    }
}

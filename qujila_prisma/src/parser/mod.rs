mod reader;
mod tokenizer;

#[cfg(test)] mod reader_test;

pub use tokenizer::*;


pub trait Parse: Sized {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>>;
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        ts.try_consume(Token::BracketOpen)?;
        let vec = ts.parse_csv(T::parse)?;
        ts.try_consume(Token::BracketClose)?;
        Ok(vec)
    }
}

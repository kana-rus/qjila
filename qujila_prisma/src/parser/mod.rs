mod reader;
mod tokenizer;

pub use tokenizer::*;

pub trait Parse: Sized {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>>;
}

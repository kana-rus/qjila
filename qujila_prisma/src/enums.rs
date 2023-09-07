use crate::*;

pub struct Enum {
    pub name:     String,
    pub variants: Vec<String>,
}

impl Parse for Enum {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        ts.try_consume(Token::Keyword(Keyword::_enum))?;

        let name = ts.try_pop_ident()?.name;

        ts.try_consume(Token::BraceOpen)?;
        let mut variants = Vec::new();
        while let Ok(v) = ts.try_pop_ident() {
            variants.push(v.name)
        }
        ts.try_consume(Token::BraceClose)?;

        Ok(Self { name, variants })
    }
}

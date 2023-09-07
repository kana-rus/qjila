use crate::*;

pub struct GeneratorClient {
    pub provider: String,
    pub output:   Option<String>,
}

impl Parse for GeneratorClient {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        ts.try_consume(Token::Keyword(Keyword::_generator))?;
        ts.try_consume_ident("client")?;

        ts.try_consume(Token::BraceOpen)?;
        let mut provider = None;
        let mut output   = None;
        while let Ok(key) = ts.try_pop_ident() {
            match &*key.name {
                "provider" => {
                    if provider.is_some() {return Err(ts.current.Msg("Duplicate definition of `provider`"))}
                    ts.try_consume(Token::Eq)?;

                    let p = ts.try_pop_string_literal()?;
                    provider = Some(p)
                }
                "output" => {
                    if output.is_some() {return Err(ts.current.Msg("Duplicate definition of `output`"))}
                    ts.try_consume(Token::Eq)?;

                    let o = ts.try_pop_string_literal()?;
                    output = Some(o)
                }
                other => return Err(ts.current.Msg(f!("Expected one of `provider`, `output` buf found `{other}`")))
            }
        }
        ts.try_consume(Token::BraceClose)?;

        Ok(Self {
            provider: provider.ok_or_else(|| Cow::Borrowed("No `provider` found in `generator`"))?,
            output
        })
    }
}

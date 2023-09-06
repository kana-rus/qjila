use crate::parser::*;
use std::{borrow::Cow, format as f};


pub struct DataSource {
    pub name:     String,
    pub provider: Provider,
    pub url:      String,
} impl Parse for DataSource {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        ts.try_consume(Token::Keyword(Keyword::_datasource))?;
        let Ident { name } = ts.try_pop_ident()?;

        let mut provider = Err(Cow::Owned(f!("No provider found in datasource block")));
        let mut url      = Err(Cow::Owned(f!("No url found in datasource block")));
        ts.try_consume(Token::BraceOpen)?;
        while let Some(t) = ts.next() {
            match &t {
                Token::Ident(Ident { name }) if name == "provider" => {
                    if provider.is_ok() {return Err(ts.current.Msg("Found duplicata definition of `provider`"))}
                    ts.try_consume(Token::Eq)?;
                    let Lit::Str(s) = ts.try_pop_literal()?
                        else {return Err(ts.current.Msg("Expected a string literal but found"))};
                    provider = Ok(Provider::parse(name)?)
                }
                Token::Ident(Ident { name }) if name == "url" => {
                    if url.is_ok() {return Err(ts.current.Msg("Found duplicata definition of `url`"))}
                    ts.try_consume(Token::Eq)?;
                    let Lit::Str(s) = ts.try_pop_literal()?
                        else {return Err(ts.current.Msg("Expected a string literal"))};
                    url = Ok(s.to_owned())
                }
                another => return Err(ts.current.Msg(f!("Expected one of `provider`, `url` but found `{another}`")))
            }
        }
        ts.try_consume(Token::BraceClose)?;

        Ok(Self {
            name:     name.to_owned(),
            provider: provider?,
            url:      url?
        })
    }
}

pub enum Provider {
    postgresql,
    mysql,
    sqlite,
} impl Provider {
    fn parse(input: &str) -> Result<Self, Cow<'static, str>> {
        match input {
            "postgresql" => Ok(Self::postgresql),
            "mysql"      => Ok(Self::mysql),
            "sqlite"     => Ok(Self::sqlite),
            another      => Err(Cow::Owned(f!("datasource::provider: Expected oneof `postgres`, `mysql`, `sqlte` but found `{another}`")))
        }
    }
}

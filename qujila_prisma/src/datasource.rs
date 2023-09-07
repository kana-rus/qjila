use crate::*;
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
                Token::Ident(key) => match &*key.name {
                    "provider" => {
                        if provider.is_ok() {return Err(ts.current.Msg("Found duplicatae definition of `provider`"))}
                        ts.try_consume(Token::Eq)?;

                        let Lit::Str(s) = ts.try_pop_literal()?
                            else {return Err(ts.current.Msg("Expected a string literal but found"))};
                        provider = Ok(Provider::from_str(s)?)
                    }
                    "url" => {
                        if url.is_ok() {return Err(ts.current.Msg("Found duplicatae definition of `url`"))}
                        ts.try_consume(Token::Eq)?;

                        url = Ok({
                            let (loc, t) = ts.try_peek()?;
                            match t {
                                Token::Ident(i) => {
                                    // `url = env("...")`
                                    env::parse(ts)?.eval()?
                                }
                                Token::Literal(Lit::Str(s)) => {
                                    ts.next();
                                    s.to_owned()
                                }
                                another => return Err(loc.Msg(f!("Expected string expression but found `{another}`")))
                            }
                        })
                    }
                    another => return Err(ts.current.Msg(f!("Expected one of `provider`, `url` but found `{another}`")))
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
    fn from_str(input: &str) -> Result<Self, Cow<'static, str>> {
        match input {
            "postgresql" => Ok(Self::postgresql),
            "mysql"      => Ok(Self::mysql),
            "sqlite"     => Ok(Self::sqlite),
            another      => Err(Cow::Owned(f!("datasource::provider: Expected oneof `postgres`, `mysql`, `sqlte` but found `{another}`")))
        }
    }
}

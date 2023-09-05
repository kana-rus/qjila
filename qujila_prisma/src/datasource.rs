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

        ts.try_consume(Token::BraceOpen)?;
        //
    }
}

pub enum Provider {
    postgresql,
    mysql,
    sqlite,
}

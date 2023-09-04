use crate::parser::*;
use std::{borrow::Cow, format as f};


pub struct DataSource {
    pub name:     String,
    pub provider: Provider,
    pub url:      String,
} impl Parse for DataSource {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let _ = match ts.next().ok_or_else(|| Cow::Borrowed("Unexpectedly end of input"))? {
            (_, Token::_datasource) => (),
            (loc, _) => return Err(loc.Msg("Expected keyword `generator`"))
        };
        let name = match ts.next().ok_or_else(|| Cow::Borrowed("Unexpectedly end of input"))? {
            (_, Token::Ident(name)) => name,
            (loc, _) => return Err(loc.Msg("Expected an identifier"))
        };
        let _ = 
    }
}

pub enum Provider {
    postgresql,
    mysql,
    sqlite,
}

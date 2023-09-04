use std::{borrow::Cow, format as f};
use crate::{
    Schema, DataSource, GeneratorClient, Model, Enum,
    tokenizer::*,
};


pub trait Parse: Sized {
    fn parse(ts: &mut TokenStream) -> Result<Self, Cow<'static, str>>;
}

impl Parse for Schema {
    fn parse(ts: &mut TokenStream) -> Result<Self, Cow<'static, str>> {
        let mut generator  = None;
        let mut datasource = None;
        let mut enums      = vec![];
        let mut models     = vec![];

        while let Some((location, t)) = ts.next() {
            match t {
            //    Token::_generator  => {
            //        if generator.is_some() {return Err(Cow::Owned(f!("{location} generator defined mutiple times")))}
            //        generator.replace(GeneratorClient::parse(ts)?);
            //    }
            //    Token::_datasource => {
            //        if datasource.is_some() {return Err(Cow::Owned(f!("{location} datasouce defined multiple times")))}
            //        datasource.replace(DataSource::parse(ts)?);
            //    }
            //    Token::_enum  => enums .push(Enum::parse(ts)?),
            //    Token::_model => models.push(Model::Parse(ts)?),

                unknown => return Err(Cow::Owned(f!("{location} Found unexpected token: `{unknown}`")))
            }
        }

        Ok(Schema {
            generator:  generator .ok_or_else(|| Cow::Owned(f!("`generator` block not found")))?,
            datasource: datasource.ok_or_else(|| Cow::Owned(f!("`datasource` block not found")))?,
            enums,
            models,
        })
    }
}

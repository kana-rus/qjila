#![allow(non_snake_case, non_camel_case_types)]

mod parser;

mod generator_client;
mod datasource;
mod function;
mod model;
#[cfg(test)] mod model_test;

use generator_client::GeneratorClient;
use datasource::DataSource;
use function::Function;
use model::Model;

use std::{borrow::Cow, path::PathBuf, format as f};
use parser::*;


pub mod db_type {
    pub use super::model::*;
}


pub struct Schema {
    pub generator:  GeneratorClient,
    pub datasource: DataSource,
    pub models:     Vec<Model>,
}

impl Parse for Schema {
    fn parse(ts: &mut TokenStream) -> Result<Self, Cow<'static, str>> {
        let mut generator  = None;
        let mut datasource = None;
        let mut models     = vec![];

        while let Some((loc, t)) = ts.peek() {
            match t {
                Token::Keyword(Keyword::_generator)  => {
                    if generator.is_some() {return Err(loc.Msg("Duplicate definition of `generator`"))}
                    generator.replace(GeneratorClient::parse(ts)?);
                }
                Token::Keyword(Keyword::_datasource) => {
                    if datasource.is_some() {return Err(loc.Msg("Duplicate definition of `datasource`"))}
                    datasource.replace(DataSource::parse(ts)?);
                }
                Token::Keyword(Keyword::_enum)  => return Err(loc.Msg("In current version, qujila doesn't support `enum`!")),
                Token::Keyword(Keyword::_model) => models.push(Model::parse(ts)?),

                unknown => return Err(loc.Msg(f!("Unexpected token: `{unknown}`")))
            }
        }

        Ok(Schema {
            generator:  generator .ok_or_else(|| Cow::Owned(f!("No `generator` found")))?,
            datasource: datasource.ok_or_else(|| Cow::Owned(f!("No `datasource` found")))?,
            models,
        })
    }
}

impl Schema {
    pub fn parse(schema_file_path: PathBuf) -> Result<Self, std::borrow::Cow<'static, str>> {
        <Schema as parser::Parse>::parse(
            &mut tokenize_file(schema_file_path)?
        )
    }
}

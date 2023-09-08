#![allow(non_snake_case, non_camel_case_types)]

mod parser;

mod generator_client;
mod datasource;
mod function;
mod enums;
mod model;

use generator_client::GeneratorClient;
use datasource::DataSource;
use function::Function;
use enums::Enum;
use model::Model;

use std::{borrow::Cow, format as f};
use parser::*;


pub struct Schema {
    pub generator:  GeneratorClient,
    pub datasource: DataSource,
    pub enums:      Vec<Enum>,
    pub models:     Vec<Model>,
}

impl Parse for Schema {
    fn parse(ts: &mut TokenStream) -> Result<Self, Cow<'static, str>> {
        let mut generator  = None;
        let mut datasource = None;
        let mut enums      = vec![];
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
                Token::Keyword(Keyword::_enum)  => enums .push(Enum::parse(ts)?),
                Token::Keyword(Keyword::_model) => models.push(Model::parse(ts)?),

                unknown => return Err(loc.Msg(f!("Unexpected token: `{unknown}`")))
            }
        }

        Ok(Schema {
            generator:  generator .ok_or_else(|| Cow::Owned(f!("No `generator` found")))?,
            datasource: datasource.ok_or_else(|| Cow::Owned(f!("No `datasource` found")))?,
            enums,
            models,
        })
    }
}

const _: () = {
    use std::{env, fs, path::{PathBuf, Path}};

    impl Schema {
        pub fn parse() -> Result<Self, std::borrow::Cow<'static, str>> {
            let schema_file_path = find_target_schema_file()?;

            <Schema as parser::Parse>::parse(
                &mut tokenize(schema_file_path)?
            )
        }
    }

    fn find_target_schema_file() -> Result</*absolute*/PathBuf, Cow<'static, str>> {
        let project_root = find_project_root()?;

        let qujila_dir = read_dir(&project_root)?
            .find(|f| f.is_dir() && &f.file_name().unwrap().to_string_lossy() == "qujila")
            .ok_or_else(|| Cow::Borrowed("No qujila directory found"))?;

        let schema_prisma = read_dir(&qujila_dir)?
            .find(|f| f.is_file() && &f.file_name().unwrap().to_string_lossy() == "schema.prisma")
            .ok_or_else(|| Cow::Borrowed("`qujila/schema.prisma` was not found"))?;

        Ok(schema_prisma)
    }
    fn find_project_root() -> Result</*absolute*/PathBuf, Cow<'static, str>> {
        let mut cd = env::current_dir().map_err(|e| Cow::Owned(f!("Can't get current directory: {e}")))?;
        loop {
            if read_dir(&cd)?.any(|f| f.is_file() && &f.file_name().unwrap().to_string_lossy() == "Cargo.toml") {
                return Ok(cd)
            } else {
                let parent = cd.parent().ok_or_else(|| Cow::Borrowed("No Cargo.toml found"))?;
                cd = parent.to_path_buf()
            }
        }
    }
    fn read_dir(path: impl AsRef<Path>) -> Result<impl Iterator<Item = PathBuf>, Cow<'static, str>> {
        Ok(fs::read_dir(path.as_ref())
            .map_err(|e| Cow::Owned(f!("Can't read directory `{}`: {e}", path.as_ref().display())))?
            .collect::<std::io::Result<Vec<_>>>()
            .map_err(|e| Cow::Owned(f!("Can't read a file or directory in `{}`: {e}", path.as_ref().display())))?
            .into_iter().map(|entry| entry.path()))
    }
};

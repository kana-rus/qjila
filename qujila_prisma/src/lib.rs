#![allow(non_snake_case, non_camel_case_types)]

mod tokenizer;
mod parser;

mod generator_client;
mod data_source;
mod enums;
mod model;

use generator_client::GeneratorClient;
use data_source::DataSource;
use enums::Enum;
use model::Model;


pub struct Schema {
    pub generator:  GeneratorClient,
    pub datasource: DataSource,
    pub enums:      Vec<Enum>,
    pub models:     Vec<Model>,
}

impl Schema {
    pub fn parse() -> Result<Self, std::borrow::Cow<'static, str>> {
        let schema_file_path = find_target_schema_file()?;

        <Schema as parser::Parse>::parse(
            &mut tokenizer::tokenize(schema_file_path)?
        )
    }
}




use std::{env, fs, format as f, borrow::Cow, path::{PathBuf, Path}};

fn find_target_schema_file() -> Result</*absolute*/PathBuf, Cow<'static, str>> {
    let project_root = find_project_root()?;
    
    let qujila_dir = read_dir(&project_root)?
        .find(|f| f.is_dir() && &f.file_name().unwrap().to_string_lossy() == "qujila")
        .ok_or_else(|| Cow::Borrowed("No qujila directory found"))?;

    let schema_prisma = read_dir(&qujila_dir)?
        .find(|f| f.is_file() && &f.file_name().unwrap().to_string_lossy() == "schema.prisma")
        .ok_or_else(|| Cow::Borrowed("`qujila/schema` not found"))?;

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

#![allow(non_snake_case, non_camel_case_types)]

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

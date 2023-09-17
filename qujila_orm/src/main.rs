mod generater;
mod type_mapper;

use generater::generate_orm;

use qujila_lib::fs::*;
use qujila_prisma::Schema;

use std::{format as f, borrow::Cow};


fn main() -> Result<(), Cow<'static, str>> {
    let Schema { generator, models, datasource } = Schema::parse(schema_file_path()?)?;

    if &*generator.provider != "qujila" {
        return Ok(())
    }

    let output_dir = {
        let mut p = qujila_dir_path()?;
        p.push(generator.output.unwrap_or_else(|| f!("../src/{}", datasource.name)));
        p
    };

    for model in models {
        generate_orm(model, &output_dir)?
    }

    Ok(())
}

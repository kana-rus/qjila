use super::type_mapper::*;

use qujila_prisma::items::{Model};
use qujila_lib::case::{snake_cased};

use std::{
    format as f,
    fs,
    io::Write,
    path::Path,
    borrow::Cow,
    process::Command,
};


pub fn generate_orm(model: Model, output_dir: &Path) -> Result<(), Cow<'static, str>> {
    let output_file_path = output_dir.join(f!("{}.rs", snake_cased(&model.name)));

    let mut output_file = fs::File::create(&output_file_path)
        .map_err(|e| Cow::Owned(f!("Failed to open file: {e}")))?;

    output_file.write_all(into_orm(model).as_bytes())
        .map_err(|e| Cow::Owned(f!("Failed to write client code: {e}")))?;

    Command::new("rustfmt")
        .arg(output_file_path)
        .spawn()
        .map_err(|e| Cow::Owned(f!("Failed to format client code: {e}")))?;

    Ok(())
}

fn into_orm(model: Model) -> String {
    let mut struct_def = f!(
        "{}pub struct {}",
        (&model.doc_comment).as_deref().unwrap_or("")
            .lines().map(|line| f!("/// {line}\n")).collect::<String>(),
        &model.name
    );
    struct_def.push('{');
    for field in &model.fields {
        let _name_ = &*field.name;
        let _type_ = &*rust_type_name(&field);
        struct_def.push_str(&f!(
            "pub {_name_}: {_type_},"
        ))
    }
    struct_def.push('}');

    f!("{struct_def}{}", [
        create_impl(&model),
    ].concat())
}




fn create_impl(model: &Model) -> String {
    todo!()
}

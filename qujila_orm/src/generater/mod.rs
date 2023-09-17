use qujila_prisma::items::{Model, Field};

use std::{
    format as f,
    fs,
    path::Path,
    borrow::Cow, io::Write,
};


pub fn generate_orm(model: Model, output_dir: &Path) -> Result<(), Cow<'static, str>> {
    let mut output_file = fs::File::create(
        output_dir.join(f!("{}.rs", (&*model.name).to_lowercase()))
    ).map_err(|e| Cow::Owned(f!("Failed to open file: {e}")))?;

    output_file.write_all(
        into_orm(model).as_bytes()
    ).map_err(|e| Cow::Owned(f!("Failed to write client code: {e}")))?;



    Ok(())
}

#[doc = "# This is\ndoc"]
fn into_orm(Model {
    doc_comment,
    name,
    fields,
    ..
}: Model) -> String {
    let doc = doc_comment.unwrap_or(f!(""))
    .lines().map(|line| f!("/// {line}\n")).collect::<String>();

    let mut struct_def = f!("{doc}struct {name} ");

    struct_def.push('{');
    struct_def.push('\n');
    for field in fields {
        TODO
    }
    struct_def.push('}');

    struct_def
}

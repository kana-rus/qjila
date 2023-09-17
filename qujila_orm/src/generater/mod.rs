use qujila_prisma::db_type::Model;

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

    output_file.write_all(f!("\
        uninpremented!()\n\
    ").as_bytes()).map_err(|e| Cow::Owned(f!("Failed to write client code: {e}")))?;

    Ok(())
}

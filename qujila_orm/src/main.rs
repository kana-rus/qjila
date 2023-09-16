use std::borrow::Cow;
use qujila_lib::fs::*;
use qujila_prisma::Schema;


fn main() -> Result<(), Cow<'static, str>> {
    let schema = Schema::parse()?;
    
    todo!();

    Ok(())
}

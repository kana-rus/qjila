use crate::{__feature__, Error};
use sqlx::FromRow as FromSqlxRow;


pub trait FromRow<'r>: Sized {
    fn from_row(row: &'r __feature__::Row) -> Result<Self, Error>;
}
#[cfg(test)]
fn __asert_impls__() {
    fn is_from_sqlx_row<'r, T: FromSqlxRow<'r, __feature__::Row>>() {}

    is_from_sqlx_row::<(i8,)>();
    is_from_sqlx_row::<(i16,)>();
    is_from_sqlx_row::<(i32,)>();
    is_from_sqlx_row::<(i64,)>();
    // is_from_sqlx_row::<(i128,)>();
    // is_from_sqlx_row::<(isize,)>();

    // is_from_sqlx_row::<(u8,)>();
    // is_from_sqlx_row::<(u16,)>();
    // is_from_sqlx_row::<(u32,)>();
    // is_from_sqlx_row::<(u64,)>();
    // is_from_sqlx_row::<(u128,)>();
    // is_from_sqlx_row::<(usize,)>();

    is_from_sqlx_row::<(String,)>();
    is_from_sqlx_row::<(&str,)>();
}
impl<'r, F: FromSqlxRow<'r, __feature__::Row>> FromRow<'r> for F {
    fn from_row(row: &'r __feature__::Row) -> Result<Self, Error> {
        Ok(<F as FromSqlxRow<'r, __feature__::Row>>::from_row(row)?)
    }
}

pub trait Model: for<'r> FromRow<'r> {
    
}

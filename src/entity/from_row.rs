use deadpool_postgres::tokio_postgres::{types::FromSql, Row};


pub trait FromDBValue<'r>: Sized {fn from_db_value(idx: usize, row: &'r Row) -> Self;}
impl<'r, FS: FromSql<'r>> FromDBValue<'r> for FS {fn from_db_value(idx: usize, row: &'r Row) -> Self {row.get(idx)}}

pub trait FromRow<'r>: Sized {
    fn from_row(row: &'r Row) -> Self;
}


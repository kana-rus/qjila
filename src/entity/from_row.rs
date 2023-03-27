use deadpool_postgres::tokio_postgres::{types::FromSql, Row};

#[cfg(test)]
fn __assert_impls__() {
    fn is_from_sql<'r, T: FromSql<'r>>() {}

    is_from_sql::<i8>();
    is_from_sql::<i16>();
    is_from_sql::<i32>();
    is_from_sql::<i64>();
    // is_from_sql::<i128>();
    // is_from_sql::<isize>();

    // is_from_sql::<u8>();
    // is_from_sql::<u16>();
    is_from_sql::<u32>();
    // is_from_sql::<u64>();
    // is_from_sql::<u128>();
    // is_from_sql::<usize>();

    is_from_sql::<String>();
    is_from_sql::<&str>();
}

pub trait FromRow<'r>: Sized {fn from_row(row: &'r Row) -> Self;}
macro_rules! from_row {
    ($row_name:ident {
        $(
            $t:ty => $by:block
        )*
    }) => {
        $(
            impl<'r> FromRow<'r> for $t {
                fn from_row($row_name: &'r Row) -> Self {$by}
            }
        )*
    };
} from_row!(row {
    // for FromSql types
    i8 => {row.get(0)}
    i16 => {row.get(0)}
    i32 => {row.get(0)}
    i64 => {row.get(0)}
    u32 => {row.get(0)}
    String => {row.get(0)}
    &'r str => {row.get(0)}

    // for !FromSql types
    usize => {row.get::<_, i64>(0) as usize}
});

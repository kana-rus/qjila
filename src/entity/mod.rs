mod from_row;

pub use self::from_row::FromRow;

pub trait newEntity {
    type Entity: Entity;

    const NON_DEFAULT_COLUMN_NAMES: &'static str;
    type NonDefaultColumnTypes;

    fn to_sql(&self) -> String;
}

pub trait Entity: for<'r> FromRow<'r> + Sized {
    const TABLE_NAME: &'static str;
}

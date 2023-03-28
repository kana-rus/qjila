mod from_row;
pub use self::from_row::FromRow;


pub trait Entity: for<'r> FromRow<'r> + Sized {
    const TABLE_NAME: &'static str;

    type Creator: CreateEntity;
    type ConditionBuilder: BuildCondition;
    type ColumnSelector: SelectColumn;
}

pub trait CreateEntity {
    const NON_DEFAULT_COLUMN_NAMES: &'static str;
    type NonDefaultColumnTypes;
}

pub trait BuildCondition: Sized {
    fn new() -> Self;
}

pub trait SelectColumn {
    fn new() -> Self;
}
pub struct Column<const NAME: &'static str>;

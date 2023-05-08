use crate::{query, FromRow};

#[cfg(test)] mod __ {
    use sqlx::FromRow;
    use crate::__feature__;

    fn __asert_impls__() {
        fn is_from_sqlx_row<'r, T: FromRow<'r, __feature__::Row>>() {}

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
}


pub trait Model: for<'r> FromRow<'r> {
    const TABLE_NAME:    &'static str;
    const COLUMN_NAMES:  &'static str;
    const UNIQUE_COLUMN: Option<&'static str>;
    type Filter;
    type Selecter;

    fn count(&self) -> query::count<Self>;
    fn exists(&self) -> query::exists<Self>;

    fn single(&self) -> query::Single<Self>;
    fn first(&self) -> query::First<Self>;
    fn all(&self) -> query::All<Self>;
    fn search(&self) -> query::Search<Self>;

    fn delete(&self) -> query::delete<Self>;
    fn deleted(&self) -> query::Delete<Self>;
    // fn deleted_<C: IntoColumns, F: FnOnce(Self::Selecter)->C>(&self, columns: F) -> query::deleted_<C::ReturnType>;

    type Create: Creater<Self>;
    fn create(&self) -> Self::Create;
    type Created: Creater<Self, true, Self>;
    fn created(&self) -> Self::Created;
    // type Created_: Cerater<Self, true, >;
    // fn created_<C: IntoColumns, F: FnOnce(Self::Selecter)->C>(&self, columns: F) -> query::deleted_<C::ReturnType>;

    type Update: Updater;
    fn update(&self) -> Self::Update;
    fn updated(&self) -> Self::Updated;
    // fn updated_<C: IntoColumns, F: FnOnce(Self::Selecter)->C>(&self, columns: F) -> query::deleted_<C::ReturnType>;
}

trait Creater<M: Model, const RETURNING: bool = false, Returning = ()> {}
trait Updater<M: Model, const RETURNING: bool = false, Returning = ()> {}

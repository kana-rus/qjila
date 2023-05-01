use crate::{
    query, Model,
};


pub trait Table: Sized {
    const TABLE_NAME: &'static str;
    const ID_COLUMN:  Option<&'static str>;

    fn count(&self) -> query::count<Self>;
    fn exists(&self) -> query::exists<Self>;
    fn is_single(&self) -> query::is_single<Self>;

    fn delete(&self) -> query::delete<Self>;
    fn Delete<M: Model>(&self) -> query::Delete<Self, M>;

    fn Single<M: Model>(&self) -> query::Single<Self, M>;
    fn First<M: Model>(&self) -> query::First<Self, M>;
    fn All<M: Model>(&self) -> query::All<Self, M>;
    fn Search<M: Model>(&self) -> query::Search<Self, M>;

    type creater;
    fn create(&self) -> Self::creater;
    type Creater;
    fn Create(&self) -> Self::Creater;

    type updater;
    fn update(&self) -> Self::updater;
    type Updater;
    fn Update(&self) -> Self::Updater;
}

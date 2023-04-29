use crate::{
    query,
};


pub trait Querier<const TABLE_NAME: &'static str> {
    fn exists(&self) -> query::exists<TABLE_NAME>;
    fn is_single(&self) -> query::is_single<TABLE_NAME>;

    fn delete(&self);
    fn Delete(&self);

    fn Single(&self);
    fn First(&self);
    fn All(&self);

    type creater;
    fn create(&self) -> Self::creater;
    type Creater;
    fn Create(&self) -> Self::Creater;

    type updater;
    fn update(&self) -> Self::updater;
    type Updater;
    fn Update(&self) -> Self::Updater;
}


pub trait Table {
    const TABLE_NAME: &'static str;
}

pub trait FilteredTable {
    const TABLE_NAME: &'static str;
}

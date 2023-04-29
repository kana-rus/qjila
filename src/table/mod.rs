use crate::{
    query,
};


pub trait Table: Sized {
    const TABLE_NAME: &'static str;
    type Filter;

    fn exists(&self) -> query::exists<Self>;
    fn is_single(&self) -> query::is_single<Self>;

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

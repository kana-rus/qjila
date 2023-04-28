use crate::{
    condition::Condition,
    query,
};


pub trait Querier {
    fn exists(&self) -> query::exists;
    fn is_single(&self);

    fn create(&self);
    fn Create(&self);

    fn update(&self);
    fn Update(&self);

    fn delete(&self);
    fn Delete(&self);

    fn Single(&self);
    fn First(&self);
    fn All(&self);
}


pub trait Table {
    const TABLE_NAME: &'static str;
}

pub trait FilteredTable {
    const TABLE_NAME: &'static str;
}

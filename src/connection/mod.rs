use deadpool_postgres::Object;
use crate::{
    query::*,
    error::Error,
    entity::{Entity, FromRow, CreateEntity},
};


pub struct Connection(
    pub(crate) Object
);
impl Connection {
    pub async fn Create<E: Entity>(&self, new: E) -> Create<E> {
        todo!()
    }
}

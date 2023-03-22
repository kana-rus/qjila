use deadpool_postgres::Object;
use crate::{
    error::Error,
    operation as op,
    entity::{Entity, FromRow, CreateEntity},
};


pub struct Connection(
    pub(crate) Object
);
impl Connection {
    pub async fn Create<E: Entity>(&self, new: E) -> op::Create<E> {
        todo!()
    }
}

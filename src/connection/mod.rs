use deadpool_postgres::Object;
use crate::{
    error::Error,
    entity::{newEntity, Entity, FromRow},
};


pub struct Connection(
    pub(crate) Object
);
impl Connection {
    pub async fn Create<E: newEntity>(&self, new: E) -> Result<E::Entity, Error> {
        let client = &self.0;
        let stmt = client
            .prepare_cached(&format!(
                "INSERT INTO {} ({}) VALUES ($1)",
                <E::Entity as Entity>::TABLE_NAME,
                E::NON_DEFAULT_COLUMN_NAMES,
            ))
            .await?;
        let row = client
            .query_one(&stmt, &[])
            .await?;
        Ok(E::Entity::from_row(&row))
    }
    pub async fn _Create<E: newEntity>(&self, ) -> Result<(), Error> {
        todo!()
    }
}

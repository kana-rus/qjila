use crate::{connection::Connection, error::Error};

pub struct Pool(
    deadpool_postgres::Pool
); impl Pool {
    pub async fn get(&self) -> Result<Connection, Error> {
        Ok(Connection(
            self.0.get().await?
        ))
    }
}

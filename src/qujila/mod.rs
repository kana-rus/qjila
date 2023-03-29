use std::hash::Hasher;
use crate::{connection::Connection, error::Error};


/// A pretty qujila
pub struct Qujila(
    deadpool_postgres::Pool
); impl Qujila {
    pub(crate) async fn next(&self) -> Result<Connection, Error> {
        Ok(Connection(
            self.0.get().await?
        ))
    }
}


macro_rules! cached {
    ($q:ident) => {
        qujila::qujila::cache::<(file!(),line!(),column!())>($q).await
    };
}

pub async fn cache<const KEY: (u32, u32)>(q: &Qujila) -> cached::Qujila<KEY> {
    cached::Qujila(
        q.next().await.expect("Failed to get connection")
    )
}
pub fn key(file: &'static str, line: u32, column: u32) -> cached::key {
    let mut h = fxhash::FxHasher32::default();
    h.write(file.as_bytes());
    h.write_u32(line);
    h.write_u32(column);
    cached::key(h.finish() as u32)
}
mod cached {
    use crate::connection::Connection;

    pub struct Qujila<const KEY: key>(
        pub(super) Connection
    );
    #[derive(PartialEq, Eq)]
    pub struct key(pub(super) u32);
}


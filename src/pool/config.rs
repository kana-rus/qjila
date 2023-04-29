use crate::__feature__::PoolConfig;


#[derive(Clone)]
pub struct Config {
    DB_URL: String,
    max_connections: u32,
} impl Config {
    pub fn max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections = max_connections;
        self
    }
}

impl Config {
    pub(crate) fn new(DB_URL: impl ToString) -> Self {
        Self { DB_URL: DB_URL.to_string(),
            max_connections: 1024,
        }
    }

    pub(crate) fn into_sqlx_pool_config(self) -> (String, PoolConfig) {
        (
            self.DB_URL,
            PoolConfig::new()
                .max_connections(self.max_connections),
        )
    }
}

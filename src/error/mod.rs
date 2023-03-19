use deadpool_postgres::PoolError;
use deadpool_postgres::tokio_postgres::Error as TokioPostgresError;

pub enum Error {
    ConfigError(String),
    DBError(String),
    ParseError(String),
}

impl From<PoolError> for Error {
    fn from(value: PoolError) -> Self {
        Self::ConfigError(value.to_string())
    }
}
impl From<TokioPostgresError> for Error {
    fn from(value: TokioPostgresError) -> Self {
        Self::DBError(value.to_string())
    }
}

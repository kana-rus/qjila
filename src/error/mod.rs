use deadpool_postgres::PoolError;
use deadpool_postgres::tokio_postgres::Error as TokioPostgresError;


#[derive(Debug)]
pub enum Error {
    ConfigError(String),
    DBError(String),
    ParseError(String),
}

impl From<&Error> for Error {
    fn from(value: &Error) -> Self {
        match value {
            Self::ConfigError(e) => Self::ConfigError(e.to_owned()),
            Self::DBError(e) => Self::DBError(e.to_owned()),
            Self::ParseError(e) => Self::ParseError(e.to_owned()),
        }
    }
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

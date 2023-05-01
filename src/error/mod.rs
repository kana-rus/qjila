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
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::DBError(value.to_string())
    }
}

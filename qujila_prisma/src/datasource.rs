use crate::*;
use std::{borrow::Cow, format as f};


#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct DataSource {
    pub doc_comment: Option<String>,

    pub name:     String,
    pub provider: Provider,
    pub url:      String,
} impl Parse for DataSource {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let doc_comment = ts.pop_doc_comment();

        ts.try_consume(Token::Keyword(Keyword::_datasource))?;
        let name = ts.try_pop_ident()?;

        ts.try_consume(Token::BraceOpen)?;
        let mut provider = Err(Cow::Owned(f!("No provider found in datasource block")));
        let mut url      = Err(Cow::Owned(f!("No url found in datasource block")));
        while let Ok(key) = ts.try_pop_ident() {
            match &*key {
                "provider" => {
                    if provider.is_ok() {return Err(ts.current.Msg("Duplicate definition of `provider`"))}
                    ts.try_consume(Token::Eq)?;

                    let p = ts.try_pop_string_literal()?;
                    provider = Ok(Provider::from_str(&p).map_err(|e| ts.current.Msg(f!("{e}")))?)
                }
                "url" => {
                    if url.is_ok() {return Err(ts.current.Msg("Duplicate definition of `url`"))}
                    ts.try_consume(Token::Eq)?;

                    url = Ok({
                        let (loc, t) = ts.try_peek()?;
                        match t {
                            Token::Literal(Lit::Str(_)) => {
                                ts.try_pop_string_literal().unwrap()
                            }
                            Token::Ident(_) => {
                                // `url = env("...")`
                                function::env::parse(ts)?.eval()?
                            }
                            other => return Err(loc.Msg(f!("Expected string expression but found `{other}`")))
                        }
                    })
                }
                other => return Err(ts.current.Msg(f!("Expected onr of `provider`, `url` but found `{other}`")))
            }
        }
        ts.try_consume(Token::BraceClose)?;

        Ok(Self {
            doc_comment,
            name:     name.to_owned(),
            provider: provider?,
            url:      url?
        })
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Provider {
    postgresql,
    mysql,
    sqlite,
} impl Provider {
    fn from_str(input: &str) -> Result<Self, Cow<'static, str>> {
        match input {
            "postgresql" => Ok(Self::postgresql),
            "mysql"      => Ok(Self::mysql),
            "sqlite"     => Ok(Self::sqlite),
            another      => Err(Cow::Owned(f!("datasource::provider: Expected oneof `postgresql`, `mysql`, `sqlite` but found `{another}`")))
        }
    }
}




#[cfg(test)] mod test {
    use super::*;
    use byte_reader::Reader;
    fn bytes(s: &str) -> Vec<u8> {
        s.trim().to_string().into_bytes()
    }


    #[test] fn test_parse_datasource() {
        let input = bytes(r#"
datasource db {
  provider = "postgresql"
  url      = "MY_DB_URL"
}
        "#); assert_eq!(
            DataSource::parse(&mut tokenize(Reader::new(input)).unwrap()).unwrap(),
            DataSource {
                doc_comment: None,
                name:     f!("db"),
                provider: Provider::postgresql,
                url:      f!("MY_DB_URL"),
            }
        );

        let input = bytes(r#"
datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}
        "#); assert_eq!(
            &*DataSource::parse(&mut tokenize(Reader::new(input)).unwrap()).unwrap_err(),
            &*f!("[3:14] Failed to fetch environment variable `DATABASE_URL`: {}", std::env::var("DATABASE_URL").unwrap_err())
        );


        let input = bytes(r#"
/// This defines the datasource.
/// Hello, my name is db!
/// My provider is postgresql and the
/// url of the database is "MY_DB_URL".
datasource db {
  provider = "postgresql"
  url      = "MY_DB_URL"
}
        "#); assert_eq!(
            DataSource::parse(&mut tokenize(Reader::new(input)).unwrap()).unwrap(),
            DataSource {
                doc_comment: Some(r#"
This defines the datasource.
Hello, my name is db!
My provider is postgresql and the
url of the database is "MY_DB_URL".
                "#.trim().to_string()),
                name:     f!("db"),
                provider: Provider::postgresql,
                url:      f!("MY_DB_URL"),
            }
        );
    }
}

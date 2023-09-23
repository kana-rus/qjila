use crate::*;


#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GeneratorClient {
    pub doc_comment: Option<String>,

    pub provider: String,
    pub output:   Option<String>,
}

impl Parse for GeneratorClient {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let doc_comment = ts.pop_doc_comment();

        ts.try_consume(Token::Keyword(Keyword::_generator))?;
        ts.try_consume_ident("client")?;

        ts.try_consume(Token::BraceOpen)?;
        let mut provider = None;
        let mut output   = None;
        while let Ok(key) = ts.try_pop_ident() {
            match &*key {
                "provider" => {
                    if provider.is_some() {return Err(ts.current.Msg("Duplicate definition of `provider`"))}
                    ts.try_consume(Token::Eq)?;

                    let p = ts.try_pop_string_literal()?;
                    provider = Some(p)
                }
                "output" => {
                    if output.is_some() {return Err(ts.current.Msg("Duplicate definition of `output`"))}
                    ts.try_consume(Token::Eq)?;

                    let o = ts.try_pop_string_literal()?;
                    output = Some(o)
                }
                other => return Err(ts.current.Msg(f!("Expected one of `provider`, `output` buf found `{other}`")))
            }
        }
        ts.try_consume(Token::BraceClose)?;

        Ok(Self {
            doc_comment,
            output,
            provider: provider.ok_or_else(|| Cow::Borrowed("No `provider` found in `generator`"))?,
        })
    }
}




#[cfg(test)] mod test {
    use super::*;
    use byte_reader::Reader;
    fn bytes(s: &str) -> Vec<u8> {
        s.trim().to_string().into_bytes()
    }

    #[test] fn test_parse_generator_client() {
        let input = bytes(r#"
generator client {
  provider = "qujila"
}
        "#); assert_eq!(
            GeneratorClient::parse(&mut tokenize(Reader::new(input)).unwrap()).unwrap(),
            GeneratorClient {
                doc_comment: None,
                provider: f!("qujila"),
                output:   None,
            }
        );

        let input = bytes(r#"
/// Hey, This is generator client for this
/// schema file!
/// The provider is "qujila", or Me, and
/// generated files will be output in
/// ../src/qujila directory.
generator client {
  provider = "qujila"
  output   = "../src/qujila"
}
        "#); assert_eq!(
            GeneratorClient::parse(&mut tokenize(Reader::new(input)).unwrap()).unwrap(),
            GeneratorClient {
                doc_comment: Some(r#"
Hey, This is generator client for this
schema file!
The provider is "qujila", or Me, and
generated files will be output in
../src/qujila directory.
                "#.trim().to_string()),
                provider: f!("qujila"),
                output:   Some(f!("../src/qujila")),
            }
        );
    }
}

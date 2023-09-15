use super::{tokenizer::*, reader::*};
use std::format as f;
fn bytes(s: &str) -> Vec<u8> {
    s.trim().to_string().into_bytes()
}
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        {let location = f!("{}:{}:{}", file!(), line!(), column!());
            let left  = $left;
            let right = $right;
            if left != right {
                panic!("\n\
                    ---- {location} ----\n\
                    [left]  {left:#?}\n\
                    [right] {right:#?}\n\
                    \n\
                ")
            }
        }
    };
}



#[test] fn test_tokenize_generator_client() {
    let input = bytes(r#"
generator client {
  provider = "qujila"
  output   = "../src/qujila/"
}
    "#);

    assert_eq!(tokenize(Reader::new(input).unwrap()).unwrap(), TokenStream::new(vec![
        (Location { line:1, column:1  }, Token::Keyword(Keyword::_generator)),
        (Location { line:1, column:11 }, Token::Ident(f!("client"))),
        (Location { line:1, column:18 }, Token::BraceOpen),

        (Location { line:2, column:3  }, Token::Ident(f!("provider"))),
        (Location { line:2, column:12 }, Token::Eq),
        (Location { line:2, column:14 }, Token::Literal(Lit::Str(f!("qujila")))),

        (Location { line:3, column:3  }, Token::Ident(f!("output"))),
        (Location { line:3, column:12 }, Token::Eq),
        (Location { line:3, column:14 }, Token::Literal(Lit::Str(f!("../src/qujila/")))),

        (Location { line:4, column:1  }, Token::BraceClose),
    ]));
}

#[test] fn test_tokenize_datasource() {
    let input = bytes(r#"
datasource db {
  provider = "postgres"
  url      = env("DATABASE_URL")
}
    "#);

    assert_eq!(tokenize(Reader::new(input).unwrap()).unwrap(), TokenStream::new(vec![
        (Location { line:1, column:1  }, Token::Keyword(Keyword::_datasource)),
        (Location { line:1, column:12 }, Token::Ident(f!("db"))),
        (Location { line:1, column:15 }, Token::BraceOpen),

        (Location { line:2, column:3  }, Token::Ident(f!("provider"))),
        (Location { line:2, column:12 }, Token::Eq),
        (Location { line:2, column:14 }, Token::Literal(Lit::Str(f!("postgres")))),

        (Location { line:3, column:3  }, Token::Ident(f!("url"))),
        (Location { line:3, column:12 }, Token::Eq),
        (Location { line:3, column:14 }, Token::Ident(f!("env"))),
        (Location { line:3, column:17 }, Token::ParenOpen),
        (Location { line:3, column:18 }, Token::Literal(Lit::Str(f!("DATABASE_URL")))),
        (Location { line:3, column:32 }, Token::ParenClose),

        (Location { line:4, column:1  }, Token::BraceClose),
    ]));
}

#[test] fn test_tokenize_model() {
    let input = bytes(r#"
model Post {
}
    "#);

    assert_eq!(tokenize(Reader::new(input).unwrap()).unwrap(), TokenStream::new(vec![
        (Location { line:1, column:1  }, Token::Keyword(Keyword::_model)),
        (Location { line:1, column:7  }, Token::Ident(f!("Post"))),
        (Location { line:1, column:12 }, Token::BraceOpen),

        (Location { line:2, column:1  }, Token::BraceClose),
    ]));


    let input = bytes(r#"
model Post {
  title String @db.VarChar(200)
}
    "#);

    assert_eq!(tokenize(Reader::new(input).unwrap()).unwrap(), TokenStream::new(vec![
        (Location { line:1, column:1  }, Token::Keyword(Keyword::_model)),
        (Location { line:1, column:7  }, Token::Ident(f!("Post"))),
        (Location { line:1, column:12 }, Token::BraceOpen),

        (Location { line:2, column:3  }, Token::Ident(f!("title"))),
        (Location { line:2, column:9  }, Token::Ident(f!("String"))),
        (Location { line:2, column:16 }, Token::At),
        (Location { line:2, column:17 }, Token::Ident(f!("db"))),
        (Location { line:2, column:19 }, Token::Dot),
        (Location { line:2, column:20 }, Token::Ident(f!("VarChar"))),
        (Location { line:2, column:27 }, Token::ParenOpen),
        (Location { line:2, column:28 }, Token::Literal(Lit::Integer(200))),
        (Location { line:2, column:31 }, Token::ParenClose),

        (Location { line:3, column:1  }, Token::BraceClose),
    ]));
}

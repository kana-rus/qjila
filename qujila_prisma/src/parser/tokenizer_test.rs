use super::{tokenizer::*, reader::*};
use std::format as f;
fn bytes(s: &str) -> Vec<u8> {
    s.to_string().into_bytes()
}

#[test] fn test_tokenize_model() {
    let input = bytes(
"model Post {
}"
    );

    assert_eq!(tokenize(Reader::new(input).unwrap()).unwrap(), TokenStream::new(vec![
        (Location { line:1, column:1  }, Token::Keyword(Keyword::_model)),
        (Location { line:1, column:7  }, Token::Ident(f!("Post"))),
        (Location { line:1, column:12 }, Token::BraceOpen),

        (Location { line:2, column:1  }, Token::BraceClose),
    ]));


    let input = bytes(
"model Post {
  title String @db.VarChar(200)
}"
    );

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

//     let input =
// "model Post {
//   title     String @db.VarChar(200)
//   n_authors Int    @default(1)
//   z_flag    Int    @default(-42)
// }".to_string().into_bytes();
// 
//     assert_eq!(tokenize(Reader::new(input).unwrap()).unwrap(), TokenStream::new(vec![
//         (Location { line:1, column:0 }, Token::Keyword(Keyword::_model)),
//         (Location { line:1, column:6 }, Token::Ident(f!("Post"))),
//     ]));
}

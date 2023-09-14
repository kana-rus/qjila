use super::{tokenizer::*, reader::*};
use std::format as f;


#[test] fn test_tokenize_model() {
    let mut r = Reader::new(f!("\
        model Post {{\n\
          title     String @db.VarChar(200)\n\
          n_authors Int    @default(1)\n\
          z_flag    Int    @default(-42)\n\
        }}\
    ").into_bytes()).unwrap();

    assert_eq!(tokenize(r).unwrap(), TokenStream::new(vec![]));
}

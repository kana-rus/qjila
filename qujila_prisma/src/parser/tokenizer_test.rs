use super::{tokenizer::*, reader::*};


#[test] fn test_tokenize_model() {
    let mut r = Reader::new("\
        model Post {\n\
          title     String @db.VarChar(200)\n\
          n_authors Int    @default(1)\n\
          z_flag    Int    @default(-42)\n\
        }\
    ".to_string().into_bytes()).unwrap();

    //assert_eq!(tokenize(r).unwrap(), TokenStream::new(vec![]));
}

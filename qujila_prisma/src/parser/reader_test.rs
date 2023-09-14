use super::reader::Reader;
use std::format as f;

#[test] fn test_consume() {
    let mut r = Reader::new(f!(
        "Hello, world!"
    ).into_bytes()).unwrap();

    r.consume(1);
    assert_eq!(r.remained(), b"ello, world!");

    r.consume(3);
    assert_eq!(r.remained(), b"o, world!");
}

#[test] fn test_parse_ident() {
    let mut r = Reader::new(f!(
        "Hello, world! I am a Reader!"
    ).into_bytes()).unwrap();

    let ident = r.parse_ident().unwrap();
    assert_eq!(ident, "Hello");
    assert_eq!(r.remained(), b", world! I am a Reader!");

    assert!(r.parse_ident().is_err());
    r.consume(1);
    assert!(r.parse_ident().is_err());
    r.consume(1);

    let ident = r.parse_ident().unwrap();
    assert_eq!(ident, "world");
    assert_eq!(r.remained(), b"! I am a Reader!")
}

#[test] fn test_parse_string_literal() {
    let mut r = Reader::new(f!("\
        \"Hello,\" He said, \"I am Reader!\"\
    ").into_bytes()).unwrap();

    let lit = r.parse_string_literal().unwrap();
    assert_eq!(lit, "Hello,");
    assert_eq!(r.remained(), b" He said, \"I am Reader!\"");

    assert!(r.parse_string_literal().is_err());
    r.skip_whitespace();
    assert_eq!(r.parse_ident().unwrap(), "He");
    r.skip_whitespace();
    assert_eq!(r.parse_ident().unwrap(), "said");
    assert_eq!(r.peek().unwrap(), &b',');
    r.consume(1);
    r.skip_whitespace();

    let lit = r.parse_string_literal().unwrap();
    assert_eq!(lit, "I am Reader!");
    assert_eq!(r.remained(), b"");
}

#[test] fn test_parse_int() {
    let mut r = Reader::new("\
        model Post {\n\
          title     String @db.VarChar(200)\n\
          n_authors Int    @default(1)\n\
          z_flag    Int    @default(-42)\n\
        }\
    ".to_string().into_bytes()).unwrap();

    assert!(r.parse_keyword("model").is_ok());
    r.skip_whitespace();
    assert_eq!(r.parse_ident().unwrap(), "Post");
    r.skip_whitespace();
    assert_eq!(r.peek().unwrap(), &b'{'); r.consume(1);
    r.skip_whitespace();
    assert_eq!(r.parse_ident().unwrap(), "title");
    r.skip_whitespace();
    assert_eq!(r.parse_ident().unwrap(), "String");
    r.skip_whitespace();
    assert_eq!(r.peek().unwrap(), &b'@'); r.consume(1);
    assert_eq!(r.parse_ident().unwrap(), "db");
    assert!(r.parse_keyword(".").is_ok());
    assert_eq!(r.parse_ident().unwrap(), "VarChar");
    assert_eq!(r.peek().unwrap(), &b'('); r.consume(1);

    let int = r.parse_positive_integer_literal().unwrap();
    assert_eq!(int, 200);
    assert_eq!(r.peek().unwrap(), &b')'); r.consume(1);

    r.skip_whitespace();
    assert_eq!(r.parse_ident().unwrap(), "n_authors");
    r.skip_whitespace();
    assert_eq!(r.parse_ident().unwrap(), "Int");
    r.skip_whitespace();
    assert_eq!(r.peek().unwrap(), &b'@'); r.consume(1);
    assert_eq!(r.parse_ident().unwrap(), "default");
    assert_eq!(r.peek().unwrap(), &b'('); r.consume(1);

    let int = r.parse_integer_literal().unwrap();
    assert_eq!(int, 1);
    assert_eq!(r.peek().unwrap(), &b')'); r.consume(1);

    r.skip_whitespace();
    assert_eq!(r.parse_ident().unwrap(), "z_flag");
    r.skip_whitespace();
    assert_eq!(r.parse_ident().unwrap(), "Int");
    r.skip_whitespace();
    assert_eq!(r.peek().unwrap(), &b'@'); r.consume(1);
    assert_eq!(r.parse_ident().unwrap(), "default");
    assert_eq!(r.peek().unwrap(), &b'('); r.consume(1);

    let int = r.parse_integer_literal().unwrap();
    assert_eq!(int, -42);
    assert_eq!(r.peek().unwrap(), &b')'); r.consume(1);

    r.skip_whitespace();
    assert_eq!(r.peek().unwrap(), &b'}'); r.consume(1);
    assert_eq!(r.peek(), None)
}

use super::reader::Reader;
use std::format as f;

#[test] fn test_consume() {
    let mut r = Reader::new(f!(
        "Hello, world!"
    ).into_bytes()).unwrap();

    r.consume(1);
    assert_eq!(r.remained(), b"ello, world!");
}

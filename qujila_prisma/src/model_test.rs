use crate::*;
use crate::model::*;
fn Model(s: &str) -> Result<Model, Cow<'static, str>> {
    Model::parse(&mut tokenize(Reader::new(
        s.trim().to_string().into_bytes()
    ).unwrap()).unwrap())
}


#[test] fn test_parse_model() {
    assert_eq!(Model(r#"
model Post {}
    "#).unwrap(), Model {
        name:    f!("Post"),
        fields:  vec![],
        map:     None,
        ids:     vec![],
        uniques: vec![],
        indexes: vec![],
    });


    assert_eq!(Model(r#"
model Post {
  id   Int    @id @default(autoincrement())
  name String
}
    "#).unwrap(), Model {
        name:    f!("Post"),
        fields:  vec![
            Field {
                name:   f!("id"),
                schema: FieldSchema::Int(Attributes {
                    id:      true,
                    unique:  false,
                    map:     None,
                    default: Some(IntValue::autoincrement),
                }),
            },
            Field {
                name:   f!("name"),
                schema: FieldSchema::String(Attributes {
                    id:      false,
                    unique:  false,
                    map:     None,
                    default: None,
                }),
            },
        ],
        map:     None,
        ids:     vec![],
        uniques: vec![],
        indexes: vec![],
    });


    assert_eq!(Model(r#"
model Post {
  id   Int    @id @default(autoincrement())
  name String

  @@map("post")
  @@unique([name])
}
    "#).unwrap(), Model {
        name:    f!("Post"),
        fields:  vec![
            Field {
                name:   f!("id"),
                schema: FieldSchema::Int(Attributes {
                    id:      true,
                    unique:  false,
                    map:     None,
                    default: Some(IntValue::autoincrement),
                }),
            },
            Field {
                name:   f!("name"),
                schema: FieldSchema::String(Attributes {
                    id:      false,
                    unique:  false,
                    map:     None,
                    default: None,
                }),
            },
        ],
        map:     Some(f!("post")),
        ids:     vec![],
        uniques: vec![
            vec![f!("name")]
        ],
        indexes: vec![],
    });
}

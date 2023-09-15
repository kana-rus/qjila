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
    "#).unwrap(), Model {doc_comment: None,
        name:    f!("Post"),
        fields:  vec![],
        map:     None,
        ids:     vec![],
        uniques: vec![],
        indexes: vec![],
    });


    assert_eq!(Model(r#"
/// Hi, I am comment!
/// 
model Post {}
    "#).unwrap(), Model {doc_comment: Some(f!("Hi, I am comment!\n")),
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

  @@index([name])
}
    "#).unwrap(), Model {doc_comment: None,
        name:    f!("Post"),
        fields:  vec![
            Field {doc_comment: None,
                name:   f!("id"),
                schema: FieldSchema::Int(Attributes {
                    id:      true,
                    unique:  false,
                    map:     None,
                    default: Some(IntValue::autoincrement),
                }),
            },
            Field {doc_comment: None,
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
        indexes: vec![vec![f!("name")]],
    });


    assert_eq!(Model(r#"
/// This is model representing `Post`, having
/// `id`, `name`, `author_name` columns.
model Post {
  /// This is id column for a a post, this
  /// increments its value automatically.
  id           Int      @id @default(autoincrement())

  /// This is name column of `Post`.
  name         String?  @unique             /// `name` is optional string.

  author_names String[] @map("authorNames") /// This stores the names of people who wrote this post.   

  @@map("post")
  @@unique([name, author_names])
}
    "#).unwrap(), Model {doc_comment: Some(f!("\
        This is model representing `Post`, having\n\
        `id`, `name`, `author_name` columns.")),
        name:    f!("Post"),
        fields:  vec![
            Field {doc_comment: Some(f!("\
                This is id column for a a post, this\n\
                increments its value automatically.")),
                name:   f!("id"),
                schema: FieldSchema::Int(Attributes {
                    id:      true,
                    unique:  false,
                    map:     None,
                    default: Some(IntValue::autoincrement),
                }),
            },
            Field {doc_comment: Some(f!("\
                This is name column of `Post`.\n\
                \n\
                `name` is optional string.")),
                name:   f!("name"),
                schema: FieldSchema::StringOptional(Attributes {
                    id:      false,
                    unique:  true,
                    map:     None,
                    default: None,
                }),
            },
            Field {doc_comment: Some(f!("\
                This stores the names of people who wrote this post.")),
                name:   f!("author_names"),
                schema: FieldSchema::StringList(Attributes {
                    id:      false,
                    unique:  false,
                    map:     Some(f!("authorNames")),
                    default: None,
                }),
            },
        ],
        map:     Some(f!("post")),
        ids:     vec![],
        indexes: vec![],
        uniques: vec![vec![f!("name"), f!("author_names")]],
    });
}

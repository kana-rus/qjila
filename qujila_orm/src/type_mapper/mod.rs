use std::{borrow::Cow, format as f};

use qujila_prisma::items::{FieldSchema, Field};


/// ref:
/// - https://www.prisma.io/docs/concepts/database-connectors/postgresql#native-type-mapping-from-prisma-to-postgresql
pub fn db_type_name(field: &Field) -> &'static str {
    #[cfg(feature="db_postgres")] match &field.schema {
        FieldSchema::String(_)           => "text NOT NULL",
        FieldSchema::StringList(_)       => "text[]",
        FieldSchema::StringOptional(_)   => "text",

        FieldSchema::Boolean(_)          => "boolean NOT NULL",
        FieldSchema::BooleanList(_)      => "boolean[]",
        FieldSchema::BooleanOptional(_)  => "boolean",

        FieldSchema::Int(_)              => "integer NOT NULL",
        FieldSchema::IntList(_)          => "integer[]",
        FieldSchema::IntOptional(_)      => "integer",

        FieldSchema::BigInt(_)           => "bigint NOT NULL",
        FieldSchema::BigIntList(_)       => "bigint[]",
        FieldSchema::BigIntOptional(_)   => "bigint",

        FieldSchema::Float(_)            => "double precision NOT NULL",
        FieldSchema::FloatList(_)        => "double precision[]",
        FieldSchema::FloatOptional(_)    => "double precision",

        FieldSchema::DateTime(_)         => "timestamp(3) NOT NULL",
        FieldSchema::DateTimeList(_)     => "timestamp(3)[]",
        FieldSchema::DateTimeOptional(_) => "timestamp(3)",

        FieldSchema::Bytes(_)            => "bytea NOT NULL",
        FieldSchema::BytesList(_)        => "bytea[]",
        FieldSchema::BytesOptional(_)    => "bytea",

        FieldSchema::Model{..} | FieldSchema::ModelList{..} | FieldSchema::ModelOptional{..} => unreachable!()
    }
}

pub fn rust_type_name(field: &Field) -> Cow<'_, str> {
    match &field.schema {
        FieldSchema::String(_)           => Cow::Borrowed("String"),
        FieldSchema::StringList(_)       => Cow::Borrowed("Vec<String>"),
        FieldSchema::StringOptional(_)   => Cow::Borrowed("Option<String>"),

        FieldSchema::Boolean(_)          => Cow::Borrowed("bool"),
        FieldSchema::BooleanList(_)      => Cow::Borrowed("Vec<bool>"),
        FieldSchema::BooleanOptional(_)  => Cow::Borrowed("Option<bool>"),

        FieldSchema::Int(_)              => Cow::Borrowed("i32"),
        FieldSchema::IntList(_)          => Cow::Borrowed("Vec<i32>"),
        FieldSchema::IntOptional(_)      => Cow::Borrowed("Option<i32>"),

        FieldSchema::BigInt(_)           => Cow::Borrowed("i64"),
        FieldSchema::BigIntList(_)       => Cow::Borrowed("Vec<i64>"),
        FieldSchema::BigIntOptional(_)   => Cow::Borrowed("Option<i64>"),

        FieldSchema::Float(_)            => Cow::Borrowed("f64"),
        FieldSchema::FloatList(_)        => Cow::Borrowed("Vec<f64>"),
        FieldSchema::FloatOptional(_)    => Cow::Borrowed("Option<f64>"),

        FieldSchema::DateTime(_)         => Cow::Borrowed("::chrono::NaiveDateTime"),
        FieldSchema::DateTimeList(_)     => Cow::Borrowed("Vec<::chrono::NaiveDateTime>"),
        FieldSchema::DateTimeOptional(_) => Cow::Borrowed("Option<::chrono::NaiveDateTime>"),

        FieldSchema::Bytes(_)            => Cow::Borrowed("Vec<u8>"),
        FieldSchema::BytesList(_)        => Cow::Borrowed("Vec<Vec<u8>>"),
        FieldSchema::BytesOptional(_)    => Cow::Borrowed("Option<Vec<u8>>"),

        FieldSchema::Model{ model_name, .. }         => Cow::Borrowed(model_name),
        FieldSchema::ModelList{ model_name, .. }     => Cow::Owned(f!("Vec<{model_name}>")),
        FieldSchema::ModelOptional{ model_name, .. } => Cow::Owned(f!("Option<{model_name}>")),
    }
}

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

pub fn rust_type_name(field: &Field) -> &'static str {
    match &field.schema {
        FieldSchema::String(_)           => "String",
        FieldSchema::StringList(_)       => "Vec<String>",
        FieldSchema::StringOptional(_)   => "Option<String>",

        FieldSchema::Boolean(_)          => "bool",
        FieldSchema::BooleanList(_)      => "Vec<bool>",
        FieldSchema::BooleanOptional(_)  => "Option<bool>",

        FieldSchema::Int(_)              => "i32",
        FieldSchema::IntList(_)          => "Vec<i32>",
        FieldSchema::IntOptional(_)      => "Option<i32>",

        FieldSchema::BigInt(_)           => "i64",
        FieldSchema::BigIntList(_)       => "Vec<i64>",
        FieldSchema::BigIntOptional(_)   => "Option<i64>",

        FieldSchema::Float(_)            => "f64",
        FieldSchema::FloatList(_)        => "Vec<f64>",
        FieldSchema::FloatOptional(_)    => "Option<f64>",

        FieldSchema::DateTime(_)         => "::chrono::NaiveDateTime",
        FieldSchema::DateTimeList(_)     => "Vec<::chrono::NaiveDateTime>",
        FieldSchema::DateTimeOptional(_) => "Option<::chrono::NaiveDateTime>",

        FieldSchema::Bytes(_)            => "Vec<u8>",
        FieldSchema::BytesList(_)        => "Vec<Vec<u8>>",
        FieldSchema::BytesOptional(_)    => "Option<Vec<u8>>",

        FieldSchema::Model{..} | FieldSchema::ModelList{..} | FieldSchema::ModelOptional{..} => unreachable!()
    }
}

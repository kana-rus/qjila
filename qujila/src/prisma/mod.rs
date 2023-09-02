#![allow(non_snake_case, non_camel_case_types)]

use chrono::NaiveDateTime; // TIMESTAMP


pub(crate) struct Schema {
    datasource: DataSource,
    generator:  GeneratorClient,
    models:     Vec<Model>,
}

struct DataSource {
    name:     String,
    provider: Provider,
    url:      String,
}
enum Provider { postgresql, mysql, sqlite }

struct GeneratorClient {
    output: Option<String>,
}

struct Model {
    name:    String,
    fields:  Vec<Field>,

    map:     Option<String>,
    ids:     Vec<Vec<String>>,
    uniques: Vec<Vec<String>>,
    indexes: Vec<Vec<String>>,
}
struct Field {
    name:          String,
    schema:        FieldSchema,

    map:           Option<String>,
    id:            Option<()>,
    unique:        Option<()>,
}
enum FieldSchema {
    String         (StringField),
    StringList     (StringListField),
    StringOptional (StringOptionalField),

    Boolean         (BooleanField),
    BooleanList     (BooleanListField),
    BooleanOptional (BooleanOptionalField),

    Int             (IntField),
    IntList         (IntListField),
    IntOptional     (IntOptionalField),

    BigInt          (BigIntField),
    BigIntList      (BigIntListField),
    BigIntOptional  (BigIntOptionalField),

    Float           (FloatField),
    FloatList       (FloatListField),
    FloatOptional   (FloatOptionalField),

    Decimal         (DecimalField),
    DecimalList     (DecimalListField),
    DecimalOptional (DecimalOptionalField),

    Enum            (EnumField),
    EnumList        (EnumListField),
    EnumOptional    (EnumOptionalField),

    DateTime        (DateTimeField),

    Json            (JsonField),

    Bytes           (BytesField),

    Relation        (Relation),
}

struct StringField {
    default: Option<StringValue>,
}
enum StringValue { value(String), cuid, uuid }

struct BooleanField {
    default: Option<bool>,
}
struct IntField {
    default: Option<i32>,
} enum IntValue { value(i32), autoincrement }
struct BigIntField {
    default: Option<i64>,
} enum BigIntValue {}
struct FloatField {
    default: Option<i64>,
}
struct DecimalField {
    default: Option<i128>,
}
struct DateTimeField {
    default:   Option<DateTimeValue>,
    updatedAt: Option<()>,
} enum DateTimeValue { value(NaiveDateTime), now }
struct JsonField  {}
struct BytesField {
    default: Option<String>,
}
struct EnumField {
    default: Option<String>,
}

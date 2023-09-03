mod string;
mod boolean;
mod int;
mod big_int;
mod float;
mod decimal;
mod enums;
mod date_time;
mod json;
mod bytes;
mod relation;


pub struct Model {
    name:    String,
    fields:  Vec<Field>,

    map:     Option<String>,
    ids:     Vec<Vec<String>>,
    uniques: Vec<Vec<String>>,
    indexes: Vec<Vec<String>>,
}

pub struct Field {
    name:          String,
    schema:        FieldSchema,

    map:           Option<String>,
    id:            Option<()>,
    unique:        Option<()>,
}

pub enum FieldSchema {
    String         (string::StringField),
    StringList     (string::StringListField),
    StringOptional (string::StringOptionalField),

    Boolean         (boolean::BooleanField),
    BooleanList     (boolean::BooleanListField),
    BooleanOptional (boolean::BooleanOptionalField),

    Int             (int::IntField),
    IntList         (int::IntListField),
    IntOptional     (int::IntOptionalField),

    BigInt          (big_int::BigIntField),
    BigIntList      (big_int::BigIntListField),
    BigIntOptional  (big_int::BigIntOptionalField),

    Float           (float::FloatField),
    FloatList       (float::FloatListField),
    FloatOptional   (float::FloatOptionalField),

    Decimal         (decimal::DecimalField),
    DecimalList     (decimal::DecimalListField),
    DecimalOptional (decimal::DecimalOptionalField),

    Enum            (EnumField),
    EnumList        (EnumListField),
    EnumOptional    (EnumOptionalField),

    DateTime        (DateTimeField),
    DateTimeList    (DateTimeListField),
    DateTimeOptional(DateTimeOptionalField),

    Bytes           (BytesField),
    BytesList       (BytesListField),
    BytesOptional   (BytesOptionalField),
    
    Json            (JsonField),

    Relation        (Relation),
}



struct JsonField  {}
struct BytesField {
    default: Option<String>,
}
struct EnumField {
    default: Option<String>,
}


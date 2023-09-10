use crate::*;

mod string;
mod boolean;
mod int;
mod big_int;
mod float;
mod decimal;
mod enums;
mod date_time;
mod bytes;

pub mod field_type {
    pub use super::string::*;
    pub use super::boolean::*;
    pub use super::int::*;
    pub use super::big_int::*;
    pub use super::float::*;
    pub use super::decimal::*;
    pub use super::enums::*;
    pub use super::date_time::*;
    pub use super::bytes::*;
}


pub struct Model {
    pub name:    String,
    pub fields:  Vec<Field>,

    pub map:     Option<String>,
    pub ids:     Vec<Vec<String>>,
    pub uniques: Vec<Vec<String>>,
    pub indexes: Vec<Vec<String>>,
} impl Parse for Model {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut M = Self {
            name:    String::new(),
            fields:  Vec::new(),
            map:     None,
            ids:     Vec::new(),
            uniques: Vec::new(),
            indexes: Vec::new(),
        };

        ts.try_consume(Token::Keyword(Keyword::_model))?;
        M.name = ts.try_pop_ident()?;

        ts.try_consume(Token::BraceOpen)?;
        loop {
            let (loc, next) = ts.try_peek()?;
            if next == &Token::BraceClose {break}

            match next {
                Token::At2 => {ts.pop();
                    match &*ts.try_pop_ident()? {
                        "map" => {
                            todo!()
                        }
                        "id" => {
                            todo!()
                        }
                        "unique" => {
                            todo!()
                        }
                        "index" => {
                            todo!()
                        }
                        other => return Err(ts.current.Msg(f!("Expected one of `map`, `id`, `unique`, `index` but found `{other}`")))
                    }
                }
                Token::Ident(_) => {
                    M.fields.push(Field::parse(ts)?)
                }
                other => return Err(loc.Msg(f!("Expected an identifier or `@@` but found `{other}`")))
            }
        }
        ts.try_consume(Token::BraceOpen)?;

        Ok(M)
    }
}

pub struct Field {
    pub name:          String,
    pub schema:        FieldSchema,

    pub map:           Option<String>,
    pub id:            Option<()>,
    pub unique:        Option<()>,
    pub relation:      Option<Relation>,
} impl Parse for Field {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        
    }
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

    Enum            (enums::EnumField),
    EnumList        (enums::EnumListField),
    EnumOptional    (enums::EnumOptionalField),

    DateTime        (date_time::DateTimeField),
    DateTimeList    (date_time::DateTimeListField),
    DateTimeOptional(date_time::DateTimeOptionalField),

    Bytes           (bytes::BytesField),
    BytesList       (bytes::BytesListField),
    BytesOptional   (bytes::BytesOptionalField),

    Model           { model_name: String, relation: Option<Relation> },
    ModelList       { model_name: String, relation: Option<Relation> },
    ModelOptional   { model_name: String, relation: Option<Relation> },
    
    Json,
}
pub struct Relation {
    pub fields:     Vec<String>,
    pub references: Vec<String>,
}

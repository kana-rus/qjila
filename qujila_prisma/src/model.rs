mod string;
mod boolean;
mod int;
mod big_int;
mod float;
mod decimal;
mod enums;
mod date_time;
mod bytes;

use crate::*;
use {string::*, boolean::*, int::*, big_int::*, float::*, decimal::*, enums::*, date_time::*, bytes::*};


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
    pub name:      String,
    pub schema:    FieldSchema,
} impl Parse for Field {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        Ok(Self {
            name:   ts.try_pop_ident()?,
            schema: FieldSchema::parse(ts)?,
        })
    }
}

pub enum FieldSchema {
    String         (StringAttributes),
    StringList     (StringListAttributes),
    StringOptional (StringOptionalAttributes),

    Boolean         (BooleanAttributes),
    BooleanList     (BooleanListAttributes),
    BooleanOptional (BooleanOptionalAttributes),

    Int             (IntAttributes),
    IntList         (IntListAttributes),
    IntOptional     (IntOptionalAttributes),

    BigInt          (BigIntAttributes),
    BigIntList      (BigIntListAttributes),
    BigIntOptional  (BigIntOptionalAttributes),

    Float           (FloatAttributes),
    FloatList       (FloatListAttributes),
    FloatOptional   (FloatOptionalAttributes),

    Decimal         (DecimalAttributes),
    DecimalList     (DecimalListAttributes),
    DecimalOptional (DecimalOptionalAttributes),

    Enum            (EnumAttributes),
    EnumList        (EnumListAttributes),
    EnumOptional    (EnumOptionalAttributes),

    DateTime        (DateTimeAttributes),
    DateTimeList    (DateTimeListAttributes),
    DateTimeOptional(DateTimeOptionalAttributes),

    Bytes           (BytesAttributes),
    BytesList       (BytesListAttributes),
    BytesOptional   (BytesOptionalAttributes),

    Model           { model_name: String, relation: Option<Relation> },
    ModelList       { model_name: String, relation: Option<Relation> },
    ModelOptional   { model_name: String, relation: Option<Relation> },
}

pub struct Relation {
    pub fields:     Vec<String>,
    pub references: Vec<String>,
} impl Parse for Relation {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut R = Relation {
            fields:     Vec::new(),
            references: Vec::new(),
        };

        ts.try_consume(Token::At)?;
        ts.try_consume_ident("relation")?;

        ts.try_consume(Token::ParenOpen)?;
        while let Ok(i) = ts.try_pop_ident() {
            match &*i {
                "fields" => {
                    ts.try_consume(Token::Colon)?;
                    ts.try_consume(Token::BracketOpen)?;
                    while let Ok(field) = ts.try_pop_ident() {
                        R.fields.push(field);
                        ts.try_consume(Token::Comma).ok();
                    }
                    ts.try_consume(Token::BracketClose)?;
                    ts.try_consume(Token::Comma).ok();
                }
                "references" => {
                    ts.try_consume(Token::Colon)?;
                    ts.try_consume(Token::BracketOpen)?;
                    while let Ok(field) = ts.try_pop_ident() {
                        R.references.push(field);
                        ts.try_consume(Token::Comma).ok();
                    }
                    ts.try_consume(Token::BracketClose)?;
                    ts.try_consume(Token::Comma).ok();}
                other => return Err(ts.current.Msg(f!("Expected one of `fields`, `references` but found `{other}`")))
            }
        }
        ts.try_consume(Token::ParenClose)?;

        Ok(R)
    }
}

impl Parse for FieldSchema {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        match &*ts.try_pop_ident()? {
            "String" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::StringList(StringListAttributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::StringOptional(StringOptionalAttributes::parse(ts)?))
                }
                _ => Ok(Self::String(StringAttributes::parse(ts)?))
            }
            "Boolean" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::BooleanList(BooleanListAttributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::BooleanOptional(BooleanOptionalAttributes::parse(ts)?))
                }
                _ => Ok(Self::Boolean(BooleanAttributes::parse(ts)?))
            }
            "Int" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::IntList(IntListAttributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::IntOptional(IntOptionalAttributes::parse(ts)?))
                }
                _ => Ok(Self::Int(IntAttributes::parse(ts)?))
            }
            "BigInt" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::BigIntList(BigIntListAttributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::BigIntOptional(BigIntOptionalAttributes::parse(ts)?))
                }
                _ => Ok(Self::BigInt(BigIntAttributes::parse(ts)?))
            }
            "Float" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::FloatList(FloatListAttributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::FloatOptional(FloatOptionalAttributes::parse(ts)?))
                }
                _ => Ok(Self::Float(FloatAttributes::parse(ts)?))
            }
            "Decimal" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::DecimalList(DecimalListAttributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::DecimalOptional(DecimalOptionalAttributes::parse(ts)?))
                }
                _ => Ok(Self::Decimal(DecimalAttributes::parse(ts)?))
            }
            "Enum" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::EnumList(EnumListAttributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::EnumOptional(EnumOptionalAttributes::parse(ts)?))
                }
                _ => Ok(Self::Enum(EnumAttributes::parse(ts)?))
            }
            "DateTime" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::DateTimeList(DateTimeListAttributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::DateTimeOptional(DateTimeOptionalAttributes::parse(ts)?))
                }
                _ => Ok(Self::DateTime(DateTimeAttributes::parse(ts)?))
            }
            "Bytes" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::BytesList(BytesListAttributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::BytesOptional(BytesOptionalAttributes::parse(ts)?))
                }
                _ => Ok(Self::Bytes(BytesAttributes::parse(ts)?))
            }
            model => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::ModelList {
                        model_name: model.to_string(),
                        relation:   ts.next_is(Token::At).then_some(Relation::parse(ts)?),
                    })
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::ModelOptional {
                        model_name: model.to_string(),
                        relation:   ts.next_is(Token::At).then_some(Relation::parse(ts)?),
                    })
                }
                _ => Ok(Self::Model {
                    model_name: model.to_string(),
                    relation:   ts.next_is(Token::At).then_some(Relation::parse(ts)?),
                })
            }
        }
    }
}

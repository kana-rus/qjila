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
        use {string::*, boolean::*, int::*, big_int::*, float::*, decimal::*, enums::*, date_time::*, bytes::*};

        match &*ts.try_pop_ident()? {
            "String" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::StringList(StringListField::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::StringOptional(StringOptionalField::parse(ts)?))
                }
                _ => Ok(Self::String(StringField::parse(ts)?))
            }
            "Boolean" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::BooleanList(BooleanListField::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::BooleanOptional(BooleanOptionalField::parse(ts)?))
                }
                _ => Ok(Self::Boolean(BooleanField::parse(ts)?))
            }
            "Int" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::IntList(IntListField::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::IntOptional(IntOptionalField::parse(ts)?))
                }
                _ => Ok(Self::Int(IntField::parse(ts)?))
            }
            "BigInt" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::BigIntList(BigIntListField::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::BigIntOptional(BigIntOptionalField::parse(ts)?))
                }
                _ => Ok(Self::BigInt(BigIntField::parse(ts)?))
            }
            "Float" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::FloatList(FloatListField::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::FloatOptional(FloatOptionalField::parse(ts)?))
                }
                _ => Ok(Self::Float(FloatField::parse(ts)?))
            }
            "Decimal" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::DecimalList(DecimalListField::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::DecimalOptional(DecimalOptionalField::parse(ts)?))
                }
                _ => Ok(Self::Decimal(DecimalField::parse(ts)?))
            }
            "Enum" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::EnumList(EnumListField::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::EnumOptional(EnumOptionalField::parse(ts)?))
                }
                _ => Ok(Self::Enum(EnumField::parse(ts)?))
            }
            "DateTime" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::DateTimeList(DateTimeListField::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::DateTimeOptional(DateTimeOptionalField::parse(ts)?))
                }
                _ => Ok(Self::DateTime(DateTimeField::parse(ts)?))
            }
            "Bytes" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::BytesList(BytesListField::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::BytesOptional(BytesOptionalField::parse(ts)?))
                }
                _ => Ok(Self::Bytes(BytesField::parse(ts)?))
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

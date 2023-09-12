use crate::*;


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
    String         (AttributesWithDefault<StringValue>),
    StringOptional (AttributesWithDefault<StringValue>),
    StringList     (AttributesWithDefault<Vec<StringValue>>),

    Boolean         (BooleanAttributes),
    BooleanList     (BooleanListAttributes),
    BooleanOptional (BooleanOptionalAttributes),

    Int             (AttributesWithDefault<IntValue>),
    IntOptional     (AttributesWithDefault<IntValue>),
    IntList         (AttributesWithDefault<Vec<IntValue>>),

    BigInt          (AttributesWithDefault<BigIntValue>),
    BigIntOptional  (AttributesWithDefault<BigIntValue>),
    BigIntList      (AttributesWithDefault<Vec<BigIntValue>>),

    Float           (FloatAttributes),
    FloatList       (FloatListAttributes),
    FloatOptional   (FloatOptionalAttributes),

    Decimal         (DecimalAttributes),
    DecimalList     (DecimalListAttributes),
    DecimalOptional (DecimalOptionalAttributes),

    DateTime        (AttributesWithDefault<DateTimeValue>),
    DateTimeOptional(AttributesWithDefault<DateTimeValue>),
    DateTimeList    (AttributesWithDefault<Vec<DateTimeValue>>),

    Bytes           (BytesAttributes),
    BytesList       (BytesListAttributes),
    BytesOptional   (BytesOptionalAttributes),

    Model           { model_name: String, relation: Option<Relation> },
    ModelList       { model_name: String, relation: Option<Relation> },
    ModelOptional   { model_name: String, relation: Option<Relation> },
}

pub struct Attributes {
    pub id:        bool,
    pub unique:    bool,
    pub map:       Option<String>,
} impl Parse for Attributes {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut A = Attributes {
            id:      false,
            unique:  false,
            map:     None,
        };

        while ts.try_consume(Token::At).is_ok() {
            match &*ts.try_pop_ident()? {
                "id"      => A.id     = true,
                "unique"  => A.unique = true,
                "map"     => {
                    ts.try_consume(Token::ParenOpen)?;
                    let map_to = ts.try_pop_string_literal()?;
                    ts.try_consume(Token::ParenClose)?;

                    if A.map.is_some_and(|s| s == map_to) {
                        return Err(ts.current.Msg("Duplicate declaring `map` attributes"))
                    }
                    A.map = Some(map_to)
                }
                other => return Err(ts.current.Msg(f!("Expected one of `id`, `unique`, `map` but found `{other}`")))
            }
        }

        Ok(A)
    }
}

pub struct AttributesWithDefault<T: Parse> {
    pub id:        bool,
    pub unique:    bool,
    pub map:       Option<String>,
    pub default:   Option<T>,
} impl<T: Parse> Parse for AttributesWithDefault<T> {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut A = AttributesWithDefault {
            id:      false,
            unique:  false,
            map:     None,
            default: None,
        };

        while ts.try_consume(Token::At).is_ok() {
            match &*ts.try_pop_ident()? {
                "id"      => A.id     = true,
                "unique"  => A.unique = true,
                "map"     => {
                    ts.try_consume(Token::ParenOpen)?;
                    let map_to = ts.try_pop_string_literal()?;
                    ts.try_consume(Token::ParenClose)?;

                    if A.map.is_some_and(|s| s == map_to) {
                        return Err(ts.current.Msg("Duplicate declaring `map` attributes"))
                    }
                    A.map = Some(map_to)
                }
                "default" => {
                    ts.try_consume(Token::ParenOpen)?;
                    A.default = Some(T::parse(ts)?);
                    ts.try_consume(Token::ParenClose)?;
                }
                other => return Err(ts.current.Msg(f!("Expected one of `id`, `unique`, `map`, `default` but found `{other}`")))
            }
        }

        Ok(A)
    }
}

pub enum StringValue {
    value(String),
    cuid,
    uuid,
} impl Parse for StringValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        if let Ok(value) = ts.try_pop_ident() {
            Ok(Self::value(value))
        } else {
            let function = match &*ts.try_pop_ident()? {
                "cuid" => Self::cuid,
                "uuid" => Self::uuid,
                other  => return Err(ts.current.Msg(f!("Expected string literal or `cuid()`, `uuid()` buf found `{other}`")))
            };
            ts.try_consume(Token::ParenOpen)?;
            ts.try_consume(Token::ParenClose)?;
            Ok(function)
        }
    }
}

pub enum IntValue {
    value(i32),
    autoincrement,
} impl Parse for IntValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        if let Ok(value) = ts.try_pop_integer_litreral() {
            Ok(Self::value(
                value.try_into().map_err(|e| ts.current.Msg(f!("{value} is not `Int`: {e}")))?
            ))
        } else {
            let function = match &*ts.try_pop_ident()? {
                "autoincrement" => Self::autoincrement,
                other => return Err(ts.current.Msg(f!("Expected `autoincrement` buf founf `{other}`")))
            };
            ts.try_consume(Token::ParenOpen)?;
            ts.try_consume(Token::ParenClose)?;
            Ok(function)
        }
    }
}

pub enum BigIntValue {
    value(i64),
    autoincrement,
} impl Parse for BigIntValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        if let Ok(value) = ts.try_pop_integer_litreral() {
            Ok(Self::value(
                value.try_into().map_err(|e| ts.current.Msg(f!("{value} is not `BigInt`: {e}")))?
            ))
        } else {
            let function = match &*ts.try_pop_ident()? {
                "autoincrement" => Self::autoincrement,
                other => return Err(ts.current.Msg(f!("Expected `autoincrement` buf founf `{other}`")))
            };
            ts.try_consume(Token::ParenOpen)?;
            ts.try_consume(Token::ParenClose)?;
            Ok(function)
        }
    }
}

pub enum DateTimeValue {
    now,
    updatedAt,
} impl Parse for DateTimeValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let function = match &*ts.try_pop_ident()? {
            "now"       => Self::now,
            "updatedAt" => Self::updatedAt,
            other => return Err(ts.current.Msg(f!("Expected one of `now`, `updatedAt` but found `{other}`")))
        };
        ts.try_consume(Token::ParenOpen)?;
        ts.try_consume(Token::ParenClose)?;
        Ok(function)
    }
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

use crate::*;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Model {pub doc_comment: Option<String>,
    pub name:    String,
    pub fields:  Vec<Field>,

    pub map:     Option<String>,
    pub ids:     Vec<Vec<String>>,
    pub uniques: Vec<Vec<String>>,
    pub indexes: Vec<Vec<String>>,
} impl Parse for Model {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut M = Self {doc_comment: ts.pop_doc_comment(),
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
                            ts.try_consume(Token::ParenOpen)?;
                            let map_to = ts.try_pop_string_literal()?;
                            if M.map.as_ref().is_some_and(|m| m == &map_to) {
                                return Err(ts.current.Msg("Duplicate declaring `map`"))
                            }
                            ts.try_consume(Token::ParenClose)?;

                            M.map = Some(map_to)
                        }
                        "id" => {
                            ts.try_consume(Token::ParenOpen)?;
                            ts.try_consume(Token::BracketOpen)?;
                            let id = ts.parse_csv(TokenStream::try_pop_ident)?;
                            ts.try_consume(Token::BracketClose)?;
                            ts.try_consume(Token::ParenClose)?;

                            M.ids.push(id)
                        }
                        "unique" => {
                            ts.try_consume(Token::ParenOpen)?;
                            ts.try_consume(Token::BracketOpen)?;
                            let unique = ts.parse_csv(TokenStream::try_pop_ident)?;
                            ts.try_consume(Token::BracketClose)?;
                            ts.try_consume(Token::ParenClose)?;

                            M.uniques.push(unique)
                        }
                        "index" => {
                            ts.try_consume(Token::ParenOpen)?;
                            ts.try_consume(Token::BracketOpen)?;
                            let index = ts.parse_csv(TokenStream::try_pop_ident)?;
                            ts.try_consume(Token::BracketClose)?;
                            ts.try_consume(Token::ParenClose)?;

                            M.indexes.push(index)
                        }
                        other => return Err(ts.current.Msg(f!("Expected one of `map`, `id`, `unique`, `index` but found `{other}`")))
                    }
                }
                Token::Ident(_) | Token::DocComment(_) => {
                    M.fields.push(Field::parse(ts)?)
                }
                other => return Err(loc.Msg(f!("Expected an identifier or `@@` but found `{other}`")))
            }
        }
        ts.try_consume(Token::BraceClose)?;

        Ok(M)
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Field {pub doc_comment: Option<String>,
    pub name:      String,
    pub schema:    FieldSchema,
} impl Parse for Field {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let leading_comment = ts.pop_doc_comment();
        let name = ts.try_pop_ident()?;
        let schema = FieldSchema::parse(ts)?;
        let trailing_comment = ts.pop_doc_comment_on_current_line();

        let doc_comment = match (leading_comment, trailing_comment) {
            (Some(l), Some(t)) => Some(f!("{l}\n\n{t}")),
            (Some(l), None   ) => Some(l),
            (None,    Some(t)) => Some(t),
            (None,    None   ) => None,
        };

        Ok(Self {doc_comment, name, schema })
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum FieldSchema {
    String         (Attributes<StringValue>),
    StringOptional (Attributes<StringValue>),
    StringList     (Attributes<Vec<StringValue>>),

    Boolean         (Attributes<BooleanValue>),
    BooleanOptional (Attributes<BooleanValue>),
    BooleanList     (Attributes<Vec<BooleanValue>>),

    Int             (Attributes<IntValue>),
    IntOptional     (Attributes<IntValue>),
    IntList         (Attributes<Vec<IntValue>>),

    BigInt          (Attributes<BigIntValue>),
    BigIntOptional  (Attributes<BigIntValue>),
    BigIntList      (Attributes<Vec<BigIntValue>>),

    Float           (Attributes<FloatValue>),
    FloatOptional   (Attributes<FloatValue>),
    FloatList       (Attributes<Vec<FloatValue>>),

    DateTime        (Attributes<DateTimeValue>),
    DateTimeList    (Attributes<Vec<DateTimeValue>>),
    DateTimeOptional(Attributes<DateTimeValue>),

    Bytes           (Attributes<BytesValue>),
    BytesOptional   (Attributes<BytesValue>),
    BytesList       (Attributes<Vec<BytesValue>>),

    Model           { model_name: String, relation: Option<Relation> },
    ModelList       { model_name: String, relation: Option<Relation> },
    ModelOptional   { model_name: String, relation: Option<Relation> },
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Attributes<T: Parse + PartialEq> {
    pub id:        bool,
    pub unique:    bool,
    pub map:       Option<String>,
    pub default:   Option<T>,
} impl<T: Parse + PartialEq> Parse for Attributes<T> {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut A = Attributes {
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

                    if A.map.is_some_and(|m| m == map_to) {
                        return Err(ts.current.Msg("Duplicate declaring `map`s"))
                    }
                    A.map = Some(map_to)
                }
                "default" => {
                    ts.try_consume(Token::ParenOpen)?;
                    let default_value = T::parse(ts)?;
                    ts.try_consume(Token::ParenClose)?;

                    if A.default.is_some_and(|d| d == default_value) {
                        return Err(ts.current.Msg("Duplicate declaring `default`s"))
                    }
                    A.default = Some(default_value)
                }
                other => return Err(ts.current.Msg(f!("Expected one of `id`, `unique`, `map`, `default` but found `{other}`")))
            }
        }

        Ok(A)
    }
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum StringValue {
    literal(String),
    cuid,
    uuid,
} impl Parse for StringValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        if let Ok(value) = ts.try_pop_ident() {
            Ok(Self::literal(value))
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

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum BooleanValue {
    literal(bool)
} impl Parse for BooleanValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        Ok(Self::literal(
            ts.try_pop_boolean_literal()?
        ))
    }
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum IntValue {
    literal(i32),
    autoincrement,
} impl Parse for IntValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        if let Ok(value) = ts.try_pop_integer_litreral() {
            Ok(Self::literal(
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

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum BigIntValue {
    literal(i64),
    autoincrement,
} impl Parse for BigIntValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        if let Ok(value) = ts.try_pop_integer_litreral() {
            Ok(Self::literal(
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

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum FloatValue {
    literal(f64)
} impl Parse for FloatValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        Ok(Self::literal(
            ts.try_pop_decimal_literal()?
        ))
    }
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
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

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum BytesValue {
    literal(String)
} impl Parse for BytesValue {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        Ok(Self::literal(
            ts.try_pop_string_literal()?
        ))
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
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
                    R.fields = ts.parse_csv(TokenStream::try_pop_ident)?;
                    ts.try_consume(Token::BracketClose)?;
                    { ts.try_consume(Token::Comma).ok(); }
                }
                "references" => {
                    ts.try_consume(Token::Colon)?;
                    ts.try_consume(Token::BracketOpen)?;
                    R.references = ts.parse_csv(TokenStream::try_pop_ident)?;
                    ts.try_consume(Token::BracketClose)?;
                    { ts.try_consume(Token::Comma).ok(); }
                }
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
                    Ok(Self::StringList(Attributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::StringOptional(Attributes::parse(ts)?))
                }
                _ => Ok(Self::String(Attributes::parse(ts)?))
            }
            "Boolean" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::BooleanList(Attributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::BooleanOptional(Attributes::parse(ts)?))
                }
                _ => Ok(Self::Boolean(Attributes::parse(ts)?))
            }
            "Int" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::IntList(Attributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::IntOptional(Attributes::parse(ts)?))
                }
                _ => Ok(Self::Int(Attributes::parse(ts)?))
            }
            "BigInt" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::BigIntList(Attributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::BigIntOptional(Attributes::parse(ts)?))
                }
                _ => Ok(Self::BigInt(Attributes::parse(ts)?))
            }
            "Float" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::FloatList(Attributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::FloatOptional(Attributes::parse(ts)?))
                }
                _ => Ok(Self::Float(Attributes::parse(ts)?))
            }
            "DateTime" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::DateTimeList(Attributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::DateTimeOptional(Attributes::parse(ts)?))
                }
                _ => Ok(Self::DateTime(Attributes::parse(ts)?))
            }
            "Bytes" => match &ts.try_peek()?.1 {
                Token::BracketOpen => {
                    ts.try_consume(Token::BracketOpen)?;
                    ts.try_consume(Token::BracketClose)?;
                    Ok(Self::BytesList(Attributes::parse(ts)?))
                }
                Token::Question => {
                    ts.try_consume(Token::Question)?;
                    Ok(Self::BytesOptional(Attributes::parse(ts)?))
                }
                _ => Ok(Self::Bytes(Attributes::parse(ts)?))
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

use crate::*;


pub struct StringAttributes {
    pub id:        bool,
    pub unique:    bool,
    pub map:       Option<String>,
    pub default:   Option<StringValue>,
} impl Parse for StringAttributes {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut SA = StringAttributes {
            id:      false,
            unique:  false,
            map:     None,
            default: None,
        };

        while ts.try_consume(Token::At).is_ok() {
            match &*ts.try_pop_ident()? {
                "id"      => SA.id     = true,
                "unique"  => SA.unique = true,
                "map"     => {
                    ts.try_consume(Token::ParenOpen)?;
                    SA.map = Some(ts.try_pop_string_literal()?);
                    ts.try_consume(Token::ParenClose)?;
                }
                "default" => {
                    ts.try_consume(Token::ParenOpen)?;
                    SA.default = Some(StringValue::parse(ts)?);
                    ts.try_consume(Token::ParenClose)?;
                }
                other => return Err(ts.current.Msg(f!("Expected one of `id`, `unique`, `map`, `default` but found `{other}`")))
            }
        }

        Ok(SA)
    }
}

pub struct StringListAttributes {
    pub id:        bool,
    pub unique:    bool,
    pub map:       Option<String>,
    pub default:   Option<Vec<StringValue>>,
} impl Parse for StringListAttributes {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut SLA = StringListAttributes {
            id:      false,
            unique:  false,
            map:     None,
            default: None,
        };

        while ts.try_consume(Token::At).is_ok() {
            match &*ts.try_pop_ident()? {
                "id"      => SLA.id     = true,
                "unique"  => SLA.unique = true,
                "map"     => {
                    ts.try_consume(Token::ParenOpen)?;
                    SLA.map = Some(ts.try_pop_string_literal()?);
                    ts.try_consume(Token::ParenClose)?;
                }
                "default" => {
                    ts.try_consume(Token::ParenOpen)?;
                    SLA.default = Some(ts.parse_csv(|__ts| StringValue::parse(__ts))?);
                    ts.try_consume(Token::ParenClose)?;
                }
                other => return Err(ts.current.Msg(f!("Expected one of `id`, `unique`, `map`, `default` but found `{other}`")))
            }
        }

        Ok(SLA)
    }
}

pub struct StringOptionalAttributes {
    pub id:        bool,
    pub unique:    bool,
    pub map:       Option<String>,
    pub default:   Option<StringValue>,
} impl Parse for StringOptionalAttributes {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut SOA = StringOptionalAttributes {
            id:      false,
            unique:  false,
            map:     None,
            default: None,
        };

        while ts.try_consume(Token::At).is_ok() {
            match &*ts.try_pop_ident()? {
                "id"      => SOA.id     = true,
                "unique"  => SOA.unique = true,
                "map"     => {
                    ts.try_consume(Token::ParenOpen)?;
                    SOA.map = Some(ts.try_pop_string_literal()?);
                    ts.try_consume(Token::ParenClose)?;
                }
                "default" => {
                    ts.try_consume(Token::ParenOpen)?;
                    SOA.default = Some(StringValue::parse(ts)?);
                    ts.try_consume(Token::ParenClose)?;
                }
                other => return Err(ts.current.Msg(f!("Expected one of `id`, `unique`, `map`, `default` but found `{other}`")))
            }
        }

        Ok(SOA)
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
            match &*ts.try_pop_ident()? {
                "cuid" => {
                    ts.try_consume(Token::ParenOpen)?;
                    ts.try_consume(Token::ParenClose)?;
                    Ok(Self::cuid)
                }
                "uuid" => {
                    ts.try_consume(Token::ParenOpen)?;
                    ts.try_consume(Token::ParenClose)?;
                    Ok(Self::uuid)
                }
                other  => Err(ts.current.Msg(f!("Expected string literal or `cuid()`, `uuid()` buf found `{other}`")))
            }
        }
    }
}

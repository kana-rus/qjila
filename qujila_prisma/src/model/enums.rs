use crate::*;


pub struct EnumAttributes {
    pub id:        bool,
    pub unique:    bool,
    pub map:       Option<String>,
    pub default:   Option<String>,
} impl Parse for EnumAttributes {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut EA = EnumAttributes {
            id:      false,
            unique:  false,
            map:     None,
            default: None,
        };

        while ts.try_consume(Token::At).is_ok() {
            match &*ts.try_pop_ident()? {
                "id"      => EA.id     = true,
                "unique"  => EA.unique = true,
                "map"     => {
                    ts.try_consume(Token::ParenOpen)?;
                    EA.map = Some(ts.try_pop_string_literal()?);
                    ts.try_consume(Token::ParenClose)?;
                }
                "default" => {
                    ts.try_consume(Token::ParenOpen)?;
                    EA.default = Some(ts.try_pop_ident()?);
                    ts.try_consume(Token::ParenClose)?;
                }
                other => return Err(ts.current.Msg(f!("Expected one of `id`, `unique`, `map`, `default` but found `{other}`")))
            }
        }

        Ok(EA)
    }
}

pub struct EnumListAttributes {
    pub id:        bool,
    pub unique:    bool,
    pub map:       Option<String>,
    pub default:   Option<Vec<String>>,
} impl Parse for EnumListAttributes {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut ELA = EnumListAttributes {
            id:      false,
            unique:  false,
            map:     None,
            default: None,
        };

        while ts.try_consume(Token::At).is_ok() {
            match &*ts.try_pop_ident()? {
                "id"      => ELA.id     = true,
                "unique"  => ELA.unique = true,
                "map"     => {
                    ts.try_consume(Token::ParenOpen)?;
                    ELA.map = Some(ts.try_pop_string_literal()?);
                    ts.try_consume(Token::ParenClose)?;
                }
                "default" => {
                    ts.try_consume(Token::ParenOpen)?;
                    ts.try_consume(Token::BracketOpen)?;
                    ELA.default = Some(ts.try_pop_csv_idents()?);
                    ts.try_consume(Token::BracketClose)?;
                    ts.try_consume(Token::ParenClose)?;
                }
                other => return Err(ts.current.Msg(f!("Expected one of `id`, `unique`, `map`, `default` but found `{other}`")))
            }
        }

        Ok(ELA)
    }
}

pub struct EnumOptionalAttributes {
    pub id:        bool,
    pub unique:    bool,
    pub map:       Option<String>,
    pub default:   Option<String>,
} impl Parse for EnumOptionalAttributes {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let mut EOA = EnumOptionalAttributes {
            id:      false,
            unique:  false,
            map:     None,
            default: None,
        };

        while ts.try_consume(Token::At).is_ok() {
            match &*ts.try_pop_ident()? {
                "id"      => EOA.id     = true,
                "unique"  => EOA.unique = true,
                "map"     => {
                    ts.try_consume(Token::ParenOpen)?;
                    EOA.map = Some(ts.try_pop_string_literal()?);
                    ts.try_consume(Token::ParenClose)?;
                }
                "default" => {
                    ts.try_consume(Token::ParenOpen)?;
                    EOA.default = Some(ts.try_pop_ident()?);
                    ts.try_consume(Token::ParenClose)?;
                }
                other => return Err(ts.current.Msg(f!("Expected one of `id`, `unique`, `map`, `default` but found `{other}`")))
            }
        }

        Ok(EOA)
    }
}

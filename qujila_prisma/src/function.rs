use crate::*;


pub trait Function<Output> {
    fn eval(self) -> Output;
}

pub struct env {
    pub(crate) loc: Location,
    pub(crate) var: String,
}
impl Parse for env {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        let loc = ts.try_consume_ident("env")?;
        ts.try_consume(Token::ParenOpen)?;
        let var = ts.try_pop_string_literal()?;
        ts.try_consume(Token::ParenClose)?;

        Ok(Self { loc, var })
    }
}
impl Function<Result<String, Cow<'static, str>>> for env {
    fn eval(self) -> Result<String, Cow<'static, str>> {
        let key = self.var;
        std::env::var(&key)
            .map_err(|e| self.loc.Msg(f!("Failed to fetch environment variable `{key}`: {e}")))
    }
}

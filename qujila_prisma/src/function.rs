use crate::*;


pub trait Function<Output> {
    fn eval(self) -> Output;
}

pub struct env {
    environment_variable_name: String,
}
impl Parse for env {
    fn parse(ts: &mut TokenStream) -> Result<Self, std::borrow::Cow<'static, str>> {
        ts.try_consume_ident("env")?;
        ts.try_consume(Token::ParenOpen)?;
        let Lit::Str(s) = ts.try_pop_literal()?
            else {return Err(ts.current.Msg("Expected a string literal"))};
        ts.try_consume(Token::ParenClose)?;

        Ok(Self { environment_variable_name: s.to_string() })
    }
}
impl Function<Result<String, Cow<'static, str>>> for env {
    fn eval(self) -> Result<String, Cow<'static, str>> {
        let key = self.environment_variable_name;
        std::env::var(&key)
            .map_err(|e| Cow::Owned(f!("Failed to fetch environment variable `{key}`: {e}")))
    }
}

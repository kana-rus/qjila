use proc_macro2::{TokenStream, Ident};
use quote::quote;
use syn::{Result, Type, ExprMacro, ExprArray, parse::Parse, token, parse2};


struct IntoQuery {
    query_name:  Ident,
    _arrow:      token::RArrow,
    result_type: Type,
    _colon:      token::Colon,

    /// expects `format!(/* ... */)`
    sql_expr:    ExprMacro,
    params:      Option<ExprArray>,
} impl Parse for IntoQuery {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        Ok(Self {
            query_name:  input.parse()?,
            _arrow:      input.parse()?,
            result_type: input.parse()?,
            _colon:      input.parse()?,

            sql_expr: input.parse()?,
            params:   input.peek(token::Bracket).then_some(input.parse()?),
        })
    }
}

pub(crate) fn into_query(stream: TokenStream) -> Result<TokenStream> {
    let input: IntoQuery = parse2(stream)?;

    Ok(quote!{
        
    })
}

/*



*/

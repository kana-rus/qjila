use proc_macro::TokenStream;

mod internal; use internal::{into_query};

#[proc_macro]
pub fn __internal__into_query(stream: TokenStream) -> TokenStream {
    into_query(stream.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

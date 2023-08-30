mod schema;
/// <br/>
/// 
/// ```ignore
/// qujila::schema! {
///     User {
///         id         usize.auto_increment();
///         name       String;
///         password   String;
///         profile    String;
///         created_at DateTime.default_now();
///         updated_at DateTime;
///     }
/// }
/// ```
#[proc_macro]
pub fn schema(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    schema::schema(input.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

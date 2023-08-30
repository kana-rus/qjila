use proc_macro2::Ident;
use syn::{parse::Parse, braced, token};


pub(super) struct Schema {
    name:   Ident,
    fields: Vec<Field>,
} impl Parse for Schema {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        let mut fields = Vec::new();
        let fields_buf; braced!(fields_buf in input);
        while !fields_buf.is_empty() {
            fields.push(fields_buf.parse()?)
        }

        Ok(Self { name, fields })
    }
}

pub(super) struct Field {
    name:        Ident,
    db_type:     Ident,
    constraints: Vec<Ident>,
} impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name    = input.parse()?;

        let db_type = input.parse()?;

        let mut constraints = Vec::new();
        while !input.peek(token::Semi) {
            let _: token::Dot = input.parse()?;
            let constraint    = input.parse()?;
            constraints.push(constraint)
        }

        Ok(Self { name, db_type, constraints })
    }
}

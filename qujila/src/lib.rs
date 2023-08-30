#![allow(non_snake_case, non_camel_case_types)]

#[doc(hidden)] pub mod __internal {
    pub mod db_type_label {
        pub struct usize;
        impl usize {
            pub const fn auto_increment(self) -> Self {self}
        }

        pub struct String;
        impl String {
            //
        }

        pub struct DateTime;
        impl DateTime {
            pub const fn default_now(self) -> Self {self}
        }
    }
}

/*

qujila::schema! {
    User {
        id          usize.auto_increment();
        name        String;
        password    String;
        profile     String;
        created_at  DateTime.default_now();
        updated_at  DateTime;
    }
}

*/

/*
/// ```ignore
/// schema! {
///     User {
///         id          usize.auto_increment();
///         name        String;
///         password    String;
///         profile     String;
///         created_at  DateTime.default_now();
///         updated_at  DateTime;
///     }
/// }
/// ```
macro_rules! schema {
    (
        $(
            $model:ident $columns:block
        )*
    ) => {
        #[allow(unused, non_upper_case_globals)] const _: () = {
            // use ::qujila::__internal::db_type_label::*;
            use crate::__internal::db_type_label::*;
            $(
                const $model: () = $columns;
            )*
        };
    };
} const _: () = {
    schema! {
        User {
            let id = usize.auto_increment();
        }
    }
};
*/

/// ```ignore
/// schema! {
///     mod User {
///         const id = ;
///     }
/// }
/// ```
macro_rules! schema {
    (
        $(
            mod $model:ident {
                $(
                    const $column:ident = $db_type:expr
                );* $(;)?
            }
        )*
    ) => {
        #[allow(unused, non_snake_case, non_upper_case_globals)]
        const _: () = {
            $(
                mod $model {
                    use crate::__internal::db_type_label::*;
                    $(
                        const $column: usize = $db_type
                    );* ;
                }
            )*
        };
    };
} const _: () = {
    schema! {
        mod User {
            const id = usize.auto_increment()
        }
    }
};

#[allow(unused, non_upper_case_globals)] const _: () = {
    use crate::__internal::db_type_label::*;
    
    const User: () = {
        let id         = usize.auto_increment();
        let name       = String;
        let password   = String;
        let profile    = String;
        let created_at = DateTime.default_now();
        let updated_at = DateTime;
    };
};

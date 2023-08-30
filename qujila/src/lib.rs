#![allow(non_snake_case, non_camel_case_types)]

#[doc(hidden)]
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

#[allow(unused, non_upper_case_globals)] const _: () = {
    use crate::db_type_label::*;
    
    const User: () = {
        let id         = usize.auto_increment();
        let name       = String;
        let password   = String;
        let profile    = String;
        let created_at = DateTime.default_now();
        let updated_at = DateTime;
    };
};

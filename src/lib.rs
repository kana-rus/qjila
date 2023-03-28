#![feature(array_methods)]

#![allow(incomplete_features)]
#![feature(adt_const_params)]

#![allow(non_snake_case, non_camel_case_types)]

mod pool;
mod error;
mod query;
mod entity;
mod db_type;
mod condition;
mod connection;


pub(crate) mod internal_macros {
    pub(crate) use qujila_macros::{
        __internal__into_query,
    };
}

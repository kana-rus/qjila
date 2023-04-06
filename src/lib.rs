#![feature(
    array_methods,
    const_ops, const_trait_impl, const_slice_index
)]

#![allow(incomplete_features)] #![feature(
    adt_const_params
)]

#![allow(
    non_snake_case, non_camel_case_types
)]

mod error;
mod query;
pub mod qujila;
mod entity;
mod config;
mod db_type;
mod condition;
mod connection;


pub(crate) mod internal_macros {
    pub(crate) use qujila_macros::{
        __internal__into_query,
    };
}

pub(crate) use {
    qujila::cached_statements::CACHED_STATEMENTS
};

pub use {
    qujila::Qujila,
};

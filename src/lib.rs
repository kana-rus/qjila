/*===== compiler features, allow lints =====*/
#![feature(
    array_methods,
    string_leak,
    const_ops, const_trait_impl, const_slice_index
)]

#![allow(incomplete_features)] #![feature(
    adt_const_params
)]

#![allow(
    non_snake_case, non_camel_case_types,
)]


/*===== feature management =====*/
#[cfg(any(
    all(feature="rt_tokio", feature="rt_async-std"),
))] compile_error!("
    Can't enable multiple `rt_*` features!
");

#[cfg(any(
    all(feature="db_postgres", feature="db_mysql"),
    all(feature="db_mysql", feature="db_sqlite"),
    all(feature="db_sqlite", feature="db_postgres"),
))] compile_error!("
    Can't enable multiple `db_*` features!
");


/*===== feature-abstruction layer =====*/
mod __feature__ {
    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::postgres::PgRow as Row;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::mysql::MySqlRow as Row;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::sqlite::SqliteRow as Row;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::PgPool as ConnectionPool;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::MySqlPool as ConnectionPool;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::SqlitePool as ConnectionPool;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::postgres::PgPoolOptions as PoolConfig;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::mysql::MySqlPoolOptions as PoolConfig;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::sqlite::SqlitePoolOptions as PoolConfig;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::Postgres as DB;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::MySql as DB;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::Sqlite as DB;
}


/*===== modules =====*/
mod error;
mod query;
mod db_type;
mod condition;

mod model;
mod pool;


/*===== in-crate reexport =====*/
pub(crate) use pool::pool;


/*===== public reexport =====*/
pub use error::Error;
pub use pool::spawn;
pub use model::Model;


/*===== external reexport =====*/
pub use sqlx::{FromRow};

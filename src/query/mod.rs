mod query_count;
mod query_exists; pub use query_exists::{exists};
mod query_is_single; pub use query_is_single::{is_single};

mod query_first; pub use query_first::{First};
mod query_all; pub use query_all::{All};
mod query_single; pub use query_single::{Single};
mod query_search; pub use query_search::{Search};

mod query_create; pub use query_create::{create, Create};
mod query_delete; pub use query_delete::{delete, Delete};
mod query_update; pub use query_update::{update, Update};

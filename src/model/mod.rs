use sqlx::FromRow;
use crate::{
    __sqlx__::Row,
};


pub trait Model: for<'r> FromRow<'r, Row> {
    
}

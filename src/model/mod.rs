use sqlx::FromRow;
use crate::{
    __feature__::Row,
};


pub trait Model: for<'r> FromRow<'r, Row> {

}

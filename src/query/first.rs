use std::{marker::PhantomData, future::IntoFuture};
use crate::{
    entity::{Entity, BuildCondition, Column},
    condition as cond, connection::Connection, error::Error,
};


pub struct First<E: Entity> {
    _entity: PhantomData<fn()->E>,
    connection: Connection,
    condition:  cond::Condition,
}
impl<E: Entity> First<E> {
    #[inline] pub fn WHERE<C: Into<cond::Condition>, F: Fn(E::ConditionBuilder)->C>(mut self, condition: F) -> Self {
        self.condition = condition(E::ConditionBuilder::new()).into();
        self
    }
    // #[inline] pub fn ORDER_ASC<const COLUMN: &'static str, F: Fn(E::ColumnSelector)->Column<COLUMN>>(mut self, column: F) -> Self {
    //     self.order.ASC(COLUMN);
    //     self
    // }
    // #[inline] pub fn ORDER_DESC<const COLUMN: &'static str, F: Fn(E::ColumnSelector)->Column<COLUMN>>(mut self, column: F) -> Self {
    //     self.order.DESC(COLUMN);
    //     self
    // }
    // #[inline] pub fn LIMIT(mut self, limit: usize) -> Self {
    //     self.limit.set(limit);
    //     self
    // }
}
const _: (/* First impls */) = {
    impl<E: Entity> IntoFuture for First<E> {
        type Output = Result<E, Error>;
        type IntoFuture = super::QueryOne<E, 0>;
        fn into_future(self) -> Self::IntoFuture {
            super::QueryOne {__as__: PhantomData,
                connection: self.connection,
                statement:  format!("SELECT * FROM {} {} LIMIT 1", E::TABLE_NAME, self.condition),
                params:     []
            }
        }
    }
};

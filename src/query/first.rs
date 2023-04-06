use std::{marker::PhantomData, future::IntoFuture};
use crate::{
    entity::{Entity, BuildCondition},
    condition as cond, connection::Connection, error::Error,
};


pub struct First<E: Entity> {
    _entity: PhantomData<fn()->E>,
    connection: Result<Connection, Error>,
    condition:  cond::Condition,
}
impl<E: Entity> First<E> {
    #[inline(always)] pub fn new(connection: Result<Connection, Error>) -> Self {
        Self { _entity: PhantomData, connection, condition: cond::Condition::new() }
    }

    #[inline(always)] pub fn WHERE<C: Into<cond::Condition>, F: Fn(E::ConditionBuilder)->C>(mut self, condition: F) -> Self {
        self.condition = condition(E::ConditionBuilder::new()).into();
        self
    }
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

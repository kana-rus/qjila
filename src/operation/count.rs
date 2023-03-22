use std::{marker::PhantomData, future::IntoFuture};
use crate::{
    condition as cond,
    entity::{Entity, BuildCondition},
};


pub struct Count<E: Entity>{
    _entity: PhantomData<fn()->E>,
    condition: cond::Condition,
}

impl<E: Entity> Count<E> {
    pub fn new() -> Self {
        Self { _entity: PhantomData, condition: cond::Condition::new() }
    }

    pub fn WHERE<C: Into<cond::Condition>, F: Fn(E::ConditionBuilder)->C>(mut self, condition: F) -> Self {
        self.condition = condition(E::ConditionBuilder::new()).into();
        self
    }
}
const _: (/* Count impls */) = {
    impl<E: Entity> IntoFuture for Count<E> {
        type IntoFuture = ;
        fn into_future(self) -> Self::IntoFuture {
            
        }
    }
};


#[cfg(test)]
fn __example__() {
    struct User {
        id: usize,
        name: String,
    } const _: (/* User impls */) = {
        impl<'r> crate::entity::FromRow<'r> for User {
            fn from_row(row: &'r deadpool_postgres::tokio_postgres::Row) -> Self {
                Self {
                    id: row.get::<_, i64>(0) as usize,
                    name: row.get(1),
                }
            }
        }

        struct UserCondition {
            id: cond::NumberCondition<"id">,
            name: cond::StringCondition<"name">,
        } impl BuildCondition for UserCondition {
            #[inline] fn new() -> Self {
                Self { id: cond::NumberCondition, name: cond::StringCondition }
            }
        }

        enum UserColumn {id, name}
        struct UserColumns {
            id: UserColumn,
            name: UserColumn,
        }
        impl crate::entity::SelectColumn for UserColumns {

        }

        impl Entity for User {
            const TABLE_NAME: &'static str = "users";
            type ConditionBuilder = UserCondition;
            type ColumnSelector = UserColumns;
        }
    };

    let _ = Count::<User>::new()
        .WHERE(|u| [
            u.id.between(100, 1000),
            u.name.like("%user%"),
        ]);
}

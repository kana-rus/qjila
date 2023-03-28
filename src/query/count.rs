use std::{
    marker::PhantomData,
    future::IntoFuture,
};
use crate::{
    error::Error,
    condition as cond,
    connection::Connection,
    entity::{Entity, BuildCondition},
};


pub struct Count<E: Entity>{
    _entity: PhantomData<fn()->E>,
    connection: Connection,
    condition: cond::Condition,
}

impl<E: Entity> Count<E> {
    pub fn WHERE<C: Into<cond::Condition>, F: Fn(E::ConditionBuilder)->C>(mut self, condition: F) -> Self {
        self.condition = condition(E::ConditionBuilder::new()).into();
        self
    }
}
const _: (/* Count impls */) = {
    impl<E: Entity> IntoFuture for Count<E> {
        type Output = Result<usize, Error>;
        type IntoFuture = super::QueryOne<0, usize>;
        fn into_future(self) -> Self::IntoFuture {
            super::QueryOne {
                __as__:     PhantomData,
                connection: self.connection,
                statement:  format!("SELECT COUNT(*) FROM {} {}", E::TABLE_NAME, self.condition),
                params:     [],
            }
        }
    }
};


#[cfg(test)] #[allow(unused)]
async fn __example__(connection: Connection) -> Result<(), Error> {
    use crate::{condition::Condition, entity::CreateEntity};

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
        } impl crate::entity::SelectColumn for UserColumns {
            fn new() -> Self {
                Self { id: UserColumn::id, name: UserColumn::name }
            }
        }

        struct CreateUser {
            name: String,
        } impl CreateEntity for CreateUser {
            const NON_DEFAULT_COLUMN_NAMES: &'static str = "name";
            type NonDefaultColumnTypes = String;
        }

        impl Entity for User {
            const TABLE_NAME: &'static str = "users";
            type Creator = CreateUser;
            type ConditionBuilder = UserCondition;
            type ColumnSelector = UserColumns;
        }
    };

    let count: usize = Count::<User> {_entity:PhantomData, condition:Condition::new(), connection}
        .WHERE(|u| [
            u.id.between(100, 1000),
            u.name.like("%user%"),
        ])
        .await?;

    Ok(())
}

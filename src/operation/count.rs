use std::{
    marker::PhantomData,
    future::{IntoFuture, Future},
    pin::{Pin, pin}, task::Poll,
};
use crate::{
    condition as cond,
    connection::Connection,
    entity::{Entity, BuildCondition}, error::Error,
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
    // impl<E: Entity> IntoFuture for Count<E> {
    //     type Output = Result<usize, crate::error::Error>;
    //     type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;
    //     fn into_future(self) -> Self::IntoFuture {
    //         Box::pin(async move {
    //             let client = &self.connection.0;
    //             let stmt = client
    //                 .prepare_cached(&format!(
    //                     "SELECT COUNT(*) FROM {}",
    //                     E::TABLE_NAME,
    //                 ))
    //                 .await?;
    //             let row = client
    //                 .query_one(&stmt, &[])
    //                 .await?;
    //             Ok(row.get::<_, i64>(0) as usize)
    //         })
    //     }
    // }

    impl<E: Entity> IntoFuture for Count<E> {
        type Output = Result<usize, Error>;
        type IntoFuture = CountResult;
        fn into_future(self) -> Self::IntoFuture {
            CountResult {
                connection: self.connection,
                sql: format!(
                    "SELECT COUNT(*) FROM {} {}",
                    E::TABLE_NAME,
                    self.condition,
                ),
            }
        }
    }

    pub struct CountResult {
        connection: Connection,
        sql:        String,
    } impl Future for CountResult {
        type Output = Result<usize, Error>;
        fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
            

            todo!()
        }
    }
};


#[cfg(test)] #[allow(unused)]
async fn __example__(connection: Connection) {
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

    let _ = Count::<User> {_entity:PhantomData, condition:Condition::new(), connection}
        .WHERE(|u| [
            u.id.between(100, 1000),
            u.name.like("%user%"),
        ])
        ;
}

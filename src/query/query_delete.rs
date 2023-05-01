use std::{
    future::Future,
    task::Poll,
    pin::pin, marker::PhantomData,
};
use crate::{
    __feature__,
    Error,
    pool,
    Model,
    Table,
    condition as cond,
};


pub struct delete<T: Table> {
    __table__: PhantomData<fn()->T>,
    condition: cond::Condition,
    limit:     cond::Limit,
    order:     cond::Order,
}
impl<T: Table> Future for delete<T> {
    type Output = Result<(), Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        #[cfg(feature="db_mysql")]
        let sql = format!(
           "DELETE FROM {} {} {} {}",
           T::TABLE_NAME,
           self.condition,
           self.order,
           self.limit,
        );

        #[cfg(not(feature="db_mysql"))]
        let sql = if (self.limit.is_set() || self.order.is_set()) && T::ID_COLUMN.is_none() {
            return Poll::Ready(Err(Error::ConfigError(format!(
                "Query `delete` from {}: In `delete`, you can set `LIMIT` or `ORDER` only when the table has a unique column.",
                T::TABLE_NAME,
            ))))
        } else {format!(
            "DELETE FROM {} WHERE {} IN ( SELECT {} FROM {} {} {} {} )",
            T::TABLE_NAME,
            T::ID_COLUMN.unwrap(),
            T::ID_COLUMN.unwrap(),
            T::TABLE_NAME,
            self.condition,
            self.order,
            self.limit,
        )};

        let delete_future = pin!(sqlx::query::<__feature__::DB>(&sql).execute(pool()));

        match delete_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(_))  => Poll::Ready(Ok(())),
        }
    }
}

pub struct Delete<T: Table, M: Model> {
    __table__: PhantomData<fn()->T>,
    __model__: PhantomData<fn()->M>,
    condition: cond::Condition,
    limit:     cond::Limit,
    order:     cond::Order,
}
impl<T: Table, M: Model> Future for Delete<T, M> {
    type Output = Result<Vec<M>, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        #[cfg(feature="db_mysql")]
        let sql = format!(
           "DELETE FROM {} {} {} {} RETURNING {}",
           T::TABLE_NAME,
           self.condition,
           self.order,
           self.limit,
           M::SELECT_COLUMNS,
        );

        #[cfg(not(feature="db_mysql"))]
        let sql = if (self.limit.is_set() || self.order.is_set()) && T::ID_COLUMN.is_none() {
            return Poll::Ready(Err(Error::ConfigError(format!(
                "Query `Delete` from {}: In `Delete`, you can set `LIMIT` or `ORDER` only when the table has a unique column.",
                T::TABLE_NAME,
            ))))
        } else {format!(
            "DELETE FROM {} WHERE {} IN ( SELECT {} FROM {} {} {} {} ) RETURNING {}",
            T::TABLE_NAME,
            T::ID_COLUMN.unwrap(),
            T::ID_COLUMN.unwrap(),
            T::TABLE_NAME,
            self.condition,
            self.order,
            self.limit,
            M::SELECT_COLUMNS,
        )};

        let delete_future = pin!(sqlx::query::<__feature__::DB>(&sql).fetch_all(pool()));

        match delete_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(err)) => Poll::Ready(Err(err.into())),
            Poll::Ready(Ok(rows)) => Poll::Ready(rows.into_iter().map(|row| M::from_row(&row)).collect()),
        }
    }
}

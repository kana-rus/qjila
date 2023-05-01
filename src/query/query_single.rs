use std::{
    future::Future,
    task::Poll,
    pin::pin, marker::PhantomData,
};
use crate::{
    __feature__,
    Error,
    pool,
    Model, Table,
    condition::Condition,
};


pub struct Single<T: Table, M: Model> {
    __table__: PhantomData<fn()->T>,
    __model__: PhantomData<fn()->M>,
    condition: Condition,
}
impl<T: Table, M: Model> Future for Single<T, M> {
    type Output = Result<M, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "SELECT {} FROM {} {} LIMIT 2",
            M::SELECT_COLUMNS,
            T::TABLE_NAME,
            self.condition,
        );
        let query_future = pin!(sqlx::query::<__feature__::DB>(&sql).fetch_all(pool()));

        match query_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(err)) => Poll::Ready(Err(err.into())),
            Poll::Ready(Ok(rows)) => Poll::Ready(
                if rows.len() == 1 {
                    M::from_row(&rows[0])
                } else {
                    Err(Error::DBError(format!("Query `Single` from `{}`: Found more than 1 row", T::TABLE_NAME)))
                }
            ),
        }
    }
}

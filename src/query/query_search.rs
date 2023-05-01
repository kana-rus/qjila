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


pub struct Search<T: Table, M: Model> {
    __table__: PhantomData<fn()->T>,
    __model__: PhantomData<fn()->M>,
    condition: Condition,
}
impl<T: Table, M: Model> Future for Search<T, M> {
    type Output = Result<Option<M>, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "SELECT {} FROM {} {} LIMIT 1",
            M::SELECT_COLUMNS,
            T::TABLE_NAME,
            self.condition,
        );
        let query_future = pin!(sqlx::query::<__feature__::DB>(&sql).fetch_optional(pool()));

        match query_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e))  => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(row)) => Poll::Ready(row.map(|r| M::from_row(&r)).transpose()),
        }
    }
}

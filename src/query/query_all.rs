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
    condition as cond,
};


pub struct All<T: Table, M: Model> {
    __table__: PhantomData<fn()->T>,
    __model__: PhantomData<fn()->M>,
    condition: cond::Condition,
    limit:     cond::Limit,
    order:     cond::Order,
}
impl<T: Table, M: Model> Future for All<T, M> {
    type Output = Result<Vec<M>, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "SELECT {} FROM {} {} {} {}",
            M::SELECT_COLUMNS,
            T::TABLE_NAME,
            self.condition,
            self.order,
            self.limit,
        );
        let query_future = pin!(sqlx::query::<__feature__::DB>(&sql).fetch_all(pool()));

        match query_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(err)) => Poll::Ready(Err(err.into())),
            Poll::Ready(Ok(rows)) => Poll::Ready(rows.into_iter().map(|row| M::from_row(&row)).collect()),
        }
    }
}

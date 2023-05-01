use std::{
    future::Future,
    pin::pin,
    task::Poll, marker::PhantomData,
};
use crate::{
    condition::Condition,
    Error,
    pool, Table,
};


pub struct exists<T: Table> {
    __table__: PhantomData<fn()->T>,
    condition: Condition,
}
impl<T: Table> Future for exists<T> {
    type Output = Result<bool, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "SELECT COUNT(id) FROM (SELECT 1 as id FROM {} {} LIMIT 1)",
            T::TABLE_NAME,
            self.condition,
        );
        let fetch_future = pin!(sqlx::query_as::<_, (i64,)>(&sql).fetch_one(pool()));

        match fetch_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok((count,))) => Poll::Ready(Ok(count > 0)),
        }
    }
}
impl<T: Table> exists<T> {
    #[inline] pub(crate) fn new(condition: Condition) -> Self {
        Self { __table__: PhantomData, condition }
    }
}

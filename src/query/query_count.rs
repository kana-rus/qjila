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


pub struct count<T: Table> {
    __table__: PhantomData<fn()->T>,
    condition: Condition,
}
impl<T: Table> Future for count<T> {
    type Output = Result<usize, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "SELECT COUNT(*) FROM {} {}",
            T::TABLE_NAME,
            self.condition,
        );
        let fetch_future = pin!(
            sqlx::query_as::<_, (i64,)>(&sql)
                .fetch_one(pool())
        );

        match fetch_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok((count,))) => Poll::Ready(Ok(count as usize)),
        }
    }
}

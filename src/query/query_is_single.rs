use std::{
    future::Future,
    pin::pin,
    task::Poll,
};
use crate::{
    condition::Condition,
    Error,
    pool,
};


pub struct is_single<const TABLE_NAME: &'static str> {
    condition: Condition,
}

impl<const TABLE_NAME: &'static str> Future for is_single<TABLE_NAME> {
    type Output = Result<bool, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "SELECT COUNT(*) FROM {} {}",
            TABLE_NAME,
            self.condition,
        );
        let fetch_future = pin!(
            sqlx::query_as::<_, (i64,)>(&sql)
                .fetch_one(pool())
        );

        match fetch_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok((count,))) => Poll::Ready(Ok(count == 1)),
        }
    }
}

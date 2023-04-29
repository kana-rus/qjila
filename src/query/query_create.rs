use std::{
    future::Future,
    task::Poll,
    pin::pin,
};
use crate::{
    __feature__,
    Error,
    pool,
    Model,
};


pub struct create<const TABLE_NAME: &'static str> {
    column_names: Vec<String>,
    column_values: Vec<String>,
}
pub struct Create<M: Model> {
    column_names: Vec<String>,
    column_values: Vec<String>,

    todo!()
}

impl<const TABLE_NAME: &'static str> Future for create<TABLE_NAME> {
    type Output = Result<(), Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = {
            let params = (1..=self.column_names.len())
                .map(|n| format!("${n}"))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "INSERT INTO {} ({}) VALUES ({})",
                TABLE_NAME,
                self.column_names.join(","),
                params,
            )
        };
        let query = {
            let mut query = sqlx::query::<__feature__::DB>(&sql);
            for value in &self.column_values {
                query = query.bind(value)
            }
            query
        };
        let create_future = pin!(query.execute(pool()));

        match create_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(_))  => Poll::Ready(Ok(())),
        }
    }
}

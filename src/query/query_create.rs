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
};


pub struct create<T: Table> {
    __table__: PhantomData<fn()->T>,
    column_names: Vec<String>,
    column_values: Vec<String>,
}
impl<T: Table> Future for create<T> {
    type Output = Result<(), Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = {
            let params = (1..=self.column_names.len())
                .map(|n| format!("${n}"))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "INSERT INTO {} ({}) VALUES ({})",
                T::TABLE_NAME,
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

pub struct Create<T: Table, M: Model> {
    __table__: PhantomData<fn()->T>,
    __model__: PhantomData<fn()->M>,
    column_names: Vec<String>,
    column_values: Vec<String>,
}
impl<T: Table, M: Model> Future for Create<T, M> {
    type Output = Result<M, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = {
            let params = (1..=self.column_names.len())
                .map(|n| format!("${n}"))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
                T::TABLE_NAME,
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
        let Create_future = pin!(query.fetch_one(pool()));

        match Create_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e))  => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(row)) => Poll::Ready(M::from_row(&row)),
        }
    }
}

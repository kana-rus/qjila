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
use super::Q;

/// create `String` like
/// `"$1,$2,$3"`
/// used in `INSERT INTO` query.
fn build_values_params<'p>(n: usize) -> String {
    let mut params = if n < 10 {
        (1..=n as u8).fold(
            String::with_capacity(3*n-1),
            |mut params, i| {
                params.push('$');
                params.push(i as char);
                params.push(',');
                params
            }
        )
    } else {
        let dig1 = (1..10u8).fold(
            String::with_capacity(3*9-1),
            |mut params, i| {
                params.push('$');
                params.push(i as char);
                params.push(',');
                params
            }
        );
        let dig2 = (10u8..n as u8).fold(
            String::with_capacity(3*(n-9)-1),
            |mut params, i| {
                params.push('$');
                params.push_str(&i.to_string());
                params.push(',');
                params
            }
        );
        [dig1, dig2].concat()
    };
    let _ = params.pop(); // remove final ','
    params
}


/// `COLUMNS_AND_VALUE_PARAMS` is like
/// ```
/// "(name, password, age) VALUES ($1, $2, $3)"
/// ```
pub struct create<T: Table> {
    __table__: PhantomData<fn()->T>,
    column_names: Q<&'static str>,
    column_values: Q<String>,
}
impl<T: Table> Future for create<T> {
    type Output = Result<(), Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            T::TABLE_NAME,
            self.column_names.join(","),
            unsafe {build_values_params(self.column_names.len)}
        );
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
impl<T: Table> create<T> {
    #[inline] pub(crate) fn new(column_names: Q<&'static str>, column_values: Q<String>) -> Self {
        Self { __table__: PhantomData, column_names, column_values }
    }
}

/// `COLUMNS_AND_VALUE_PARAMS` is like
/// ```
/// "(name, password, age) VALUES ($1, $2, $3)"
/// ```
pub struct Create<T: Table, M: Model> {
    __table__: PhantomData<fn()->T>,
    __model__: PhantomData<fn()->M>,
    column_names:  Q<&'static str>,
    column_values: Q<String>,
}
impl<T: Table, M: Model> Future for Create<T, M> {
    type Output = Result<M, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
            T::TABLE_NAME,
            self.column_names.join(","),
            build_values_params(self.column_names.len),
            M::SELECT_COLUMNS,
        );
        let query = {
            let mut query = sqlx::query::<__feature__::DB>(&sql);
            for value in &self.column_values {
                query = query.bind(value)
            }
            query
        };
        let create_future = pin!(query.fetch_one(pool()));

        match create_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e))  => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(row)) => Poll::Ready(M::from_row(&row)),
        }
    }
}
impl<T: Table, M: Model> Create<T, M> {
    #[inline] pub(crate) fn new(column_names: Q<&'static str>, column_values: Q<String>) -> Self {
        Self { __table__:PhantomData, __model__:PhantomData, column_names, column_values }
    }
}

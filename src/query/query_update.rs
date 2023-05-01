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
use super::Q;

/// create `String` like
/// `"name=$1,password=$2,age=$3"`
/// used in `UPDATE table SET 〜` query
#[inline] fn build_set_params(set_columns: &Q<&'static str>) -> String {
    let mut params = if set_columns.len < 10 {
        set_columns.enumerate()
            .fold(
                String::with_capacity(6*set_columns.len-1),
                |mut params, (n, column)| {
                    params.push_str(column);
                    params.push('=');
                    params.push('$');
                    params.push(n as u8 as char);
                    params.push(',');
                    params
                }
            )
    } else {
        set_columns.enumerate()
            .fold(
                String::with_capacity(6*set_columns.len-1),
                |mut params, (n, column)| {
                    params.push_str(column);
                    params.push('=');
                    params.push('$');
                    params.push_str(&n.to_string());
                    params.push(',');
                    params
                }
            )
    };
    let _ = params.pop(); // remove final ','
    params
}


pub struct update<T: Table> {
    __table__:   PhantomData<fn()->T>,
    set_columns: Q<&'static str>,
    set_values:  Q<String>,
    condition:   cond::Condition,
    limit:       cond::Limit,
    order:       cond::Order,
}
impl<T: Table> Future for update<T> {
    type Output = Result<(), Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if self.set_columns.is_empty() {return Poll::Ready(Ok(()))}

        #[cfg(feature="db_mysql")]
        let sql = format!(
           "UPDATE {} SET {} {} {} {}",
           T::TABLE_NAME,
           build_set_params(&self.set_columns),
           self.condition,
           self.order,
           self.limit,
        );

        #[cfg(not(feature="db_mysql"))]
        let sql = if (self.limit.is_set() || self.order.is_set()) && T::ID_COLUMN.is_none() {
            return Poll::Ready(Err(Error::ConfigError(format!(
                "Query `update` from {}: In `update`, you can set `LIMIT` or `ORDER` only when the table has a unique column.",
                T::TABLE_NAME,
            ))))
        } else {format!(
            "UPDATE {} SET {} WHERE {} IN ( SELECT {} FROM {} {} {} {} )",
            T::TABLE_NAME,
            build_set_params(&self.set_columns),
            T::ID_COLUMN.unwrap(),
            T::ID_COLUMN.unwrap(),
            T::TABLE_NAME,
            self.condition,
            self.order,
            self.limit,
        )};

        let query = {
            let mut query = sqlx::query::<__feature__::DB>(&sql);
            for value in &self.set_values {
                query = query.bind(value)
            }
            query
        };
        let update_future = pin!(query.execute(pool()));

        match update_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(_))  => Poll::Ready(Ok(())),
        }
    }
}
impl<T: Table> update<T> {
    #[inline] pub(crate) fn new(condition: cond::Condition, set_columns: Q<&'static str>, set_values: Q<String>) -> Self {
        Self { __table__:PhantomData, set_columns, set_values, condition,
            limit: cond::Limit::new(),
            order: cond::Order::new(),
        }
    }
}

pub struct Update<T: Table, M: Model> {
    __table__:   PhantomData<fn()->T>,
    __model__:   PhantomData<fn()->M>,
    set_columns: Q<&'static str>,
    set_values:  Q<String>,
    condition:   cond::Condition,
    limit:       cond::Limit,
    order:       cond::Order,
}
impl<T: Table, M: Model> Future for Update<T, M> {
    type Output = Result<Vec<M>, Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if self.set_columns.is_empty() {return Poll::Ready(Ok(vec![]))}

        #[cfg(feature="db_mysql")]
        let sql = format!(
           "UPDATE {} SET {} {} {} {} RETURNING {}",
           T::TABLE_NAME,
           build_set_params(&self.set_columns),
           set_params(self.set_columns),
           self.condition,
           self.order,
           self.limit,
           M::SELECT_COLUMNS,
        );

        #[cfg(not(feature="db_mysql"))]
        let sql = if (self.limit.is_set() || self.order.is_set()) && T::ID_COLUMN.is_none() {
            return Poll::Ready(Err(Error::ConfigError(format!(
                "Query `Update` from {}: In `Update`, you can set `LIMIT` or `ORDER` only when the table has a unique column.",
                T::TABLE_NAME,
            ))))
        } else {format!(
            "UPDATE {} SET {} WHERE {} IN ( SELECT {} FROM {} {} {} {} ) RETURNING {}",
            T::TABLE_NAME,
            build_set_params(&self.set_columns),
            T::ID_COLUMN.unwrap(),
            T::ID_COLUMN.unwrap(),
            T::TABLE_NAME,
            self.condition,
            self.order,
            self.limit,
            M::SELECT_COLUMNS,
        )};

        let query = {
            let mut query = sqlx::query::<__feature__::DB>(&sql);
            for value in &self.set_values {
                query = query.bind(value)
            }
            query
        };
        let update_future = pin!(query.fetch_all(pool()));

        match update_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(err)) => Poll::Ready(Err(err.into())),
            Poll::Ready(Ok(rows)) => Poll::Ready(rows.into_iter().map(|row| M::from_row(&row)).collect()),
        }
    }
}
impl<T: Table, M: Model> Update<T, M> {
    #[inline] pub(crate) fn new(condition: cond::Condition, set_columns: Q<&'static str>, set_values: Q<String>) -> Self {
        Self { __table__:PhantomData, __model__:PhantomData, set_columns, set_values, condition,
            limit: cond::Limit::new(),
            order: cond::Order::new(),
        }
    }
}

#[cfg(test)]
fn __<T1: Table, M1: Model>() {
    let _updater = Update::<T1, M1>::new(cond::Condition::new(), Q::new(), Q::new());
}

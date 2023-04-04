mod create; pub(crate) use create::{Create, _Create};
mod first;  pub(crate) use first::First;
mod all; pub(crate) use all::All;
mod update; pub(crate) use update::{Update, _Update};
mod delete; pub(crate) use delete::{Delete, _Delte};
mod count; pub(crate) use count::Count;

use deadpool_postgres::tokio_postgres::types::ToSql;
use std::{marker::PhantomData, future::{Future, IntoFuture}, pin::{Pin, pin}, task::Poll};
use crate::{connection::Connection, entity::FromRow, error::Error};


pub struct QueryOne<As: for<'r> FromRow<'r>, const N_PARAMS: usize> {
    __as__:     PhantomData<fn()->As>,
    connection: Connection,
    statement:  String,
    params:     [String; N_PARAMS],
} const _: (/* QueryOne impls */) = {
    impl<As: for<'r> FromRow<'r>, const N_PARAMS: usize> Future for QueryOne<As, N_PARAMS> {
        type Output = Result<As, Error>;
        fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
            let mut iter = self.params.iter().map(|s| s as &(dyn ToSql + Sync));
            let params: [_; N_PARAMS] = std::array::from_fn(move |_| unsafe {iter.next().unwrap_unchecked()});

            let client = &self.connection.0;
            match pin!(client.prepare_cached(&self.statement)).poll(cx) {
                Poll::Pending         => return Poll::Pending,
                Poll::Ready(Err(e))   => return Poll::Ready(Err(e.into())),
                Poll::Ready(Ok(stmt)) => match pin!(client.query_one(&stmt, &params)).poll(cx) {
                    Poll::Pending        => Poll::Pending,
                    Poll::Ready(Err(e))  => Poll::Ready(Err(e.into())),
                    Poll::Ready(Ok(row)) => Poll::Ready(Ok(As::from_row(&row))),
                }
            }
        }
    }
};

pub struct QueryMany<As: for<'r> FromRow<'r>, const N_PARAMS: usize> {
    __as__:     PhantomData<fn()->As>,
    connection: Connection,
    statement:  String,
    params:     [String; N_PARAMS],
} const _: (/* QueyMany impls */) = {
    impl<As: for<'r> FromRow<'r>, const N_PARAMS: usize> Future for QueryMany<As, N_PARAMS> {
        type Output = Result<Vec<As>, Error>;
        fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
            let mut iter = self.params.iter().map(|s| s as &(dyn ToSql + Sync));
            let params: [_; N_PARAMS] = std::array::from_fn(move |_| unsafe {iter.next().unwrap_unchecked()});

            let client = &self.connection.0;
            match pin!(client.prepare_cached(&self.statement)).poll(cx) {
                Poll::Pending         => return Poll::Pending,
                Poll::Ready(Err(err)) => return Poll::Ready(Err(err.into())),
                Poll::Ready(Ok(stmt)) => match pin!(client.query(&stmt, &params)).poll(cx) {
                    Poll::Pending         => Poll::Pending,
                    Poll::Ready(Err(err)) => Poll::Ready(Err(err.into())),
                    Poll::Ready(Ok(rows)) => Poll::Ready(Ok(
                        rows.into_iter()
                            .map(|row| As::from_row(&row))
                            .collect()
                    )),
                }
            }
        }
    }
};


#[cfg(test)] #[allow(unused)]
fn __assert_impls__() {
    fn ToSql_Sync<T: ToSql + Sync>() {}
    ToSql_Sync::<String>();

    let string: String              = format!("");
    let _:      &(dyn ToSql + Sync) = &string;

    const N: usize = 3;

    let strings: [String; N]              = [format!(""), format!(""), format!("")];
    let strings: [&String; N]             = strings.each_ref();
    let strings: [&(dyn ToSql + Sync); N] = strings.map(|s| s as &(dyn ToSql + Sync));
    let _: &[&(dyn ToSql + Sync)]         = &strings;

    let slice: &[String; N]              = &[format!(""), format!(""), format!("")];
    let slice: &[&String; N]             = &slice.each_ref();
    //let slice: &[&(dyn ToSql + Sync); N] = slice.;

}

#[cfg(test)] #[allow(unused)]
mod experement {
    use deadpool_postgres::tokio_postgres::types::ToSql;

    fn f<const N: usize>(ref_array: &[String; N]) {
        let mut iter = ref_array
            .iter()
            .map(|s: &String| s as &(dyn ToSql + Sync));

        let new: [&(dyn ToSql + Sync); N] = std::array::from_fn(
            move |_| unsafe {
                iter.next().unwrap_unchecked()
            }
        );

        let _: &[&(dyn ToSql + Sync)] = &new;
    }
}

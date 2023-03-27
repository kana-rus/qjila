mod create; pub(crate) use create::{Create, _Create};
mod first; use deadpool_postgres::tokio_postgres::types::ToSql;
pub(crate) use first::First;
mod all; pub(crate) use all::All;
mod update; pub(crate) use update::{Update, _Update};
mod delete; pub(crate) use delete::{Delete, _Delte};
mod count; pub(crate) use count::Count;

use std::{marker::PhantomData, future::{Future, IntoFuture}, pin::{Pin, pin}, task::Poll};
use crate::{connection::Connection, entity::FromRow, error::Error};


// pub struct Query<'params, 'tosql, As: for<'r> FromRow<'r>> {
//     _as:        PhantomData<fn()->As>,
//     connection: Connection,
//     sql:        String,
//     params:     &'params [&'tosql(dyn ToSql + Sync)],
// } const _: (/* Query impls */) = {
//     impl<'params, 'tosql, As: for<'r> FromRow<'r>> Future for Query<'params, 'tosql, As> {
//         type Output = Result<As, Error>;
//         fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
//             let client = &self.connection.0;
//             match pin!(client.prepare_cached(&self.sql)).poll(cx) {
//                 Poll::Pending         => return Poll::Pending,
//                 Poll::Ready(Err(e))   => return Poll::Ready(Err(e.into())),
//                 Poll::Ready(Ok(stmt)) => match pin!(client.query_one(&stmt, &[])).poll(cx) {
//                     Poll::Pending        => Poll::Pending,
//                     Poll::Ready(Err(e))  => Poll::Ready(Err(e.into())),
//                     Poll::Ready(Ok(row)) => Poll::Ready(Ok(As::from_row(&row))),
//                 }
//             }
//         }
//     }
// };
// 
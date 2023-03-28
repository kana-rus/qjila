mod create; pub(crate) use create::{Create, _Create};
mod first;  pub(crate) use first::First;
mod all; pub(crate) use all::All;
mod update; pub(crate) use update::{Update, _Update};
mod delete; pub(crate) use delete::{Delete, _Delte};
mod count; pub(crate) use count::Count;

use deadpool_postgres::tokio_postgres::types::ToSql;
use std::{marker::PhantomData, future::{Future, IntoFuture}, pin::{Pin, pin}, task::Poll};
use crate::{connection::Connection, entity::FromRow, error::Error};


pub struct QueryOne<const N_PARAMS: usize, As: for<'r> FromRow<'r>> {
    __as__:     PhantomData<fn()->As>,
    connection: Connection,
    statement:  String,
    params:     [String; N_PARAMS],
} const _: (/* Query impls */) = {
    impl<const N_PARAMS: usize, As: for<'r> FromRow<'r>> Future for QueryOne<N_PARAMS, As> {
        type Output = Result<As, Error>;
        fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
            let params: &[&String] = &self.params.each_ref();
            let params: &[&(dyn ToSql + Sync)] = &[];//params as &[&(dyn ToSql + Sync)];

            let client = &self.connection.0;
            match pin!(client.prepare_cached(&self.statement)).poll(cx) {
                Poll::Pending         => return Poll::Pending,
                Poll::Ready(Err(e))   => return Poll::Ready(Err(e.into())),
                Poll::Ready(Ok(stmt)) => {
                    match pin!(client.query_one(&stmt, params)).poll(cx) {
                        Poll::Pending        => Poll::Pending,
                        Poll::Ready(Err(e))  => Poll::Ready(Err(e.into())),
                        Poll::Ready(Ok(row)) => Poll::Ready(Ok(As::from_row(&row))),
                    }
                }
            }
        }
    }
};


#[cfg(test)]
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

#[cfg(test)]
fn string_array_each_ref<const N: usize>(array: &[String; N]) -> [&String; N] {
    

    // std::array::fr
    todo!()
}
/*

#[unstable(feature = "array_methods", issue = "76118")]
pub fn each_ref(&self) -> [&T; N] {
    from_trusted_iterator(self.iter())
}

#[inline]
fn from_trusted_iterator<T, const N: usize>(iter: impl UncheckedIterator<Item = T>) -> [T; N] {
    try_from_trusted_iterator(iter.map(NeverShortCircuit)).0
}

#[inline]
fn try_from_trusted_iterator<T, R, const N: usize>(
    iter: impl UncheckedIterator<Item = R>,
) -> ChangeOutputType<R, [T; N]>
where
    R: Try<Output = T>,
    R::Residual: Residual<[T; N]>,
{
    assert!(iter.size_hint().0 >= N);
    fn next<T>(mut iter: impl UncheckedIterator<Item = T>) -> impl FnMut(usize) -> T {
        move |_| {
            // SAFETY: We know that `from_fn` will call this at most N times,
            // and we checked to ensure that we have at least that many items.
            unsafe { iter.next_unchecked() }
        }
    }

    try_from_fn(next(iter))
}

*/

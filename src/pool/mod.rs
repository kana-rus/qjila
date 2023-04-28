mod config;

use config::Config;
use std::future::Future;


mod __ {
    use std::sync::OnceLock;
    use crate::__sqlx__::ConnectionPool;

    pub(crate) static CONNECTION_POOL: OnceLock<ConnectionPool> = OnceLock::new();
}


/// **MUST** be called **AFTER** `spawn().〜.await?;` was executed
pub(crate) fn pool<'p>() -> &'p crate::__sqlx__::ConnectionPool {
    __::CONNECTION_POOL.get()
        .expect("connection pool isn't initialized")
}

/// Establish connnection pool with given configuration.
/// 
/// **ALL** queries **MUST** be executed **AFTER** `qujila::spawn()/* some config */.await` returned `Ok(())`.
/// 
/// <br/>
/// 
/// ```ignore
/// use qujila::Error;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     qujila::spawn("MY_DB_URL")
///         .max_connections(42)
///         .await?;
/// 
///     /* do something with DB */
/// }
/// ```
pub fn spawn(DB_URL: impl ToString) -> Config {
    Config::new(DB_URL)
} const _: () = {
    use std::{task::Poll, pin::pin};

    impl Future for Config {
        type Output = Result<(), crate::Error>;
        fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
            let (db_url, pool_options) = self.get_mut().clone().into_sqlx_pool_config();
            let connection_future = pool_options.connect(db_url.leak());

            match pin!(connection_future).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(err)) => Poll::Ready(Err(err.into())),
                Poll::Ready(Ok(pool)) => Poll::Ready(
                    if __::CONNECTION_POOL.set(pool).is_ok() {Ok(())} else {
                        Err(crate::Error::ConfigError(
                            format!("Failed to establish connection pool...")
                        ))
                    }
                )
            }
        }
    }
};

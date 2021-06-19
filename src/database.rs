use actix::{Actor, Addr, Context, Handler, Message, Response};
use deadpool_postgres::{Client, Config, ManagerConfig, Pool, PoolError, RecyclingMethod};
use futures::TryFutureExt;
use std::future::Future;
use tokio_postgres::Row;

pub fn get_config() -> Config {
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);
    cfg.dbname = Some("actors".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("admin".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg
}

pub struct Database {
    pool: Pool,
}

impl Actor for Database {
    type Context = Context<Self>;
    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.pool.close();
    }
}
impl Database {
    pub fn new(pool: Pool) -> Addr<Self> {
        Database { pool }.start()
    }
    fn get_client(&self) -> impl Future<Output = Result<Client, PoolError>> {
        let pool = self.get_ref_pool();
        pool.get()
    }
    fn get_ref_pool(&self) -> &'static Pool {
        let pointer: *const Pool = &self.pool;
        unsafe { pointer.as_ref().unwrap() }
    }
}

#[derive(Message)]
#[rtype(result = "Result<String, PoolError>")]
pub struct Hello;

impl Handler<Hello> for Database {
    type Result = Response<Result<String, PoolError>>;
    fn handle(
        &mut self,
        _msg: Hello,
        _ctx: &mut Self::Context,
    ) -> Response<Result<String, PoolError>> {
        let result = self.get_client().and_then(|client: Client| async move {
            let stmt = client
                .prepare_cached("select concat('Hello ', $1::TEXT)")
                .await
                .unwrap();
            let rows: &[Row] = &client.query(&stmt, &[&"world"]).await.unwrap();
            let result = rows[0].get(0);
            Ok::<String, PoolError>(result)
        });
        Response::fut(result)
    }
}

#[cfg(test)]
mod database_test {
    use crate::database::{Database, Hello};
    use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod};
    use futures::{stream, StreamExt};
    use std::{ops::Range, sync::Arc};
    use tokio_postgres::NoTls;

    fn get_config() -> Config {
        let mut cfg = Config::new();
        cfg.host = Some("localhost".to_string());
        cfg.port = Some(5432);
        cfg.dbname = Some("actors".to_string());
        cfg.user = Some("postgres".to_string());
        cfg.password = Some("admin".to_string());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        cfg
    }

    #[actix::test]
    async fn say_hello() {
        let cfg = get_config();
        let pool = cfg.create_pool(NoTls).unwrap();
        let addr = Database::new(pool);
        let message = addr.send(Hello).await.unwrap().unwrap();
        assert_eq!("Hello world".to_string(), message);
    }

    #[actix::test]
    async fn say_hello_many_times() {
        let cfg = get_config();
        let pool = cfg.create_pool(NoTls).unwrap();
        let addr = Database::new(pool);
        let saying_stream = stream::iter::<Range<u16>>(0..1000);
        let messages = saying_stream
            .fold(Vec::with_capacity(1000), |mut messages, _| async {
                let message = addr.send(Hello).await.unwrap().unwrap();
                messages.push(message);
                messages
            })
            .await;
        assert_eq!(1000, messages.len());
    }
    #[actix::test]
    async fn say_hello_in_many_threads() {
        let cfg = get_config();
        let pool = cfg.create_pool(NoTls).unwrap();
        let addr = Arc::new(Database::new(pool));
        stream::iter::<Range<u16>>(0..10000)
            .map(|_| Arc::clone(&addr))
            .for_each(|actor| async move {
                actix::spawn(async move {
                    let message = actor.send(Hello).await.unwrap().unwrap();
                    assert_eq!("Hello world", message);
                })
                .await
                .unwrap();
            })
            .await;
    }
}

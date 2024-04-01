use std::future::Future;

use crate::platform::config::config;
use crate::platform::config::socket_address;
use crate::platform::db::connection_manager::setup_connection_pool_sqlx;
use crate::platform::service::service_factory::setup_service;

mod domain;
mod platform;

#[tokio::main]
async fn main() {
    let config = config().await;
    let pool_sqlx =
        match setup_connection_pool_sqlx(&config).await {
            Ok(poolSql) => { poolSql }
            Err(_) => { panic!("could not create connection pool") }
        };

    let app = setup_service(pool_sqlx);
    axum::Server::bind(&socket_address(&config))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use crate::platform::config::config;
use crate::platform::db::connection_manager::setup_connection_pool;
use crate::platform::service::service_factory::setup_service;
use crate::platform::db::migration::run_db_migrations;
use std::net::SocketAddr;

mod domain;
mod platform;

#[tokio::main]
async fn main() {
    let config = config().await;
    let pool = setup_connection_pool(&config);

    run_db_migrations(&pool).await;

    let app = setup_service(pool);

    let address = format!("{}:{}", config.server_host(), config.server_port());
    let socket_addr: SocketAddr = address.parse().unwrap();
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

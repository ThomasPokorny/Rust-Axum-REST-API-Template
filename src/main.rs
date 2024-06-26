use crate::platform::config::config;
use crate::platform::config::socket_address;
use crate::platform::db::connection_manager::setup_connection_pool;
use crate::platform::db::migration::run_db_migrations;
use crate::platform::service::service_factory::setup_service;

mod domain;
mod platform;

#[tokio::main]
async fn main() {
    let config = config().await;
    let pool = setup_connection_pool(&config);

    run_db_migrations(&pool).await;

    let app = setup_service(pool);

    let listener = tokio::net::TcpListener::bind(&socket_address(&config))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use axum_template::platform::config::config;
use axum_template::platform::db::connection_manager::setup_connection_pool;
use axum_template::platform::db::migration::run_db_migrations;
use axum_template::platform::service::service_factory::setup_service;
use serde::Deserialize;
use std::env;

use http_body_util::BodyExt;
use tower::util::ServiceExt;

use uuid::Uuid;

#[derive(Deserialize)]
struct Tweet {
    pub id: Uuid,
    pub body: String,
}

#[derive(Deserialize)]
struct Tweets {
    pub tweets: Vec<Tweet>,
}

fn set_env_variable() {
    env::set_var(
        "DATABASE_URL",
        "postgres://postgres:password@localhost:15433/axum_template_db",
    );
}

async fn setup_app() -> Router {
    set_env_variable();
    let config = config().await;
    let pool = setup_connection_pool(&config);

    run_db_migrations(&pool).await;

    setup_service(pool)
}

#[tokio::test]
async fn it_adds_two() {
    let app = setup_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/tweets")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let tweets: Tweets = serde_json::from_slice(&body).unwrap();
    assert!(tweets.tweets.len() > 0);
}

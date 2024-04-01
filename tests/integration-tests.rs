use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum_template::platform::config::config;
use axum_template::platform::db::connection_manager::setup_connection_pool;
use axum_template::platform::db::migration::run_db_migrations;
use axum_template::platform::service::service_factory::setup_service;
use serde::Deserialize;

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

#[tokio::test]
async fn it_adds_two() {
    let config = config().await;
    let pool = setup_connection_pool(&config);

    run_db_migrations(&pool).await;

    let app = setup_service(pool);

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
    let s: Tweets = serde_json::from_slice(&body).unwrap();
    //let something: Tweets = serde_json::from_value(s).unwrap();
    assert!(s.tweets.len() > 0);
}

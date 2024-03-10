use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde_json::json;
use std::env;

use crate::domain::tweet::tweet_controller;

pub fn get_app_router() -> Router {
    Router::new()
        .route("/", get(handler_server_status))
        .merge(tweet_controller::router())
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

async fn handler_server_status() -> impl IntoResponse {
    let response = json!({
        "data": {
            "version": env::var("VERSION").unwrap_or_else(|_| String::from("")),
        },
        "message": "Service is running..."
    });
    (StatusCode::OK, Json(response))
}

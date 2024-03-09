use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde_json::json;

pub fn router() -> Router {
    Router::new().route("/api/v1/service/status", get(status))
}

async fn status() -> impl IntoResponse {
    let response = json!({
        "data": {
            "version": "1.0",
        },
        "message": "Service is running..."
    });
    (StatusCode::OK, Json(response))
}

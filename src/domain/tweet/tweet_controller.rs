use crate::domain::tweet::dto::TweetDTO;
use crate::domain::tweet::dto::TweetsDTO;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};

pub fn router() -> Router {
    Router::new().route("/api/v1/tweets", get(get_tweets))
}

async fn get_tweets() -> impl IntoResponse {
    let tweets = vec![
        TweetDTO {
            id: 1,
            body: String::from("Tweet 1"),
        },
        TweetDTO {
            id: 2,
            body: String::from("Tweet 2"),
        },
        TweetDTO {
            id: 3,
            body: String::from("Tweet 3"),
        },
    ];
    (StatusCode::OK, Json(TweetsDTO { tweets }))
}

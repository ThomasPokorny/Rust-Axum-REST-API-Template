use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::routing::{delete, put};
use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router};
use uuid::Uuid;

use crate::domain::tweet::dto::TweetDTO;
use crate::domain::tweet::dto::{to_dto_list, CreateUpdateTweetDTO};
use crate::domain::tweet::tweet_service::TweetService;
use crate::domain::tweet::tweet_service::TweetServiceTrait;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/tweets", get(get_tweets))
        .route("/api/v1/tweets/:tweet_id", get(get_tweet))
        .route("/api/v1/tweets/:tweet_id", put(update_tweet))
        .route("/api/v1/tweets/:tweet_id", delete(delete_tweet))
        .route("/api/v1/tweets", post(create_tweet))
}

async fn get_tweets(Extension(tweet_service): Extension<Arc<TweetService>>) -> impl IntoResponse {
    let tweets = tweet_service.get_tweets().await;
    (StatusCode::OK, Json(to_dto_list(tweets.unwrap())))
}

async fn get_tweet(
    Extension(tweet_service): Extension<Arc<TweetService>>,
    Path(tweet_id): Path<Uuid>,
) -> impl IntoResponse {
    let tweet = tweet_service.get_tweet(tweet_id).await;
    (StatusCode::OK, Json(TweetDTO::from(tweet.unwrap())))
}

async fn create_tweet(
    Extension(tweet_service): Extension<Arc<TweetService>>,
    Json(create_tweet): Json<CreateUpdateTweetDTO>,
) -> impl IntoResponse {
    let tweet = tweet_service.create_tweet(create_tweet.into()).await;
    (StatusCode::OK, Json(TweetDTO::from(tweet.unwrap())))
}

async fn update_tweet(
    Extension(tweet_service): Extension<Arc<TweetService>>,
    Path(tweet_id): Path<Uuid>,
    Json(create_tweet): Json<CreateUpdateTweetDTO>,
) -> impl IntoResponse {
    let tweet = tweet_service
        .update_tweet(tweet_id, create_tweet.into())
        .await;
    (StatusCode::OK, Json(TweetDTO::from(tweet.unwrap())))
}

async fn delete_tweet(
    Extension(tweet_service): Extension<Arc<TweetService>>,
    Path(tweet_id): Path<Uuid>,
) -> impl IntoResponse {
    let status = match tweet_service.delete_tweet(tweet_id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    return status;
}

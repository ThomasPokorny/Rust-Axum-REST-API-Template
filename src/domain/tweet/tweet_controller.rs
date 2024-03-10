use std::sync::Arc;

use axum::{http::StatusCode, Json, response::IntoResponse, Router, routing::get, routing::post};
use axum::extract::{Extension, Path};
use axum::routing::put;
use uuid::Uuid;

use crate::domain::tweet::dto::CreateUpdateTweetDTO;
use crate::domain::tweet::dto::TweetDTO;
use crate::domain::tweet::dto::TweetsDTO;
use crate::domain::tweet::model::CreateUpdateTweet;
use crate::domain::tweet::model::Tweet;
use crate::domain::tweet::tweet_service::TweetService;
use crate::domain::tweet::tweet_service::TweetServiceTrait;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/tweets", get(get_tweets))
        .route("/api/v1/tweets/:tweet_id", get(get_tweet))
        .route("/api/v1/tweets/:tweet_id", put(update_tweet))
        .route("/api/v1/tweets", post(create_tweet))
}

async fn get_tweets(Extension(tweet_service): Extension<Arc<TweetService>>) -> impl IntoResponse {
    let tweets = tweet_service.get_tweets().await;
    (
        StatusCode::OK,
        Json(adapt_tweets_to_list_tweets_dto(tweets.unwrap())),
    )
}

async fn get_tweet(
    Extension(tweet_service): Extension<Arc<TweetService>>,
    Path(tweet_id): Path<Uuid>,
) -> impl IntoResponse {
    let tweet = tweet_service.get_tweet(tweet_id).await;
    (
        StatusCode::OK,
        Json(adapt_tweet_to_tweet_dto(tweet.unwrap())),
    )
}

async fn create_tweet(
    Extension(tweet_service): Extension<Arc<TweetService>>,
    Json(create_tweet): Json<CreateUpdateTweetDTO>,
) -> impl IntoResponse {
    let tweet = tweet_service.create_tweet(to_domain(create_tweet)).await;
    (
        StatusCode::OK,
        Json(adapt_tweet_to_tweet_dto(tweet.unwrap())),
    )
}

async fn update_tweet(
    Extension(tweet_service): Extension<Arc<TweetService>>,
    Path(tweet_id): Path<Uuid>,
    Json(create_tweet): Json<CreateUpdateTweetDTO>,
) -> impl IntoResponse {
    let tweet = tweet_service.update_tweet(tweet_id, to_domain(create_tweet)).await;
    (
        StatusCode::OK,
        Json(adapt_tweet_to_tweet_dto(tweet.unwrap())),
    )
}

fn to_domain(dto: CreateUpdateTweetDTO) -> CreateUpdateTweet {
    CreateUpdateTweet { body: dto.body }
}

fn adapt_tweet_to_tweet_dto(tweet: Tweet) -> TweetDTO {
    TweetDTO {
        id: tweet.id,
        body: tweet.body,
    }
}

fn adapt_tweets_to_list_tweets_dto(tweets: Vec<Tweet>) -> TweetsDTO {
    let tweets_response: Vec<TweetDTO> = tweets.into_iter().map(adapt_tweet_to_tweet_dto).collect();

    TweetsDTO {
        tweets: tweets_response,
    }
}

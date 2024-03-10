use crate::domain::tweet::dto::TweetDTO;
use crate::domain::tweet::dto::TweetsDTO;
use crate::domain::tweet::tweet_repository::Tweet;
use crate::domain::tweet::tweet_service::TweetService;
use crate::domain::tweet::tweet_service::TweetServiceTrait;
use axum::extract::Extension;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;

pub fn router() -> Router {
    Router::new().route("/api/v1/tweets", get(get_tweets))
}

async fn get_tweets(Extension(tweet_service): Extension<Arc<TweetService>>) -> impl IntoResponse {
    let tweets = tweet_service.get_tweets().await;
    (
        StatusCode::OK,
        Json(adapt_tweets_to_list_tweets_dto(tweets.unwrap())),
    )
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

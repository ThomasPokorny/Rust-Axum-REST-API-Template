use crate::domain::tweet::dto::TweetDTO;
use crate::domain::tweet::dto::TweetsDTO;
use crate::domain::tweet::dto::CreateTweetDTO;
use crate::domain::tweet::tweet_service::TweetService;
use crate::domain::tweet::tweet_service::TweetServiceTrait;
use crate::domain::tweet::model::Tweet;
use crate::domain::tweet::model::CreateUpdateTweet;
use axum::extract::Extension;
use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router};
use std::sync::Arc;

pub fn router() -> Router {
    Router::new()
    .route("/api/v1/tweets", get(get_tweets))
    .route("/api/v1/tweets", post(create_tweet))
}

async fn get_tweets(Extension(tweet_service): Extension<Arc<TweetService>>) -> impl IntoResponse {
    let tweets = tweet_service.get_tweets().await;
    (
        StatusCode::OK,
        Json(adapt_tweets_to_list_tweets_dto(tweets.unwrap())),
    )
}

async fn create_tweet(Extension(tweet_service): Extension<Arc<TweetService>>, Json(create_tweet): Json<CreateTweetDTO>) -> impl IntoResponse {
    let tweet = tweet_service.create_tweet(to_domain(create_tweet)).await;
    (
        StatusCode::OK,
        Json(adapt_tweet_to_tweet_dto(tweet.unwrap())),
    )
}

fn to_domain(dto: CreateTweetDTO) -> CreateUpdateTweet {
    CreateUpdateTweet {
        body: dto.body,
    }
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

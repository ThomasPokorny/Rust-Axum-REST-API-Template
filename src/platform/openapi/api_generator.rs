use crate::domain::tweet::dto::TweetDTO;
use crate::domain::tweet::dto::TweetsDTO;
use crate::domain::tweet::dto::CreateUpdateTweetDTO;
use crate::domain::tweet::tweet_controller;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        tweet_controller::get_tweets,
        tweet_controller::get_tweet,
        tweet_controller::create_tweet,
        tweet_controller::update_tweet,
        tweet_controller::delete_tweet
    ),
    components(schemas(TweetsDTO, TweetDTO, CreateUpdateTweetDTO)))
]
struct ApiDoc;

fn get_openapi_json() -> String {
    return ApiDoc::openapi().to_pretty_json().unwrap();
}

pub async fn handler_openapi_docs() -> impl IntoResponse {
    (StatusCode::OK, get_openapi_json())
}

use deadpool_diesel::postgres::{Pool};
use std::sync::Arc;
use axum::Router;
use axum::Extension;

use crate::platform::router::handler::get_app_router;
use crate::domain::tweet::tweet_repository::TweetRepository;
use crate::domain::tweet::tweet_service::TweetService;


pub fn setup_service(pool: Pool) -> Router {
    let tweet_repository = Arc::new(TweetRepository::new(Arc::new(pool)));
    let tweet_service = Arc::new(TweetService::new(tweet_repository));

    get_app_router()
        .layer(Extension(tweet_service))
}
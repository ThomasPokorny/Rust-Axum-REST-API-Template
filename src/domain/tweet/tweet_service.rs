// tweet_service.rs
//use std::sync::Arc;
use crate::domain::tweet::tweet_repository::Tweet;
use crate::domain::tweet::tweet_repository::TweetRepository;
use diesel::result::Error;
use std::sync::Arc;
//use crate::repository::TweetRepository;
//use crate::models::Tweet;

pub trait TweetServiceTrait {
    async fn get_tweets(&self) -> Result<Vec<Tweet>, Error>;
}

pub struct TweetService {
    repository: Arc<TweetRepository>,
}

impl TweetService {
    pub fn new(repository: Arc<TweetRepository>) -> Self {
        Self { repository }
    }
}

impl TweetServiceTrait for TweetService {
    async fn get_tweets(&self) -> Result<Vec<Tweet>, Error> {
        self.repository.get_all().await
    }
}

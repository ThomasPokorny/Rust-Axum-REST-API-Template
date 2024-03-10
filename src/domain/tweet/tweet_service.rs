use crate::domain::tweet::model::Tweet;
use crate::domain::tweet::model::CreateUpdateTweet;
use crate::domain::tweet::tweet_repository::TweetRepository;
use diesel::result::Error;
use std::sync::Arc;

pub trait TweetServiceTrait {
    async fn get_tweets(&self) -> Result<Vec<Tweet>, Error>;
    async fn create_tweet(&self, create_tweet: CreateUpdateTweet) -> Result<Tweet, Error>;
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

    async fn create_tweet(&self, create_tweet: CreateUpdateTweet) -> Result<Tweet, Error> {
        self.repository.save(create_tweet).await
    }
}

use std::sync::Arc;

use diesel::result::Error;
use uuid::Uuid;

use crate::domain::tweet::model::CreateUpdateTweet;
use crate::domain::tweet::model::Tweet;
use crate::domain::tweet::tweet_repository::TweetRepository;

pub trait TweetServiceTrait {
    async fn get_tweets(&self) -> Result<Vec<Tweet>, Error>;
    async fn get_tweet(&self, tweet_id: Uuid) -> Result<Tweet, Error>;
    async fn create_tweet(&self, create_tweet: CreateUpdateTweet) -> Result<Tweet, Error>;
    async fn update_tweet(
        &self,
        tweet_id: Uuid,
        create_tweet: CreateUpdateTweet,
    ) -> Result<Tweet, Error>;
    async fn delete_tweet(&self, tweet_id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
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

    async fn get_tweet(&self, tweet_id: Uuid) -> Result<Tweet, Error> {
        self.repository.get_by_id(tweet_id).await
    }

    async fn create_tweet(&self, create_tweet: CreateUpdateTweet) -> Result<Tweet, Error> {
        self.repository.save(create_tweet).await
    }

    async fn update_tweet(
        &self,
        tweet_id: Uuid,
        create_tweet: CreateUpdateTweet,
    ) -> Result<Tweet, Error> {
        self.repository.update(tweet_id, create_tweet).await
    }

    async fn delete_tweet(&self, tweet_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        match self.repository.delete(tweet_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

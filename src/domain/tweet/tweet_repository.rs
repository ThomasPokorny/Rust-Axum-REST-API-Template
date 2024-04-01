use std::sync::Arc;

use sqlx::PgPool;

use crate::domain::tweet::model::Tweet;

pub struct TweetRepository {
    pool: Arc<PgPool>,
}

impl TweetRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Tweet>, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;

        let tweets = sqlx::query_as::<_, Tweet>("SELECT * FROM TWEET")
            .fetch_all(&mut *conn)
            .await?;

        Ok(tweets)
    }
    /*pub async fn get_by_id(&self, tweet_id: Uuid) -> Result<Tweet, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;

        let tweet: Tweet = sqlx::query_as::<_, Tweet>("SELECT * FROM tweets WHERE id = $1")
            .bind(tweet_id)
            .fetch_one(&mut *conn)
            .await?;

        Ok(tweet)
    }

    pub async fn save(
        &self,
        create_tweet: CreateUpdateTweet,
    ) -> Result<Tweet, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;

        let tweet = sqlx::query_as::<_, Tweet>(
            "INSERT INTO tweets (body) VALUES ($1) RETURNING *"
        )
            .bind(create_tweet.body)
            .fetch_one(&mut *conn)
            .await?;

        Ok(tweet)
    }

    pub async fn update(
        &self,
        tweet_id: Uuid,
        create_tweet: CreateUpdateTweet,
    ) -> Result<Tweet, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;

        let tweet = sqlx::query_as::<_, Tweet>(
            "UPDATE tweets SET body = $1 WHERE id = $2 RETURNING *"
        )
            .bind(create_tweet.body)
            .bind(tweet_id)
            .fetch_one(&mut *conn)
            .await?;

        Ok(tweet)
    }

    pub async fn delete(
        &self,
        tweet_id: Uuid,
    ) -> Result<u64, sqlx::Error()> {
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query(
            "DELETE FROM tweets WHERE id = $1"
        )
            .bind(tweet_id)
            .execute(&mut *conn)
            .await?;

        Ok(result.rows_affected())
    }*/
}

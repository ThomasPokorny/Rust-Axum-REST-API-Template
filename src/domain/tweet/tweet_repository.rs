use std::sync::Arc;

use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::db::schema::tweet;
use crate::domain::db::schema::tweet::dsl::*;
use crate::domain::tweet::model::CreateUpdateTweet;
use crate::domain::tweet::model::Tweet;

pub struct TweetRepository {
    pool: Arc<deadpool_diesel::postgres::Pool>,
}

impl TweetRepository {
    pub fn new(pool: Arc<deadpool_diesel::postgres::Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Tweet>, diesel::result::Error> {
        let conn = self.pool.get().await.unwrap();

        conn.interact(move |conn| tweet.load::<Tweet>(conn))
            .await
            .unwrap()
    }

    pub async fn get_by_id(&self, tweet_id: Uuid) -> Result<Tweet, diesel::result::Error> {
        let conn = self.pool.get().await.unwrap();

        conn
            .interact(move |conn| {
                tweet::table
                    .filter(id.eq(tweet_id))
                    .select(Tweet::as_select())
                    .get_result(conn)
            })
            .await
            .unwrap()
    }

    pub async fn save(
        &self,
        create_tweet: CreateUpdateTweet,
    ) -> Result<Tweet, diesel::result::Error> {
        let conn = self.pool.get().await.unwrap();
        conn.interact(|conn| {
            diesel::insert_into(tweet::table)
                .values(create_tweet)
                .returning(Tweet::as_returning())
                .get_result(conn)
        })
            .await
            .unwrap()
    }

    pub async fn update(
        &self,
        tweet_id: Uuid,
        create_tweet: CreateUpdateTweet,
    ) -> Result<Tweet, diesel::result::Error> {
        let conn = self.pool.get().await.unwrap();

        conn.interact(move |conn| {
            diesel::update(tweet::table)
                .filter(id.eq(tweet_id))
                .set(create_tweet)
                .returning(Tweet::as_returning())
                .get_result(conn)
        })
            .await
            .unwrap()
    }
}

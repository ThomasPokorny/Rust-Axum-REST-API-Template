use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

// Define the Tweet struct corresponding to the tweet table
#[derive(Debug, Queryable)]
pub struct Tweet {
    pub id: Uuid,
    pub body: String,
}

pub struct TweetRepository {
    pool: Arc<deadpool_diesel::postgres::Pool>,
}

impl TweetRepository {
    pub fn new(pool: Arc<deadpool_diesel::postgres::Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Tweet>, diesel::result::Error> {
        let conn = self.pool.get().await.unwrap();

        conn.interact(move |conn| {
            use crate::domain::db::schema::tweet::dsl::*;

            tweet.load::<Tweet>(conn)
        })
        .await
        .unwrap()
    }
}

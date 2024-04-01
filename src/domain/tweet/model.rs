use diesel::prelude::*;
use serde::Deserialize;
use sqlx::types::Uuid;

use crate::domain::db::schema::tweet;

#[derive(Debug, Queryable, Selectable, sqlx::FromRow)]
#[diesel(table_name = tweet)]
pub struct Tweet {
    pub id: Uuid,
    pub body: String,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = tweet)]
pub struct CreateUpdateTweet {
    pub body: String,
}

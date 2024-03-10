use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::db::schema::tweet;

#[derive(Debug, Queryable, Selectable)]
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

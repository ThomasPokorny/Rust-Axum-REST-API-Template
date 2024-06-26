use o2o::o2o;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::tweet::model::CreateUpdateTweet;
use crate::domain::tweet::model::Tweet;

#[derive(Serialize, o2o, ToSchema)]
#[serde(rename_all = "camelCase")]
#[from_owned(Tweet)]
pub struct TweetDTO {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub body: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TweetsDTO {
    pub tweets: Vec<TweetDTO>,
}

#[derive(Deserialize, o2o, ToSchema)]
#[serde(rename_all = "camelCase")]
#[owned_into(CreateUpdateTweet)]
pub struct CreateUpdateTweetDTO {
    pub body: String,
}

pub fn to_dto_list(tweets: Vec<Tweet>) -> TweetsDTO {
    let tweets: Vec<TweetDTO> = tweets.into_iter().map(TweetDTO::from).collect();
    TweetsDTO { tweets }
}

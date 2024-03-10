use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetDTO {
    pub id: Uuid,
    pub body: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetsDTO {
    pub tweets: Vec<TweetDTO>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateTweetDTO {
    pub body: String,
}

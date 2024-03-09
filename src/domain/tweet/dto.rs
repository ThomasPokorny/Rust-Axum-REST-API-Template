use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetDTO {
    pub id: i64,
    pub body: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetsDTO {
    pub tweets: Vec<TweetDTO>,
}

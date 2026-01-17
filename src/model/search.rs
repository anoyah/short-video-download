use std::fmt;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchModel {
    pub code: i64,
    pub msg: String,
    pub data: Vec<SearchData>,
    pub page: String,
    pub tips: String,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchData {
    #[serde(rename = "book_id")]
    pub book_id: String,
    pub title: String,
    pub author: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "play_cnt")]
    pub play_cnt: i64,
    #[serde(rename = "episode_cnt")]
    pub episode_cnt: i64,
    pub cover: String,
    pub intro: String,
}

impl fmt::Display for SearchData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

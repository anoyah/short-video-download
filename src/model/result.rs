use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultModel {
    pub code: i64,
    pub msg: String,
    pub data: Vec<ResultData>,
    pub total: String,
    #[serde(rename = "book_id")]
    pub book_id: String,
    #[serde(rename = "book_name")]
    pub book_name: String,
    pub author: String,
    pub category: String,
    pub desc: String,
    pub duration: String,
    #[serde(rename = "book_pic")]
    pub book_pic: String,
    pub tips: String,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultData {
    #[serde(rename = "video_id")]
    pub video_id: String,
    pub title: String,
    pub first_pass_time: String,
    #[serde(rename = "volume_name")]
    pub volume_name: String,
    #[serde(rename = "chapter_word_number")]
    pub chapter_word_number: i64,
}

impl fmt::Display for ResultData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}




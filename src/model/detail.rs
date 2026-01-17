use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailModel {
    pub code: i64,
    pub msg: String,
    pub data: DetailData,
    pub tips: String,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailData {
    pub title: String,
    pub pic: String,
    pub url: String,
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub quality: String,
    pub fps: i64,
    pub bitrate: String,
    pub codec: String,
    pub duration: String,
    pub size: i64,
    #[serde(rename = "size_str")]
    pub size_str: String,
    pub height: i64,
    pub width: i64,
}

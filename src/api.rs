use anyhow::{Result, anyhow};

use crate::{
    http::HttpClient,
    model::{
        detail::{DetailData, DetailModel},
        result::{ResultData, ResultModel},
        search::{SearchData, SearchModel},
    },
};

#[derive(Clone)]
pub struct Api {
    client: HttpClient,
}

impl Api {
    pub fn new() -> Self {
        let client = HttpClient::new(10).set_base_api("https://api.cenguigui.cn".to_string());
        Api { client }
    }

    pub async fn search(&self, title: &str, page: i8) -> Result<Vec<SearchData>> {
        let url = format!("/api/duanju/api.php?name={title}&page={page}&showRawParams=false");
        let response: SearchModel = self.client.get_json(url.as_str()).await?;
        if response.code != 200 {
            return Err(anyhow!("API returned error code: {}", response.code));
        }
        Ok(response.data)
    }

    pub async fn get_with_book_id(&self, book_id: &str) -> Result<Vec<ResultData>> {
        let url = format!("/api/duanju/api.php?book_id={book_id}&showRawParams=false");
        let response: ResultModel = self.client.get_json(url.as_str()).await?;

        if response.code != 200 {
            return Err(anyhow!("API returned error code: {}", response.code));
        }
        Ok(response.data)
    }

    pub async fn get_with_video_id(&self, video_id: &str) -> Result<DetailData> {
        let url = format!("/api/duanju/api.php?video_id={video_id}&type=json&showRawParams=false");
        let response: DetailModel = self.client.get_json(url.as_str()).await?;
        if response.code != 200 {
            return Err(anyhow!("API returned error code: {}", response.code));
        }
        Ok(response.data)
    }

}

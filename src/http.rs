use reqwest::{Client, Error};
use serde::de::DeserializeOwned;
use std::time::Duration;

#[derive(Clone)]
pub struct HttpClient {
    base_api: String,
    client: Client,
}

impl HttpClient {
    pub fn new(timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .unwrap();

        HttpClient {
            base_api: "".to_string(),
            client,
        }
    }

    pub fn set_base_api(mut self, base_api: String) -> Self {
        self.base_api = base_api;
        return self;
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let resp = self.client.get(self.build_url(url)).send().await?;
        resp.json::<T>().await
    }

    fn build_url(&self, path: &str) -> String {
        return format!("{}{}", self.base_api, path);
    }

}

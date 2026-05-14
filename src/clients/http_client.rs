use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl HttpClient {
    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, reqwest::Error> {
        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await
    }
}

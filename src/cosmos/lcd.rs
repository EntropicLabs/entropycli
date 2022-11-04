use super::network::Network;

impl Network {
    pub async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::get(&url).await
    }

    pub async fn post(&self, path: &str, body: &serde_json::Value) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::Client::new()
            .post(&url)
            .json(&body)
            .send()
            .await
    }
}
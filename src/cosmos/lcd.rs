use super::network::Network;

impl Network {
    pub async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(10))
            .build()?
            .get(&url)
            .send()
            .await
    }

    pub async fn post(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(10))
            .build()?
            .post(&url)
            .json(body)
            .send()
            .await
    }
}

use super::network::Network;

impl Network {
    pub async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        let timeout = std::env::var("LCD_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .unwrap_or(30);
        reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(timeout))
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
        let timeout = std::env::var("LCD_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .unwrap_or(30);
        reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(timeout))
            .build()?
            .post(&url)
            .json(body)
            .send()
            .await
    }
}

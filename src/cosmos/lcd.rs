use super::network::Network;

impl Network {
    pub fn get_blocking(&self, path: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::blocking::get(&url)
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::get(&url).await
    }

    pub fn post_blocking(&self, path: &str, body: &serde_json::Value) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::blocking::Client::new()
            .post(&url)
            .json(&body)
            .send()
    }

    pub async fn post(&self, path: &str, body: &serde_json::Value) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::Client::new()
            .post(&url)
            .json(&body)
            .send()
            .await
    }
    
    pub async fn broadcast(&self, tx: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/txs", self.lcd_url);
        reqwest::Client::new()
            .post(&url)
            .json(&serde_json::json!({
                "tx": tx,
                "mode": "block"
            }))
            .send()
            .await
    }
}
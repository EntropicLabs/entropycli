use super::network::Network;

impl Network {
    #[allow(dead_code)]
    pub fn get_blocking(&self, path: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::blocking::get(&url)
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.lcd_url, path);
        reqwest::get(&url).await
    }

    #[allow(dead_code)]
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
}
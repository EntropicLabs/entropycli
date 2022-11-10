use serde_json::json;

pub async fn info(url: impl Into<String> + Clone, message: impl Into<String> + Clone) -> Result<reqwest::Response, reqwest::Error> {
    let url = url.clone().into();
    let message = message.clone().into();
    let body = json!({
        "content": message,
    });
    reqwest::Client::new()
            .post(&url)
            .json(&body)
            .send()
            .await
}

pub async fn error(url: impl Into<String>, message: impl Into<String>) -> Result<reqwest::Response, reqwest::Error> {
    let url = url.into();
    let message = message.into();
    let body = json!({
        "content": format!("@here {}", message)
    });
    reqwest::Client::new()
            .post(&url)
            .json(&body)
            .send()
            .await
}
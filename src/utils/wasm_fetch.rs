use reqwest::Client;
use std::{io::Cursor, path::PathBuf};
use thiserror::Error;

pub const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/EntropicLabs/mock_beacon/releases/latest";

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("Failed to fetch latest release info")]
    ReleaseInfo(reqwest::Error),
    #[error("Failed to parse latest release info")]
    ResponseJSON(reqwest::Error),
    #[error("Failed to parse latest release info")]
    ParseJSON(),
    #[error("IO error")]
    IO(std::io::Error),
    #[error("Failed to download latest release")]
    Download(reqwest::Error),
}

pub async fn fetch_release_url() -> Result<String, FetchError> {
    let client = Client::new();

    let response = client
        .get(LATEST_RELEASE_URL)
        .header("User-Agent", "entropycli")
        .send()
        .await
        .map_err(FetchError::ReleaseInfo)?;
    let json: serde_json::Value = response.json().await.map_err(FetchError::ResponseJSON)?;
    let wasm_download_url = json["assets"][0]["browser_download_url"]
        .as_str()
        .ok_or(FetchError::ParseJSON())?;
    Ok(wasm_download_url.to_string())
}

pub async fn download_file(url: String, path: PathBuf) -> Result<PathBuf, FetchError> {
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(FetchError::Download)?;
    let mut file = tokio::fs::File::create(path.clone())
        .await
        .map_err(FetchError::IO)?;
    let mut content = Cursor::new(response.bytes().await.map_err(FetchError::Download)?);
    tokio::io::copy(&mut content, &mut file)
        .await
        .map_err(FetchError::IO)?;
    Ok(path)
}

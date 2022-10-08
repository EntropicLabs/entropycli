use std::path::PathBuf;
use thiserror::Error;

use reqwest::blocking::Client;
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

pub fn fetch_release_url() -> Result<String, FetchError> {
    let client = Client::new();

    let response = client
        .get(LATEST_RELEASE_URL)
        .header("User-Agent", "entropycli")
        .send()
        .map_err(FetchError::ReleaseInfo)?;
    let json: serde_json::Value = response.json().map_err(FetchError::ResponseJSON)?;
    let wasm_download_url = json["assets"][0]["browser_download_url"]
        .as_str()
        .ok_or(FetchError::ParseJSON())?;
    Ok(wasm_download_url.to_string())
}

pub fn download_file(url: String, path: PathBuf) -> Result<PathBuf, FetchError> {
    let client = Client::new();
    let mut response = client.get(&url).send().map_err(FetchError::Download)?;
    let mut file = std::fs::File::create(path.clone()).map_err(FetchError::IO)?;
    std::io::copy(&mut response, &mut file).map_err(FetchError::IO)?;
    Ok(path)
}
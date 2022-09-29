use reqwest::blocking::Client;
pub const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/EntropicLabs/mock_beacon/releases/latest";

pub enum FetchError {
    Reqwest(reqwest::Error),
    Json(reqwest::Error),
}

pub fn get_latest_release() -> Result<bool, FetchError> {
    let client = Client::new();

    let response = client
        .get(LATEST_RELEASE_URL)
        .header("User-Agent", "entropycli")
        .send()
        .map_err(FetchError::Reqwest)?;
    let json: serde_json::Value = response.json().map_err(FetchError::Json)?;
    let wasm_download_url = json["assets"][0]["browser_download_url"]
        .as_str()
        .unwrap();
    println!("{:#?}", wasm_download_url);
    Ok(true)
}

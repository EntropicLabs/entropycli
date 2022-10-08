use super::wallet::Wallet;

use thiserror::Error;
#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Network Error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Cosmrs Error: {0}")]
    CosmrsError(#[from] cosmrs::ErrorReport),
    #[error("Account \"{0}\" not found")]
    AccountNotFound(String),
    #[error("Error parsing response: {0}")]
    ParseError(String),
}

impl Wallet {
    pub async fn account_number_and_sequence(&self) -> Result<(u64, u64), QueryError> {
        let path = format!("cosmos/auth/v1beta1/accounts/{}", self.address);
        let response = self.network.get(&path).await?;

        let json: serde_json::Value = response.json().await?;
        let account_number = json["account"]["account_number"]
            .as_u64()
            .ok_or_else(|| QueryError::AccountNotFound(self.address.to_string()))?;
        let sequence = json["account"]["sequence"]
            .as_u64()
            .ok_or_else(|| QueryError::AccountNotFound(self.address.to_string()))?;

        Ok((account_number, sequence))
    }

    pub async fn block_height(&self) -> Result<u32, QueryError> {
        let response = self.network.get("blocks/latest").await?;
        let json: serde_json::Value = response.json().await?;
        let height = json["block"]["header"]["height"]
            .as_u64()
            .ok_or_else(|| QueryError::ParseError("Failed to parse block height".to_string()))?;

        Ok(height as u32)
    }
}

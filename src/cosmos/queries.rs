use super::{network::Network, response::TxResponse, tx::TxError, wallet::Wallet};

use serde::Serialize;
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
            .as_str()
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|e| QueryError::ParseError(e.to_string()))
            })
            .transpose()?
            .ok_or_else(|| QueryError::AccountNotFound(self.address.to_string()))?;
        let sequence = json["account"]["sequence"]
            .as_str()
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|e| QueryError::ParseError(e.to_string()))
            })
            .transpose()?
            .ok_or_else(|| QueryError::AccountNotFound(self.address.to_string()))?;

        Ok((account_number, sequence))
    }

    pub async fn block_height(&self) -> Result<u32, QueryError> {
        let response = self.network.get("cosmos/base/tendermint/v1beta1/blocks/latest").await?;
        let json: serde_json::Value = response.json().await?;

        let height = json["block"]["header"]["height"]
            .as_str()
            .map(|s| {
                s.parse::<u32>()
                    .map_err(|e| QueryError::ParseError(e.to_string()))
            })
            .transpose()?
            .ok_or_else(|| QueryError::ParseError("Failed to parse block height".to_string()))?;

        Ok(height)
    }

    pub async fn wait_for_hash(&self, tx_hash: String) -> Result<TxResponse, TxError> {
        for _ in 0..60 {
            let res = self
                .network
                .get(&format!("cosmos/tx/v1beta1/txs/{}", tx_hash))
                .await
                .map_err(QueryError::NetworkError)?;

            let res = res.json::<serde_json::Value>().await?;

            if res["code"].as_u64().map_or(false, |c| c == 5) {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                continue;
            }

            let res = serde_json::from_value::<TxResponse>(res["tx_response"].clone())
                .map_err(|e| QueryError::ParseError(e.to_string()))?;

            if res.code != 0 {
                return Err(TxError::TxFailed(res));
            }
            return Ok(res);
        }
        Err(TxError::Timeout)
    }
}

impl Network {
    pub async fn query(
        &self,
        address: String,
        query: impl Serialize,
    ) -> Result<serde_json::Value, QueryError> {
        let path = format!(
            "cosmwasm/wasm/v1/contract/{address}/smart/{query_data}",
            address = address,
            query_data = base64::encode(
                serde_json::to_string(&query).map_err(|e| QueryError::ParseError(e.to_string()))?
            )
        );
        let response = self.get(&path).await?;
        let json: serde_json::Value = response.json().await?;
        if json["code"].as_u64().is_some() {
            return Err(QueryError::ParseError(format!(
                "{}. Request: {}",
                json["message"],
                path
            )));
        }

        Ok(json["data"].clone())
    }
}

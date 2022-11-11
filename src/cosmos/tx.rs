use cosmrs::{tx::{
    mode_info::Single, AuthInfo, Body, Fee, Gas, ModeInfo, Msg, SignDoc, SignMode, SignerInfo,
}, AccountId};

use serde_json::json;

use super::{queries::QueryError, response::TxResponse, utils::mul_gas_float, wallet::Wallet};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TxError {
    // Parse can be from any type of error:
    #[error("Failed to parse transaction: {0}")]
    Parse(String),
    #[error("Failed to broadcast transaction: {0}")]
    Broadcast(#[from] reqwest::Error),
    #[error("{0}")]
    Query(#[from] super::queries::QueryError),
    #[error("ChainID {0} unparsable")]
    ChainID(String),
    #[error("Out of gas")]
    OutOfGas,
    #[error("Transaction failed {:?}", .0)]
    TxFailed(TxResponse),
}

pub const HEIGHT_TIMEOUT_INTERVAL: u32 = 10;

impl Wallet {
    pub async fn broadcast_msg<M>(&self, msg: M, gas: Option<Gas>, granter: Option<AccountId>) -> Result<String, TxError>
    where
        M: Msg,
    {
        let block_height = self.block_height().await?;

        let (acc_num, seq) = self.account_number_and_sequence().await?;

        let body = Body::new(
            vec![msg.to_any().map_err(|e| TxError::Parse(e.to_string()))?],
            String::new(),
            block_height + HEIGHT_TIMEOUT_INTERVAL,
        );

        let gas = match gas {
            Some(gas) => gas,
            None => self.estimate_gas(msg).await?,
        };

        let mut fee = self
            .network
            .gas_info
            .gas_to_fee(gas)
            .map_err(|e| TxError::Parse(e.to_string()))?;

        fee.granter = granter;

        let auth_info = SignerInfo::single_direct(Some(self.pubkey), seq).auth_info(fee);

        let sign_doc = SignDoc::new(&body, &auth_info, &self.network.chain_id, acc_num)
            .map_err(|e| TxError::Parse(e.to_string()))?;

        let tx_raw = sign_doc
            .sign(&self.signing_key())
            .map_err(|e| TxError::Parse(e.to_string()))?
            .to_bytes()
            .map_err(|e| TxError::Parse(e.to_string()))?;

        let tx_raw = base64::encode(tx_raw);

        let res = self
            .network
            .post(
                "cosmos/tx/v1beta1/txs",
                &json!({
                    "tx_bytes": tx_raw,
                    "mode": "BROADCAST_MODE_SYNC",
                }),
            )
            .await
            .map_err(QueryError::NetworkError)?;

        let res = res.json::<serde_json::Value>().await?;

        if res["tx_response"]["code"]
            .as_u64()
            .map_or(false, |c| c == 11)
        {
            return Err(TxError::OutOfGas);
        }

        let tx_hash = res["tx_response"]["txhash"]
            .as_str()
            .ok_or_else(|| TxError::Parse("Error parsing txhash, unexpected response".to_string()))?
            .to_string();
        Ok(tx_hash)
    }

    pub fn single_unspecified_signer_auth(&self, sequence_number: u64) -> AuthInfo {
        SignerInfo {
            public_key: Some(self.pubkey.into()),
            mode_info: ModeInfo::Single(Single {
                mode: SignMode::Unspecified,
            }),
            sequence: sequence_number,
        }
        .auth_info(Fee {
            amount: vec![],
            gas_limit: Gas::default(),
            payer: None,
            granter: None,
        })
    }

    pub async fn estimate_gas<M>(&self, msg: M) -> Result<Gas, QueryError>
    where
        M: Msg,
    {
        let block_height = self.block_height().await?;

        let (acc_num, seq) = self.account_number_and_sequence().await?;

        let body = Body::new(
            vec![msg.to_any()?],
            String::new(),
            block_height + HEIGHT_TIMEOUT_INTERVAL,
        );

        let auth_info = self.single_unspecified_signer_auth(seq);

        let sign_doc = SignDoc::new(&body, &auth_info, &self.network.chain_id, acc_num)?;

        let tx_raw = sign_doc.sign(&self.signing_key())?.to_bytes()?;
        let tx_raw = base64::encode(tx_raw);

        let res = self
            .network
            .post(
                "cosmos/tx/v1beta1/simulate",
                &json!({
                    "tx_bytes": tx_raw,
                }),
            )
            .await
            .map_err(QueryError::NetworkError)?;

        let res = res.json::<serde_json::Value>().await?;

        let gas = res["gas_info"]["gas_used"]
            .as_str()
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|e| QueryError::ParseError(e.to_string()))
            })
            .transpose()?
            .ok_or_else(|| QueryError::AccountNotFound(self.address.to_string()))?;

        let gas = mul_gas_float(gas, self.network.gas_info.gas_adjustment);

        Ok(gas)
    }
}

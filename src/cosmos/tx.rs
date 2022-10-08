use cosmrs::{
    tx::{Body, Msg, SignDoc, SignerInfo, ModeInfo, mode_info::Single, SignMode, AuthInfo, Fee},
    ErrorReport, Tx,
};

use super::wallet::Wallet;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TxError {
    #[error("Failed to parse transaction: {0}")]
    ParseError(#[from] ErrorReport),
    #[error("Failed to broadcast transaction: {0}")]
    BroadcastError(#[from] reqwest::Error),
    #[error("{0}")]
    QueryError(#[from] super::queries::QueryError),
    #[error("ChainID {0} unparsable")]
    ChainIDError(String),
}

pub const HEIGHT_TIMEOUT_INTERVAL: u32 = 10;

impl Wallet {
    pub async fn create_and_sign<M>(&self, msg: M) -> Result<(), TxError>
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

        let auth_info = SignerInfo::single_direct(Some(self.pubkey), seq).auth_info(gas);

        let sign_doc = SignDoc::new(
            &body,
            &auth_info,
            &self.network.chain_id,
            acc_num,
        )?;

        let tx_raw = sign_doc.sign(&self.signing_key())?;

        todo!()

        // let res = self.block_on(tx_raw.broadcast_commit(&self.rpc))?;

        // Ok(broadcast_tx_response(M::Proto::TYPE_URL, res))
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
            gas_limit: Default::default(),
            payer: None,
            granter: None,
        })
    }

    pub async fn estimate_gas<M>(&self, msg: M) -> Result<u64, TxError>
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

        let sign_doc = SignDoc::new(
            &body,
            &auth_info,
            &self.network.chain_id,
            acc_num,
        )?;

        let tx_raw = sign_doc.sign(&self.signing_key())?.to_bytes()?;

        let tx_hex = hex::encode(tx_raw);

        self.network.post("cosmos/tx/v1beta1/simulate", &tx_hex).await;
        todo!()
    }
}

use std::str::FromStr;

use cosmrs::cosmwasm::MsgExecuteContract;
use cosmrs::tx::Gas;
use cosmrs::AccountId;
use cosmwasm_std::Uint128;
use ecvrf_rs::{Proof, PublicKey, SecretKey};
use entropy_beacon_cosmos::msg::{ExecuteMsg as BeaconExecuteMsg, QueryMsg as BeaconQueryMsg};
use entropy_beacon_cosmos::provide::{
    ActiveRequestsQuery, ActiveRequestsResponse, LastEntropyQuery, LastEntropyResponse,
    SubmitEntropyMsg, MAX_PAGINATION_LIMIT,
};

use crate::cosmos::response::TxResponse;
use crate::cosmos::tx::TxError;
use crate::cosmos::{network::Network, queries::QueryError, wallet::Wallet};

pub fn test_pk() -> PublicKey {
    //d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a as bytes
    let bytes: [u8; 32] = [
        215, 90, 152, 1, 130, 177, 10, 183, 213, 75, 254, 211, 201, 100, 7, 58, 14, 225, 114, 243,
        218, 166, 35, 37, 175, 2, 26, 104, 247, 7, 81, 26,
    ];

    PublicKey::from_bytes(&bytes)
}

#[allow(dead_code)]
pub fn test_sk() -> SecretKey {
    //9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60 as bytes
    let bytes: [u8; 32] = [
        157, 97, 177, 157, 239, 253, 90, 96, 186, 132, 74, 244, 146, 236, 44, 196, 68, 73, 197,
        105, 123, 50, 105, 25, 112, 59, 172, 3, 28, 174, 127, 96,
    ];
    SecretKey::from_slice(&bytes)
}

pub struct Beacon {
    pub network: Network,
    pub signer: Wallet,
    pub address: String,
}

impl Beacon {
    pub fn new(network: Network, signer: Wallet, address: String) -> Self {
        Self {
            network,
            signer,
            address,
        }
    }

    pub async fn fetch_active_requests(&self) -> Result<ActiveRequestsResponse, QueryError> {
        let mut requests = vec![];
        let mut start_after = None;
        loop {
            let response = self
                .network
                .query(
                    self.address.clone(),
                    BeaconQueryMsg::ActiveRequests(ActiveRequestsQuery {
                        start_after,
                        limit: Some(MAX_PAGINATION_LIMIT),
                    }),
                )
                .await?;

            let response = serde_json::from_value::<ActiveRequestsResponse>(response)
                .map_err(|e| QueryError::ParseError(e.to_string()))?;
            requests.extend(response.requests.clone());
            if response.requests.len() < MAX_PAGINATION_LIMIT.try_into().unwrap() {
                break;
            }
            start_after = Some(response.requests.last().unwrap().id);
        }

        Ok(ActiveRequestsResponse { requests })
    }
    
    pub async fn fetch_last_entropy(&self) -> Result<LastEntropyResponse, QueryError> {
        serde_json::from_value::<LastEntropyResponse>(
            self.network
                .query(
                    self.address.clone(),
                    BeaconQueryMsg::LastEntropy(LastEntropyQuery {}),
                )
                .await?,
        )
        .map_err(|e| QueryError::ParseError(e.to_string()))
    }

    pub async fn submit_entropy(&self, proof: &Proof, gas: Gas, request_ids: Vec<Uint128>) -> Result<TxResponse, TxError> {
        let msg = SubmitEntropyMsg {
            proof: proof.clone(),
            request_ids,
        };

        let msg = serde_json::to_string(&BeaconExecuteMsg::SubmitEntropy(msg)).unwrap();

        let msg = MsgExecuteContract {
            sender: self.signer.address.clone(),
            contract: AccountId::from_str(&self.address).unwrap(),
            msg: msg.into_bytes(),
            funds: vec![],
        };

        let hash = self.signer.broadcast_msg(msg, Some(gas)).await?;
        let res = self.signer.wait_for_hash(hash).await?;

        Ok(res)
    }
}

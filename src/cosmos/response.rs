use cosmrs::cosmwasm::MsgStoreCodeResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::tx::TxError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(serialize_with = "serialization::serialize_attributes")]
    #[serde(deserialize_with = "serialization::deserialize_attributes")]
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxLogEntry {
    pub msg_index: u32,
    pub log: String,
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxResponse {
    pub code: u32,
    pub codespace: String,
    pub data: String,
    pub gas_wanted: String,
    pub gas_used: String,
    pub height: String,
    pub info: String,
    pub txhash: String,
    pub logs: Vec<TxLogEntry>,
    pub timestamp: String,
}

mod serialization {
    use std::collections::HashMap;

    use serde::{ser::SerializeSeq, Deserialize};
    use serde_json::json;

    pub(crate) fn serialize_attributes<S>(
        attrs: &HashMap<String, String>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize each kv as {"key": k, "value": v}
        let attrs = attrs
            .iter()
            .map(|(k, v)| json!({"key": k, "value": v}))
            .collect::<Vec<_>>();
        let mut seq = serializer.serialize_seq(Some(attrs.len()))?;
        for attr in attrs {
            seq.serialize_element(&attr)?;
        }
        seq.end()
    }

    pub(crate) fn deserialize_attributes<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<String, String>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize each kv in the list from {"key": k, "value": v} to HashMap
        #[derive(Deserialize)]
        struct Attr {
            key: String,
            value: String,
        }
        let attrs = Vec::<Attr>::deserialize(deserializer)?;
        Ok(HashMap::from_iter(
            attrs.into_iter().map(|attr| (attr.key, attr.value)),
        ))
    }
}

impl TryFrom<TxResponse> for MsgStoreCodeResponse {
    type Error = TxError;
    fn try_from(tx: TxResponse) -> Result<Self, Self::Error> {
        let code_id = tx
            .logs
            .iter()
            .find(|log| {
                log.events.iter().any(|event| {
                    event.attributes.get("action")
                        == Some(&"/cosmwasm.wasm.v1.MsgStoreCode".to_string())
                })
            })
            .and_then(|log| {
                log.events
                    .iter()
                    .find_map(|event| event.attributes.get("code_id").cloned())
            })
            .map(|code_id| {
                code_id
                    .parse::<u64>()
                    .map_err(|_| TxError::Parse("Unable to parse code_id".to_string()))
            })
            .transpose()?
            .ok_or_else(|| TxError::Parse("Unable to find code_id in logs".to_string()))?;

        Ok(MsgStoreCodeResponse { code_id })
    }
}

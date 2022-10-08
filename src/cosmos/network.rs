use bip32::DerivationPath;
use cosmrs::tendermint::chain::Id as ChainId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub lcd_url: String,
    pub chain_id: ChainId,
    pub account_info: NetworkAccountInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAccountInfo {
    #[serde(serialize_with = "serialization::serialize_derivation_path")]
    #[serde(deserialize_with = "serialization::deserialize_derivation_path")]
    pub derivation_path: DerivationPath,
    pub chain_prefix: String,
}

mod serialization {
    use std::str::FromStr;

    use bip32::DerivationPath;
    use serde::Deserialize;

    pub(crate) fn serialize_derivation_path<S>(path: &DerivationPath, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let path = path.to_string();
        serializer.serialize_str(&path)
    }
    
    pub(crate) fn deserialize_derivation_path<'de, D>(deserializer: D) -> Result<DerivationPath, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = String::deserialize(deserializer)?;
        let path = DerivationPath::from_str(&path).map_err(serde::de::Error::custom)?;
        Ok(path)
    }
}

impl Network {
    pub fn new(lcd_url: String, chain_id: ChainId, account_info: NetworkAccountInfo) -> Self {
        Self {
            lcd_url,
            chain_id,
            account_info,
        }
    }

    pub fn default_localterra() -> Self {
        Self {
            lcd_url: "http://localhost:1317".to_string(),
            chain_id: ChainId::try_from("localterra".to_string()).unwrap(),
            account_info: NetworkAccountInfo {
                derivation_path: "m/44'/330'/0'/0/0".parse().unwrap(),
                chain_prefix: "terra".to_string(),
            },
        }
    }

    pub fn default_localkujira() -> Self {
        // TODO: Correct Kujira endpoints, etc.
        Self {
            lcd_url: "http://localhost:1317".to_string(),
            chain_id: ChainId::try_from("localkujira".to_string()).unwrap(),
            account_info: NetworkAccountInfo {
                derivation_path: "m/44'/330'/0'/0/0".parse().unwrap(),
                chain_prefix: "kujira".to_string(),
            },
        }
    }
}

impl NetworkAccountInfo {
    pub fn new(derivation_path: DerivationPath, chain_prefix: String) -> Self {
        Self {
            derivation_path,
            chain_prefix,
        }
    }
}

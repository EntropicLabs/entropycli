use std::str::FromStr;

use bip32::DerivationPath;
use cosmrs::{
    tendermint::chain::Id as ChainId,
    tx::{Fee, Gas},
    Coin, Denom, ErrorReport,
};
use serde::{Deserialize, Serialize};

use super::utils::mul_gas_float;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub lcd_url: String,
    pub chain_id: ChainId,
    pub account_info: NetworkAccountInfo,
    pub gas_info: NetworkGasInfo,
    pub deployed_beacon_address: Option<String>,
    pub subsidized_callbacks: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAccountInfo {
    #[serde(serialize_with = "serialization::serialize_derivation_path")]
    #[serde(deserialize_with = "serialization::deserialize_derivation_path")]
    pub derivation_path: DerivationPath,
    pub chain_prefix: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkGasInfo {
    pub denom: String,
    pub gas_price: f64,
    pub gas_adjustment: f64,
}

mod serialization {
    use std::str::FromStr;

    use bip32::DerivationPath;
    use serde::Deserialize;

    pub(crate) fn serialize_derivation_path<S>(
        path: &DerivationPath,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let path = path.to_string();
        serializer.serialize_str(&path)
    }

    pub(crate) fn deserialize_derivation_path<'de, D>(
        deserializer: D,
    ) -> Result<DerivationPath, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = String::deserialize(deserializer)?;
        let path = DerivationPath::from_str(&path).map_err(serde::de::Error::custom)?;
        Ok(path)
    }
}

impl Network {
    pub fn default_localterra() -> Self {
        Self {
            lcd_url: "http://localhost:1317".to_string(),
            chain_id: ChainId::try_from("localterra".to_string()).unwrap(),
            account_info: NetworkAccountInfo {
                derivation_path: "m/44'/330'/0'/0/0".parse().unwrap(),
                chain_prefix: "terra".to_string(),
            },
            gas_info: NetworkGasInfo {
                denom: "uluna".to_string(),
                gas_price: 5.0,
                gas_adjustment: 1.25,
            },
            deployed_beacon_address: None,
            subsidized_callbacks: Some(false),
        }
    }

    pub fn default_localkujira() -> Self {
        Self {
            lcd_url: "http://localhost:1317".to_string(),
            chain_id: ChainId::try_from("harpoon-2".to_string()).unwrap(),
            account_info: NetworkAccountInfo {
                derivation_path: "m/44'/118'/0'/0/0".parse().unwrap(),
                chain_prefix: "kujira".to_string(),
            },
            gas_info: NetworkGasInfo {
                denom: "ukuji".to_string(),
                gas_price: 0.00125,
                gas_adjustment: 1.25,
            },
            deployed_beacon_address: None,
            subsidized_callbacks: Some(true),
        }
    }
}

impl NetworkAccountInfo {
    #[allow(dead_code)]
    pub fn new(derivation_path: DerivationPath, chain_prefix: String) -> Self {
        Self {
            derivation_path,
            chain_prefix,
        }
    }
}

impl NetworkGasInfo {
    #[allow(dead_code)]
    pub fn new(denom: String, gas_price: f64, gas_adjustment: f64) -> Self {
        Self {
            denom,
            gas_price,
            gas_adjustment,
        }
    }

    pub fn gas_to_fee(&self, gas: impl Into<Gas> + Clone) -> Result<Fee, ErrorReport> {
        #[allow(clippy::cast_precision_loss)]
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        // We can safely cast here because we know that the gas price won't be
        // mangled by these conversions.
        let amount = u128::from(mul_gas_float(gas.clone(), self.gas_price).value());

        Ok(Fee::from_amount_and_gas(
            Coin {
                denom: Denom::from_str(self.denom.as_str())?,
                amount,
            },
            gas,
        ))
    }
}

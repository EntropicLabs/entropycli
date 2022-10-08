use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::cosmos::network::Network;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub networks: Option<HashMap<String, Network>>,
    pub default_network: Option<String>,
    pub default_wallet: Option<String>,
    pub wallets: Option<HashMap<String, Option<String>>>,
}

impl Config {
    pub fn load(path: Option<String>) -> Self {
        let path = match path {
            Some(path) => PathBuf::from(path),
            None => std::env::current_dir().unwrap().join("entropy.json"),
        };
        let config = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub name: String,
    pub rpc: String,
    pub chain_id: String,
    pub lcd: String,
    pub fcd: String,
    pub default_wallet: String,
    pub beacon_address: Option<String>,
}

impl NetworkInfo {
    pub fn localterra() -> Self {
        Self {
            name: "localterra".to_string(),
            rpc: "http://localhost:26657".to_string(),
            chain_id: "localterra".to_string(),
            lcd: "http://localhost:1317".to_string(),
            fcd: "http://localhost:3060".to_string(),
            default_wallet: "test1".to_string(),
            beacon_address: None,
        }
    }
    pub fn localkujira() -> Self {
        // TODO: Correct Kujira endpoints, etc.
        Self {
            name: "localkujira".to_string(),
            rpc: "http://localhost:26657".to_string(),
            chain_id: "localkujira".to_string(),
            lcd: "http://localhost:1317".to_string(),
            fcd: "http://localhost:3060".to_string(),
            default_wallet: "test1".to_string(),
            beacon_address: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WalletType {
    STRING,
    ENV,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    pub name: String,
    pub mnemonic_type: WalletType,
    pub mnemonic: Option<String>,
}

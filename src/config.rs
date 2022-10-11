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

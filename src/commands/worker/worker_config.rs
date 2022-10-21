use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    cosmos::network::Network,
};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    #[serde(flatten)]
    pub network: Network,
    pub signer_mnemonic: Option<String>,
}

#[derive(Debug,Clone, Serialize, Deserialize, Default)]
pub struct WorkerConfig {
    pub registered_keys: Vec<String>,
    pub networks: HashMap<String, NetworkConfiguration>,
    pub default_network: Option<String>,
}

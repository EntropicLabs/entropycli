use std::{collections::HashMap, path::Path};

use dialoguer::Confirm;
use serde::{Deserialize, Serialize};

use crate::{
    cosmos::network::Network,
    utils::{config::ConfigUtils, user_prompts::create_network},
    utils::{user_prompts::create_wallet, CLITheme},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub networks: Option<HashMap<String, Network>>,
    pub default_network: Option<String>,
    pub default_wallet: Option<String>,
    pub wallets: Option<HashMap<String, Option<String>>>,
}

impl ProjectConfig {
    pub fn prompt_config_creation<P>(path: &P) -> ProjectConfig
    where
        P: AsRef<Path> + Into<String>,
    {
        let theme = CLITheme::default();
        let (network_name, network_info) = create_network();

        let mut config = ProjectConfig {
            networks: Some(HashMap::from_iter(vec![(
                network_name.to_string(),
                network_info,
            )])),
            default_network: Some(network_name),
            default_wallet: None,
            wallets: None,
        };

        if Confirm::with_theme(&theme)
            .with_prompt("Would you like to create a default wallet?")
            .default(true)
            .interact()
            .unwrap()
        {
            let (name, mnemonic) = create_wallet();
            config.default_wallet = Some(name.clone());
            if let Some(ref mut wallets) = config.wallets {
                wallets.insert(name, mnemonic);
            } else {
                config.wallets = Some(HashMap::new());
                config.wallets.as_mut().unwrap().insert(name, mnemonic);
            }
        } else {
            config.default_wallet = Some("<WALLET_NAME>".to_string());
            config.wallets = Some(HashMap::from_iter(vec![(
                "<WALLET_NAME>".to_string(),
                Some("<WALLET_MNEMONIC>".to_string()),
            )]));
            println!(
                "{}",
                theme
                    .highlight
                    .bold()
                    .apply_to("Add your test wallet information to the config before deploying.")
            );
        }
        ConfigUtils::save(&config, &path).unwrap_or_else(|e| {
            println!(
                "{} {}",
                theme.error.apply_to("Error writing config file: "),
                theme.error.apply_to(e.to_string())
            );
            std::process::exit(1);
        });

        println!("{}", theme.dimmed.apply_to("Wrote configuration to file."));

        config
    }

    pub fn get_network(&self, name: &Option<String>) -> Result<(String, Option<Network>), ()> {
        let name = name
            .as_ref()
            .map_or(self.default_network.as_ref(), Some)
            .ok_or(())?;
        Ok((
            name.clone(),
            self.networks.as_ref().and_then(|n| n.get(name)).cloned(),
        ))
    }
}

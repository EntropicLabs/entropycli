use std::{
    collections::HashMap,
    io::{self, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{cosmos::network::Network, theme::CLITheme, utils::user_prompts::create_network};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub networks: Option<HashMap<String, Network>>,
    pub default_network: Option<String>,
    pub default_wallet: Option<String>,
    pub wallets: Option<HashMap<String, Option<String>>>,
}

impl Config {
    pub fn prompt_config_creation<P>(path: &P) -> Config
    where
        P: AsRef<Path> + Into<String>,
    {
        let theme = CLITheme::default();
        let (network_name, network_info) = create_network();

        print!(
            "{} {}{}",
            theme.dimmed.apply_to("Writing configuration to"),
            theme.dimmed.apply_to(path.as_ref().to_str().unwrap()),
            theme.dimmed.apply_to("..."),
        );
        let mut file = std::fs::File::create(path).unwrap();

        let config = Config {
            networks: Some(HashMap::from_iter(vec![(
                network_name.to_string(),
                network_info,
            )])),
            default_network: Some(network_name),
            default_wallet: Some("<WALLET_NAME>".to_string()),
            wallets: Some(HashMap::from_iter(vec![(
                "<WALLET_NAME>".to_string(),
                Some("<WALLET_MNEMONIC>".to_string()),
            )])),
        };

        let json = serde_json::to_string_pretty(&config).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        println!(" {}", theme.success.apply_to("Done."));
        println!(
            "{}",
            theme
                .highlight
                .bold()
                .apply_to("Add your test wallet information to the config before proceeding.")
        );
        config
    }

    pub fn load<P>(path: &P) -> Result<Self, io::Error>
    where
        P: AsRef<Path>,
    {
        let config = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&config).unwrap())
    }

    pub fn save<P>(&self, path: &P) -> Result<(), io::Error>
    where
        P: AsRef<Path>,
    {
        let config = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, config)?;
        Ok(())
    }
}

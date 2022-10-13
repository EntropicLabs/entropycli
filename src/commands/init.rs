use std::{collections::HashMap, io::Write, path::PathBuf};

use crate::{
    config::Config,
    theme::CLITheme,
    utils::{user_prompts::create_network, deploy::deploy_beacon},
};

use clap::Parser;
use dialoguer::Confirm;

use console::{style};

#[derive(Debug, Parser, Clone)]
pub struct InitCommandOptions {
    /// Optional path to the configuration file
    #[clap(short, long)]
    pub config: Option<String>,
    /// Network to use (defined in config). Optional if default network is set in config
    #[clap(short, long)]
    pub network: Option<String>,
    /// Wallet to use (defined in config). Optional if default wallet is set in config
    #[clap(short, long)]
    pub wallet: Option<String>,
}

pub async fn init_cmd(options: InitCommandOptions) {
    let theme = CLITheme::default();

    println!(
        "{}",
        style(format!("entropy init v{}", env!("CARGO_PKG_VERSION"))).bold()
    );

    let mut prompt = "Create new config file?";

    if check_config(options.config.clone()) {
        prompt = "Create new config file? (OVERWRITES EXISTING CONFIG)";
    }

    if Confirm::with_theme(&theme)
        .with_prompt(prompt)
        .default(false)
        .interact()
        .unwrap()
    {
        create_config(options.config.clone());
    }

    let config = Config::load(options.config.clone()).unwrap();

    if Confirm::with_theme(&theme)
        .with_prompt("Deploy mock beacon?")
        .default(true)
        .interact()
        .unwrap()
    {
        deploy_beacon(options.network, options.wallet, config).await;
    }
}

// Check whether a configuration file exists in the current directory
// If it does, return the path to the file
// If it doesn't, create a new configuration file and return the path to the file
pub fn check_config(path: Option<String>) -> bool {
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir().unwrap().join("entropy.json"),
    };
    path.exists()
}

pub fn create_config(path: Option<String>) -> Config {
    let theme = CLITheme::default();
    let (network_name, network_info) = create_network();

    let path = path.map_or_else(
        || std::env::current_dir().unwrap().join("entropy.json"),
        PathBuf::from,
    );
    print!(
        "{} {}{}",
        theme.dimmed.apply_to("Writing configuration to"),
        theme.dimmed.apply_to(path.to_str().unwrap()),
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

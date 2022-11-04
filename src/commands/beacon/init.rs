use std::path::PathBuf;

use crate::{
    commands::beacon::project_config::ProjectConfig,
    utils::{deploy::deploy_beacon, config::ConfigType},
    utils::{CLITheme, config::ConfigUtils},
};

use clap::Parser;
use dialoguer::Confirm;

#[derive(Debug, Parser, Clone)]
pub struct InitCommandOptions {
    /// Optional path to the configuration file
    #[clap(short, long)]
    #[clap(default_value = "entropy.json")]
    pub config: String,
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
        dialoguer::console::style(format!("entropy beacon init v{}", env!("CARGO_PKG_VERSION"))).bold()
    );

    let mut prompt = "Create new config file?";

    if PathBuf::from(&options.config).exists() {
        prompt = "Create new config file? (OVERWRITES EXISTING CONFIG)";
    }

    if Confirm::with_theme(&theme)
        .with_prompt(prompt)
        .default(false)
        .interact()
        .unwrap()
    {
        ProjectConfig::prompt_config_creation(&options.config);
    }

    let config = ConfigUtils::load(&options.config).unwrap_or_else(|e| {
        println!(
            "{} {}",
            theme.error.apply_to("Error loading config file: "),
            theme.error.apply_to(e.to_string())
        );
        std::process::exit(1);
    });
    let mut config = if let ConfigType::Project(config) = config {
        config
    } else {
        println!("{}", theme.error.apply_to("Config file is not a project config"));
        std::process::exit(1);
    };

    if Confirm::with_theme(&theme)
        .with_prompt("Deploy mock beacon?")
        .default(true)
        .interact()
        .unwrap()
    {
        deploy_beacon(options.network, options.wallet, &mut config).await;

        ConfigUtils::save(&config, &options.config).unwrap_or_else(|e| {
            println!(
                "{} {}",
                theme.error.apply_to("Error updating config file: "),
                theme.error.apply_to(e.to_string())
            );
            std::process::exit(1);
        });

        println!(
            "{}",
            theme
                .dimmed
                .apply_to("Updated config file with deployed beacon address")
        );
    }
}

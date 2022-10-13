use std::path::PathBuf;

use crate::{config::Config, utils::CLITheme, utils::deploy::deploy_beacon};

use clap::Parser;
use dialoguer::Confirm;

use console::style;

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
        style(format!("entropy init v{}", env!("CARGO_PKG_VERSION"))).bold()
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
        Config::prompt_config_creation(&options.config);
    }

    let mut config = Config::load(&options.config).unwrap();

    if Confirm::with_theme(&theme)
        .with_prompt("Deploy mock beacon?")
        .default(true)
        .interact()
        .unwrap()
    {
        deploy_beacon(options.network, options.wallet, &mut config).await;

        config.save(&options.config).unwrap_or_else(|e| {
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

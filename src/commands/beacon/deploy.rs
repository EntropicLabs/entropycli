use clap::Parser;

use crate::{
    utils::{config::ConfigType, CLITheme},
    utils::{config::ConfigUtils, deploy::deploy_beacon},
};

#[derive(Debug, Parser, Clone)]
pub struct DeployCommandOptions {
    /// Path to the configuration file
    #[clap(short, long)]
    #[clap(default_value = "entropy.json")]
    config: String,
    /// Network to use (defined in config). Optional if default network is set in config
    #[clap(short, long)]
    pub network: Option<String>,
    /// Wallet to use (defined in config). Optional if default wallet is set in config
    #[clap(short, long)]
    pub wallet: Option<String>,
}

pub async fn deploy_cmd(options: DeployCommandOptions) {
    let theme = CLITheme::default();
    println!(
        "{}",
        dialoguer::console::style(format!("entropy beacon deploy v{}", env!("CARGO_PKG_VERSION"))).bold()
    );

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
        println!(
            "{}",
            theme.error.apply_to("Config file is not a project config")
        );
        std::process::exit(1);
    };

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

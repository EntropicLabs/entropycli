use clap::Parser;
use console::style;

use crate::{config::Config, theme::CLITheme, utils::deploy::deploy_beacon};

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
        style(format!("entropy deploy v{}", env!("CARGO_PKG_VERSION"))).bold()
    );

    let mut config = Config::load(&options.config).unwrap();

    deploy_beacon(options.network, options.wallet, &mut config).await;

    config.save(&options.config).unwrap_or_else(|e| {
        println!(
            "{} {}",
            theme.error.apply_to("Error updating config file: "),
            theme.error.apply_to(e.to_string())
        );
    });

    println!(
        "{}",
        theme
            .dimmed
            .apply_to("Updated config file with deployed beacon address")
    );
}

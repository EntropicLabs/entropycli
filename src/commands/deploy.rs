use clap::Parser;
use console::style;

use crate::{config::Config, utils::deploy::deploy_beacon};

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
    println!(
        "{}",
        style(format!("entropy deploy v{}", env!("CARGO_PKG_VERSION"))).bold()
    );

    let config = Config::load(Some(options.config)).unwrap();

    deploy_beacon(options.network, options.wallet, config).await;
}

use clap::{Parser, Subcommand};

pub mod deploy;
pub mod dev;
pub mod init;
pub mod project_config;
pub mod wallet;

use deploy::{deploy_cmd, DeployCommandOptions};
use dev::{dev_cmd, DevCommandOptions};
use init::{init_cmd, InitCommandOptions};
use wallet::{wallet_cmd, WalletCommandOptions};

use super::network::{network_cmd, NetworkCommandOptions};

#[derive(Debug, Parser, Clone)]
pub struct BeaconCommandOptions {
    #[clap(subcommand)]
    pub command: BeaconCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum BeaconCommand {
    #[clap(about = "Initialize a new project")]
    Init(InitCommandOptions),
    #[clap(about = "Deploy a new instance of Beacon")]
    Deploy(DeployCommandOptions),
    #[clap(about = "Manage wallets")]
    Wallet(WalletCommandOptions),
    #[clap(about = "Run a local development instance of workers")]
    Dev(DevCommandOptions),
    #[clap(about = "Manage networks (alias for `entropy network`)")]
    Network(NetworkCommandOptions),
}

pub async fn beacon_cmd(options: BeaconCommandOptions) {
    match options.command {
        BeaconCommand::Init(options) => init_cmd(options).await,
        BeaconCommand::Deploy(options) => deploy_cmd(options).await,
        BeaconCommand::Wallet(options) => wallet_cmd(options),
        BeaconCommand::Dev(options) => dev_cmd(options).await,
        BeaconCommand::Network(options) => network_cmd(options),
    }
}

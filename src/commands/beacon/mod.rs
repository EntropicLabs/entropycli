use clap::{Parser, Subcommand};

pub mod deploy;
pub mod init;
pub mod project_config;
pub mod wallet;

use deploy::{DeployCommandOptions, deploy_cmd};
use init::{InitCommandOptions, init_cmd};
use wallet::{WalletCommandOptions, wallet_cmd};

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
}

pub async fn beacon_cmd(options: BeaconCommandOptions) {
    match options.command {
        BeaconCommand::Init(options) => init_cmd(options).await,
        BeaconCommand::Deploy(options) => deploy_cmd(options).await,
        BeaconCommand::Wallet(options) => wallet_cmd(options),
    }
}

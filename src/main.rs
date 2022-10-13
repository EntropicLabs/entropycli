#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
mod commands;
mod utils;
mod config;
mod wasm_fetch;
mod cosmos;

use clap::{Parser, Subcommand};
use commands::{init_cmd, network_cmd, deploy_cmd, wallet_cmd};


use crate::commands::{network::NetworkCommandOptions, init::InitCommandOptions, deploy::DeployCommandOptions, wallet::WalletCommandOptions};

#[derive(Debug, Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "A CLI tool to develop applications using the Entropic Labs ecosystem")]
#[clap(name = "entropy")]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(about = "Initialize a new project")]
    Init(InitCommandOptions),
    #[clap(about = "Manage networks")]
    Network(NetworkCommandOptions),
    #[clap(about = "Deploy a new instance of Beacon")]
    Deploy(DeployCommandOptions),
    #[clap(about = "Manage wallets")]
    Wallet(WalletCommandOptions),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Init(options) => init_cmd(options).await,
        Command::Deploy(options) => deploy_cmd(options).await,
        Command::Network(options) => network_cmd(options),
        Command::Wallet(options) => wallet_cmd(options),
    }
}
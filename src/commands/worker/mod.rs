use clap::{Parser, Subcommand};

pub mod worker_config;
pub mod keys;
pub mod start;

use start::{start_cmd};
use keys::{key_cmd, KeyCommandOptions};


#[derive(Debug, Parser, Clone)]
pub struct WorkerCommandOptions {
    #[clap(subcommand)]
    pub command: WorkerCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum WorkerCommand {
    #[clap(about = "Manage keys for a worker")]
    Keys(KeyCommandOptions),
    #[clap(about = "Start a worker")]
    Start,
}

pub async fn worker_cmd(options: WorkerCommandOptions) {
    match options.command {
        WorkerCommand::Keys(options) => key_cmd(&options),
        WorkerCommand::Start => start_cmd().await,
    }
}
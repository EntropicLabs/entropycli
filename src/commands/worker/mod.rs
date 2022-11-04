use clap::{Parser, Subcommand};

pub mod worker_config;
pub mod keys;
pub mod start;

use start::{start_cmd};


#[derive(Debug, Parser, Clone)]
pub struct WorkerCommandOptions {
    #[clap(subcommand)]
    pub command: WorkerCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum WorkerCommand {
    // #[clap(about = "Manage keys for a worker")]
    // Keys(KeyCommandOptions)
    #[clap(about = "Start a worker")]
    Start,
}

pub async fn worker_cmd(options: WorkerCommandOptions) {
    match options.command {
        // WorkerCommand::Keys(options) => key_cmd(options).await,
        WorkerCommand::Start => start_cmd().await,
    }
}
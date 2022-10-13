use std::collections::HashMap;

use clap::{Parser, Subcommand};
use console::style;
use dialoguer::Select;

use crate::{config::Config, utils::user_prompts::create_wallet, utils::CLITheme};

#[derive(Debug, Parser, Clone)]
pub struct WalletCommandOptions {
    #[clap(subcommand)]
    pub command: WalletCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum WalletCommand {
    #[clap(about = "Add a new wallet to the configuration file")]
    #[clap(alias = "add")]
    New {
        /// Path to the configuration file
        #[clap(short, long)]
        #[clap(default_value = "entropy.json")]
        config: String,
    },
    #[clap(about = "List all wallets in the configuration file")]
    List {
        /// Path to the configuration file
        #[clap(short, long)]
        #[clap(default_value = "entropy.json")]
        config: String,

        #[clap(short = 'm', long)]
        show_mnemonics: bool,
    },
    #[clap(about = "Remove a wallet from the configuration file")]
    Remove {
        /// The name of the wallet to remove
        wallet: Option<String>,
        /// Path to the configuration file
        #[clap(short, long)]
        #[clap(default_value = "entropy.json")]
        config: String,
    },
}

pub fn wallet_cmd(options: WalletCommandOptions) {
    println!(
        "{}",
        style(format!("entropy wallet v{}", env!("CARGO_PKG_VERSION"))).bold()
    );

    match options.command {
        WalletCommand::New { config } => new_wallet(&config),
        WalletCommand::List {
            config,
            show_mnemonics,
        } => list_wallets(&config, show_mnemonics),
        WalletCommand::Remove { wallet, config } => remove_wallet(wallet, &config),
    }
}

fn new_wallet(config: &str) {
    let mut cfg = Config::load(&config).unwrap();
    let theme = CLITheme::default();

    println!("{}", theme.highlight.apply_to("Creating a new wallet."),);
    let (name, mnemonic) = create_wallet();

    if let Some(ref mut wallets) = cfg.wallets {
        wallets.insert(name, mnemonic);
    } else {
        cfg.wallets = Some(HashMap::new());
        cfg.wallets.as_mut().unwrap().insert(name, mnemonic);
    }

    cfg.save(&config).unwrap_or_else(|e| {
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
            .apply_to("Updated config file with new wallet.")
    );
}

fn list_wallets(config: &str, show_mnemonics: bool) {
    let config = Config::load(&config).unwrap();
    let theme = CLITheme::default();

    if let Some(wallets) = config.wallets {
        println!("{}", theme.highlight.apply_to("Wallets:"),);
        for (name, mnemonic) in wallets {
            if show_mnemonics {
                println!(
                    "  {}: {}",
                    theme.normal.apply_to(name),
                    theme
                        .dimmed
                        .apply_to(mnemonic.unwrap_or_else(|| "Fetched from ENV".to_string()))
                );
            } else {
                println!(
                    "  {}: {}",
                    theme.normal.apply_to(name),
                    theme
                        .dimmed
                        .apply_to(mnemonic.map_or("Fetched from ENV", |_| "<mnemonic hidden>"))
                );
            }
        }
    } else {
        println!("{}", theme.warning.apply_to("No wallets in config file."));
    }
}

fn remove_wallet(wallet: Option<String>, config: &str) {
    let mut cfg = Config::load(&config).unwrap();
    let theme = CLITheme::default();

    let wallet = wallet.unwrap_or_else(|| {
        if let Some(ref wallets) = cfg.wallets {
            let select_opts = wallets.keys().cloned().collect::<Vec<_>>();
            let network_name = Select::with_theme(&theme)
                .with_prompt("Select a wallet to remove")
                .default(0)
                .items(&select_opts)
                .interact()
                .unwrap();
            return select_opts[network_name].to_string();
        }
        println!("{}", theme.error.apply_to("No wallets in config file."));
        std::process::exit(1);
    });

    let removed = cfg
        .wallets
        .as_mut()
        .map_or(false, |wallets| wallets.remove(&wallet).is_some());
    if removed {
        cfg.save(&config).unwrap_or_else(|e| {
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
                .apply_to("Updated config file with new wallet.")
        );
    } else {
        println!(
            "{}",
            theme
                .error
                .apply_to(format!("Wallet {} not found.", wallet))
        );
    }
}

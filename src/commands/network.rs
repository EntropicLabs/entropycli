use std::collections::HashMap;

use clap::{Parser, Subcommand};

use crate::{
    commands::worker::worker_config::NetworkConfiguration,
    cosmos::network::Network,
    utils::{config::ConfigType, CLITheme},
    utils::{
        config::ConfigUtils,
        user_prompts::{create_network, create_wallet},
    },
};

#[derive(Debug, Parser, Clone)]
pub struct NetworkCommandOptions {
    #[clap(subcommand)]
    pub command: NetworkCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum NetworkCommand {
    #[clap(about = "Add a new network to the configuration file")]
    #[clap(alias = "add")]
    New {
        /// Path to the configuration file
        #[clap(short, long)]
        #[clap(default_value = "entropy.json")]
        config: String,
    },
    #[clap(about = "List all networks in the configuration file")]
    List {
        /// Path to the configuration file
        #[clap(short, long)]
        #[clap(default_value = "entropy.json")]
        config: String,
    },
    #[clap(about = "Remove a network from the configuration file")]
    Remove {
        /// Path to the configuration file
        #[clap(short, long)]
        #[clap(default_value = "entropy.json")]
        config: String,
        network: String,
    },
}

pub fn network_cmd(options: NetworkCommandOptions) {
    println!(
        "{}",
        dialoguer::console::style(format!("entropy network v{}", env!("CARGO_PKG_VERSION"))).bold()
    );
    match options.command {
        NetworkCommand::New { config } => new_network(&config),
        NetworkCommand::List { config } => list_networks(&config),
        NetworkCommand::Remove { config, network } => remove_network(&config, &network),
    }
}

fn new_network(config: &String) {
    let theme = CLITheme::default();
    let mut cfg = ConfigUtils::load(&config).unwrap_or_else(|e| {
        println!(
            "{} {}",
            theme.error.apply_to("Error loading config file: "),
            theme.error.apply_to(e.to_string())
        );
        std::process::exit(1);
    });
    println!("{}", theme.highlight.apply_to("Creating a new network."),);
    let (name, network) = create_network();
    match cfg {
        ConfigType::Project(ref mut cfg) => {
            if let Some(ref mut networks) = cfg.networks {
                networks.insert(name, network);
            } else {
                cfg.networks = Some(HashMap::new());
                cfg.networks.as_mut().unwrap().insert(name, network);
            }
        }
        ConfigType::Worker(ref mut cfg) => {
            println!(
                "{}",
                theme
                    .highlight
                    .apply_to("Mnemonic for this network (leave blank to use ENV variables)"),
            );
            let (_, mnemonic) = create_wallet();
            let network = NetworkConfiguration {
                network,
                signer_mnemonic: mnemonic,
                subsidized_callbacks: None,
            };
            cfg.networks.insert(name, network);
        }
    }

    ConfigUtils::save(&cfg, &config).unwrap_or_else(|e| {
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
            .apply_to("Updated config file with new network.")
    );
}

fn list_networks(config: &String) {
    let theme = CLITheme::default();

    let cfg = ConfigUtils::load(&config).unwrap_or_else(|e| {
        println!(
            "{} {}",
            theme.error.apply_to("Error loading config file: "),
            theme.error.apply_to(e.to_string())
        );
        std::process::exit(1);
    });

    match cfg {
        ConfigType::Project(cfg) => {
            if let Some(networks) = cfg.networks {
                for (name, network) in networks {
                    print_network(&network, &name, &theme);
                }
            } else {
                println!(
                    "{}",
                    theme
                        .error
                        .apply_to("No networks found in configuration file.")
                );
            }
        }
        ConfigType::Worker(cfg) => {
            if cfg.networks.is_empty() {
                println!(
                    "{}",
                    theme
                        .error
                        .apply_to("No networks found in configuration file.")
                );
            } else {
                for (name, net_cfg) in cfg.networks {
                    print_network(&net_cfg.network, &name, &theme);
                    println!(
                        "    {} {}",
                        theme.dimmed.apply_to("signer-mnemonic:"),
                        theme.normal.apply_to(
                            net_cfg
                                .signer_mnemonic
                                .map_or("Fetched from ENV", |_| "<mnemonic hidden>")
                        )
                    );
                }
            }
        }
    }
}

fn remove_network(config: &String, network: &String) {
    let theme = CLITheme::default();
    let mut cfg = ConfigUtils::load(&config).unwrap_or_else(|e| {
        println!(
            "{} {}",
            theme.error.apply_to("Error loading config file: "),
            theme.error.apply_to(e.to_string())
        );
        std::process::exit(1);
    });
    match cfg {
        ConfigType::Project(ref mut cfg) => {
            if let Some(ref mut networks) = cfg.networks {
                if networks.remove(network).is_none() {
                    println!(
                        "{}",
                        theme
                            .error
                            .apply_to(format!("Network {} not found", network))
                    );
                }
            } else {
                println!(
                    "{}",
                    theme
                        .error
                        .apply_to(format!("Network {} not found", network))
                );
            }
        }
        ConfigType::Worker(ref mut cfg) => {
            if cfg.networks.remove(network).is_none() {
                println!(
                    "{}",
                    theme
                        .error
                        .apply_to(format!("Network {} not found", network))
                );
            }
        }
    }
    ConfigUtils::save(&cfg, &config).unwrap_or_else(|e| {
        println!(
            "{} {}",
            theme.error.apply_to("Error updating config file: "),
            theme.error.apply_to(e.to_string())
        );
        std::process::exit(1);
    });

    println!("{}", theme.dimmed.apply_to("Saved all changes."));
}

fn print_network(network: &Network, name: &String, theme: &CLITheme) {
    println!("{}:", theme.highlight.apply_to(name));
    println!("  {} {}", theme.dimmed.apply_to("LCD:"), network.lcd_url);
    println!(
        "  {} {}",
        theme.dimmed.apply_to("chain-id:"),
        network.chain_id
    );
    println!("  {}", theme.dimmed.apply_to("gas:"));
    println!(
        "    {} {}",
        theme.dimmed.apply_to("denom:"),
        network.gas_info.denom
    );
    println!(
        "    {} {}",
        theme.dimmed.apply_to("price:"),
        network.gas_info.gas_price
    );
    println!(
        "    {} {}",
        theme.dimmed.apply_to("adjustment:"),
        network.gas_info.gas_adjustment
    );
    println!("  {}", theme.dimmed.apply_to("account:"));
    println!(
        "    {} {}",
        theme.dimmed.apply_to("derivation-path:"),
        network.account_info.derivation_path
    );
    println!(
        "    {} {}",
        theme.dimmed.apply_to("chain-prefix:"),
        network.account_info.chain_prefix
    );
}

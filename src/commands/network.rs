use std::collections::HashMap;

use clap::{Parser, Subcommand};
use console::style;

use crate::{
    config::Config, cosmos::network::Network, utils::user_prompts::create_network, utils::CLITheme,
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
        network: String,
        /// Path to the configuration file
        #[clap(short, long)]
        #[clap(default_value = "entropy.json")]
        config: String,
    },
}

pub fn network_cmd(options: NetworkCommandOptions) {
    let theme = CLITheme::default();

    println!(
        "{}",
        style(format!("entropy network v{}", env!("CARGO_PKG_VERSION"))).bold()
    );
    match options.command {
        NetworkCommand::New { config } => {
            let mut cfg = Config::load(&config).unwrap();
            println!("{}", theme.highlight.apply_to("Creating a new network."),);
            let (name, network) = create_network();
            if let Some(ref mut networks) = cfg.networks {
                networks.insert(name, network);
            } else {
                cfg.networks = Some(HashMap::new());
                cfg.networks.as_mut().unwrap().insert(name, network);
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
                    .apply_to("Updated config file with new network.")
            );
        }
        NetworkCommand::List { config } => {
            let config = Config::load(&config).unwrap();
            if let Some(networks) = config.networks {
                for (name, network) in networks {
                    print_network(&network, &name, &theme);
                }
            } else {
                println!(
                    "{}",
                    theme
                        .error
                        .apply_to("No networks found in configuration file")
                );
            }
        }
        NetworkCommand::Remove { network, config } => {
            let mut cfg = Config::load(&config).unwrap();
            if let Some(ref mut networks) = cfg.networks {
                if networks.remove(&network).is_none() {
                    println!(
                        "{}",
                        theme
                            .error
                            .apply_to(format!("Network {} not found", network))
                    );
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
                        .apply_to("Updated config file with new network.")
                );
            } else {
                println!(
                    "{}",
                    theme
                        .error
                        .apply_to(format!("Network {} not found", network))
                );
            }
        }
    }
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

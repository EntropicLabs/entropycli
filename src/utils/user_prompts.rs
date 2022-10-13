use dialoguer::{Input, Select};

use crate::{
    cosmos::network::{Network, NetworkAccountInfo, NetworkGasInfo},
    theme::CLITheme,
};

pub fn create_network() -> (String, Network) {
    let theme = CLITheme::default();
    let select_opts = vec!["localterra", "localkujira", "Manual Setup"];
    let network_name = Select::with_theme(&theme)
        .with_prompt("Add a network")
        .default(0)
        .items(&select_opts)
        .interact()
        .unwrap();
    match network_name {
        0 => ("localterra".to_string(), Network::default_localterra()),
        1 => ("localkujira".to_string(), Network::default_localkujira()),
        2 => {
            let name = Input::with_theme(&theme)
                .with_prompt("Network Name")
                .interact()
                .unwrap();
            let chain_id = Input::with_theme(&theme)
                .with_prompt("Chain ID")
                .interact()
                .unwrap();
            let lcd_url = Input::with_theme(&theme)
                .with_prompt("LCD URL")
                .interact()
                .unwrap();
            let gas_info = {
                let denom = Input::with_theme(&theme)
                    .with_prompt("Gas Denom")
                    .interact()
                    .unwrap();
                let gas_price = Input::with_theme(&theme)
                    .with_prompt("Gas Price")
                    .interact()
                    .unwrap();
                let gas_adjustment = Input::with_theme(&theme)
                    .with_prompt("Gas Adjustment")
                    .interact()
                    .unwrap();
                NetworkGasInfo {
                    denom,
                    gas_price,
                    gas_adjustment,
                }
            };
            let account_info = {
                let derivation_path = Input::with_theme(&theme)
                    .with_prompt("Derivation Path")
                    .interact()
                    .unwrap();
                let chain_prefix = Input::with_theme(&theme)
                    .with_prompt("Chain Prefix")
                    .interact()
                    .unwrap();
                NetworkAccountInfo {
                    derivation_path,
                    chain_prefix,
                }
            };
            (
                name,
                Network {
                    chain_id,
                    lcd_url,
                    gas_info,
                    account_info,
                },
            )
        }
        _ => unreachable!(),
    }
}

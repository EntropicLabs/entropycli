use std::collections::HashMap;

use dialoguer::{Input, Select};

use crate::{
    cosmos::network::{Network, NetworkAccountInfo, NetworkGasInfo},
    utils::CLITheme,
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
                    deployed_beacon_address: None,
                },
            )
        }
        _ => unreachable!(),
    }
}

pub fn create_wallet() -> (String, Option<String>) {
    let theme = CLITheme::default();
    let select_opts = vec!["builtin (localterra)", "Manual Setup"];
    let source = Select::with_theme(&theme)
        .with_prompt("Choose a wallet source")
        .default(0)
        .items(&select_opts)
        .interact()
        .unwrap();
    match source {
        0 => {
            let accounts = localterra_accounts();
            let mut names = accounts.keys().cloned().collect::<Vec<String>>();
            names.sort_by(|a, b| {
                if let Some(a) = a.strip_prefix("test") {
                    if let Some(b) = b.strip_prefix("test") {
                        a.parse::<u32>().unwrap().cmp(&b.parse::<u32>().unwrap())
                    } else {
                        std::cmp::Ordering::Less
                    }
                } else if b.starts_with("test") {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            let account = Select::with_theme(&theme)
                .with_prompt("Choose an account")
                .default(0)
                .items(&names)
                .interact()
                .unwrap();
            let name = names[account].clone();
            let mnemonic = accounts.get(&name).unwrap().to_string();
            (name, Some(mnemonic))
        }
        1 => {
            let name = Input::with_theme(&theme)
                .with_prompt("Wallet Name")
                .interact()
                .unwrap();
            let mnemonic: String = Input::with_theme(&theme)
                .with_prompt("Mnemonic")
                .allow_empty(true)
                .interact()
                .unwrap();
            if mnemonic.is_empty() {
                (name, None)
            } else {
                (name, Some(mnemonic))
            }
        }
        _ => unreachable!(),
    }
}

pub fn localterra_accounts() -> HashMap<String, String> {
    HashMap::from([
      ("validator".to_string(),
        "satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn".to_string()),
      ("test1".to_string(),
          "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius".to_string()),
      ("test2".to_string(),
          "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty".to_string()),
      ("test3".to_string(),
          "symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb".to_string()),
      ("test4".to_string(),
          "bounce success option birth apple portion aunt rural episode solution hockey pencil lend session cause hedgehog slender journey system canvas decorate razor catch empty".to_string()),
      ("test5".to_string(),
          "second render cat sing soup reward cluster island bench diet lumber grocery repeat balcony perfect diesel stumble piano distance caught occur example ozone loyal".to_string()),
      ("test6".to_string(),
          "spatial forest elevator battle also spoon fun skirt flight initial nasty transfer glory palm drama gossip remove fan joke shove label dune debate quick".to_string()),
      ("test7".to_string(),
          "noble width taxi input there patrol clown public spell aunt wish punch moment will misery eight excess arena pen turtle minimum grain vague inmate".to_string()),
      ("test8".to_string(),
          "cream sport mango believe inhale text fish rely elegant below earth april wall rug ritual blossom cherry detail length blind digital proof identify ride".to_string()),
      ("test9".to_string(),
          "index light average senior silent limit usual local involve delay update rack cause inmate wall render magnet common feature laundry exact casual resource hundred".to_string()),
      ("test10".to_string(),
          "prefer forget visit mistake mixture feel eyebrow autumn shop pair address airport diesel street pass vague innocent poem method awful require hurry unhappy shoulder".to_string()),
  ])
}

use cosmrs::{crypto::secp256k1::SigningKey, AccountId};

use thiserror::Error;

use super::network::Network;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Invalid mnemonic")]
    InvalidMnemonic,
    #[error("Derivation Error")]
    Derivation,
}

#[derive(Debug, Clone)]
pub struct Wallet {
    pub privkey: bip32::XPrv,
    pub pubkey: cosmrs::crypto::PublicKey,
    pub address: AccountId,
    pub (crate) network: Network,
}

impl Wallet {
    pub fn new(mnemonic: String, network: Network) -> Result<Self, WalletError> {
        let seed = bip39::Mnemonic::parse(mnemonic)
            .map_err(|_| WalletError::InvalidMnemonic)?
            .to_seed("");

        let privkey = bip32::XPrv::derive_from_path(&seed, &network.account_info.derivation_path)
            .map_err(|_| WalletError::Derivation)?;

        let pubkey = SigningKey::from(&privkey).public_key();

        let address = pubkey
            .account_id(&network.account_info.chain_prefix)
            .map_err(|_| WalletError::Derivation)?;

        Ok(Self {
            privkey,
            pubkey,
            address,
            network,
        })
    }

    pub fn signing_key(&self) -> SigningKey {
        SigningKey::from(&self.privkey)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_derived_properly() {
        let mnemonic = "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";
        let network = Network::default_localterra();
        let wallet = Wallet::new(mnemonic.to_string(), network).unwrap();
        assert_eq!(
            wallet.address.to_string(),
            "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v"
        );
    }

    #[test]
    fn errors_invalid_mnemonic() {
        let mnemonic = "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius invalid";
        let network = Network::default_localterra();
        let wallet = Wallet::new(mnemonic.to_string(), network);
        assert!(wallet.is_err());
    }
}
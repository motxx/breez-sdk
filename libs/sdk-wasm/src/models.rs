use strum_macros::Display;
use serde::{Deserialize, Serialize};

/// The different supported bitcoin networks
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq, Serialize, Deserialize)]
pub enum Network {
    /// Mainnet
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl From<String> for Network {
    fn from(network: String) -> Self {
        match network.to_lowercase().as_str() {
            "bitcoin" => Network::Bitcoin,
            "testnet" => Network::Testnet,
            "signet" => Network::Signet,
            "regtest" => Network::Regtest,
            _ => Network::Bitcoin, // TODO: Return None
        }
    }
}

impl From<bitcoin::network::constants::Network> for Network {
    fn from(network: bitcoin::network::constants::Network) -> Self {
        match network {
            bitcoin::network::constants::Network::Bitcoin => Network::Bitcoin,
            bitcoin::network::constants::Network::Testnet => Network::Testnet,
            bitcoin::network::constants::Network::Signet => Network::Signet,
            bitcoin::network::constants::Network::Regtest => Network::Regtest,
        }
    }
}

impl From<Network> for bitcoin::network::constants::Network {
    fn from(network: Network) -> Self {
        match network {
            Bitcoin => bitcoin::network::constants::Network::Bitcoin,
            Testnet => bitcoin::network::constants::Network::Testnet,
            Signet => bitcoin::network::constants::Network::Signet,
            Regtest => bitcoin::network::constants::Network::Regtest,
        }
    }
}

#[derive(Clone)]
pub struct SignerClientConfig {
    pub network: Network,
}

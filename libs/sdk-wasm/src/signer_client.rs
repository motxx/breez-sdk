use anyhow::Result;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};

pub type ClientResult<T, E = ClientError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Generic: {0}")]
    Generic(anyhow::Error),
}


/// Trait covering functions affecting the LN node
#[tonic::async_trait]
pub trait SignerClient: Send + Sync {
    async fn sign_message(&self, message: &str) -> ClientResult<String>;
    fn derive_bip32_key(&self, path: Vec<ChildNumber>) -> ClientResult<ExtendedPrivKey>;
}

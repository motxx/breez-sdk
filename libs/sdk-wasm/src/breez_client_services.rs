use std::sync::Arc;

use crate::error::{SdkError, SdkResult};
use crate::greenlight::GreenlightClient;
use crate::models::*;

use super::signer_client::SignerClient;

pub struct BreezClientServices {
    #[allow(dead_code)]
    config: SignerClientConfig,
    signer_client: Arc<dyn SignerClient>,
}

impl BreezClientServices {
    pub async fn connect(config: SignerClientConfig, seed: Vec<u8>) -> SdkResult<Arc<BreezClientServices>> {
        let breez_client_services = BreezClientServicesBuilder::new(config)
            .seed(seed)
            .build()
            .await?;
        Ok(breez_client_services)
    }

    pub async fn sign_message(&self, message: &str) -> SdkResult<String> {
        let signature = self.signer_client.sign_message(message).await?;
        Ok(signature)
    }
}

struct BreezClientServicesBuilder {
    config: SignerClientConfig,
    seed: Option<Vec<u8>>,
}

impl BreezClientServicesBuilder {
    pub fn new(config: SignerClientConfig) -> Self {
        Self {
            config,
            seed: None,
        }
    }

    pub fn seed(mut self, seed: Vec<u8>) -> Self {
        self.seed = Some(seed);
        self
    }

    pub async fn build(self) -> SdkResult<Arc<BreezClientServices>> {
        if self.seed.is_none() {
            return Err(SdkError::Generic {
                err: "seed is required".into()
            });
        }
        let seed = self.seed.clone().unwrap();
        let signer_client = Arc::new(GreenlightClient::connect(self.config.clone(), seed).await?);
        let breez_client_services = Arc::new(BreezClientServices {
            config: self.config.clone(),
            signer_client,
        });
        Ok(breez_client_services)
    }
}
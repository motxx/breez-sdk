use std::any;

use super::signer_client::ClientError;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@breez_client_service/error")]
extern "C" {
    #[wasm_bindgen]
    pub type GenericError;

    #[wasm_bindgen(constructor)]
    pub fn new(msg: String) -> GenericError;
}

pub type SdkResult<T, E = SdkError> = Result<T, E>;

/// General error returned by the SDK
#[derive(Debug, Error)]
pub enum SdkError {
    #[error("Generic: {err}")]
    Generic { err: String },

    #[error("Service connectivity: {err}")]
    ServiceConnectivity { err: String },
}

impl From<anyhow::Error> for SdkError {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic {
            err: err.to_string(),
        }
    }
}

impl From<SdkError> for JsValue {
    fn from(err: SdkError) -> JsValue {
        GenericError::new(err.to_string()).into()
    }
}

impl From<anyhow::Error> for ClientError {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic(err)
    }
}

impl From<bitcoin::util::bip32::Error> for ClientError {
    fn from(err: bitcoin::util::bip32::Error) -> Self {
        Self::Generic(anyhow::Error::new(err))
    }
}

impl From<ClientError> for SdkError {
    fn from(value: ClientError) -> Self {
        Self::Generic { err: value.to_string() }
    }
}

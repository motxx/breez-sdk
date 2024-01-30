use anyhow::Result;
use futures::Future;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;
use wasm_bindgen::prelude::*;

use super::breez_client_services::BreezClientServices;
use super::error::SdkError;
use super::models::*;

static BREEZ_CLIENT_SERVICES_INSTANCE: Lazy<Mutex<Option<Arc<BreezClientServices>>>> =
    Lazy::new(|| Mutex::new(None));
static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

/*  Breez Client Services API's */

/// Wrapper around [BreezClientServices::connect] which also initializes SDK logging
#[wasm_bindgen]
pub fn client_connect(network: String, seed: Vec<u8>) -> Result<(), JsValue> {
    block_on(async move {
        let mut locked = BREEZ_CLIENT_SERVICES_INSTANCE.lock().await;
        match *locked {
            None => {
                let breez_client_services =
                    BreezClientServices::connect(SignerClientConfig {
                      network: Network::from(network)
                    },
                    seed/*, TODO: Box::new(BindingEventListener {})*/
                )
                .await?;

                *locked = Some(breez_client_services);
                Ok(())
            }
            Some(_) => Err(SdkError::Generic {
                err: "Static node services already set, please call disconnect() first".into(),
            }),
        }
    })
    .map_err(SdkError::from)
    .map_err(JsValue::from)
}

/// See [BreezClientServices::sign_message]
#[wasm_bindgen]
pub fn sign_message(msg: String) -> Result<String, JsValue> {
    block_on(async { get_breez_client_services().await?.sign_message(&msg).await })
        .map_err(SdkError::from)
        .map_err(JsValue::from)
}

/// Check whether node service is initialized or not
#[wasm_bindgen]
pub fn is_initialized() -> bool {
    block_on(async { get_breez_client_services().await.is_ok() })
}

async fn get_breez_client_services() -> Result<Arc<BreezClientServices>, SdkError> {
    match BREEZ_CLIENT_SERVICES_INSTANCE.lock().await.as_ref() {
        None => Err(SdkError::Generic {
            err: "Node service was not initialized".into(),
        }),
        Some(sdk) => Ok(sdk.clone()),
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    rt().block_on(future)
}

pub(crate) fn rt() -> &'static tokio::runtime::Runtime {
    &RT
}

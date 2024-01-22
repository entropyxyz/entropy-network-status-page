pub mod app;
pub mod error_template;
pub mod fileserv;
pub mod program;
pub mod registered_account;

use cfg_if::cfg_if;
use pretty_bytes_rust::pretty_bytes;
use serde::{Deserialize, Serialize};
use std::fmt;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}}

cfg_if! { if #[cfg(feature = "ssr")] {
    use entropy_testing_utils::{
        test_client::{get_api, get_rpc},
    };
    use entropy_testing_utils::chain_api::EntropyConfig;
    use subxt::{backend::legacy::LegacyRpcMethods, OnlineClient};

    pub async fn get_api_rpc() -> (
        OnlineClient<EntropyConfig>,
        LegacyRpcMethods<EntropyConfig>,
    ) {

        let endpoint_addr =
            std::env::var("ENTROPY_TESTNET").unwrap_or("ws://localhost:9944".to_string());

        // TODO a panic here means the endpoint is unreachable - deal with this gracefully
        let api = get_api(&endpoint_addr).await.unwrap();
        let rpc = get_rpc(&endpoint_addr).await.unwrap();
        (api, rpc)
    }
}}

/// For displaying Vec<u8> nicely
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HexVec(Vec<u8>);

impl fmt::Display for HexVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() > 3 {
            write!(
                f,
                "0x{}…{}",
                hex::encode(&self.0.get(0..2).unwrap()),
                hex::encode(&self.0.get(self.0.len() - 2..).unwrap())
            )
        } else if !self.0.is_empty() {
            write!(f, "0x{}", hex::encode(&self.0))
        } else {
            write!(f, "")
        }
    }
}

impl fmt::Debug for HexVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

pub fn display_bytes(bytes: u64) -> String {
    match bytes {
        0 => "0".to_string(),
        _ => pretty_bytes(
            bytes,
            Some(pretty_bytes_rust::PrettyBytesOptions {
                use_1024_instead_of_1000: Some(true),
                number_of_decimal: None,
                remove_zero_decimal: Some(true),
            }),
        ),
    }
}

use cfg_if::cfg_if;
pub mod app;
pub mod error_template;
pub mod fileserv;
pub mod program;
pub mod registered_account;

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

        let api = get_api(&endpoint_addr).await.unwrap();
        let rpc = get_rpc(&endpoint_addr).await.unwrap();
        (api, rpc)
    }
}}

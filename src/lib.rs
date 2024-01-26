pub mod app;
pub mod error_template;
pub mod fileserv;
pub mod program;
pub mod registered_account;
pub mod validator;

use cfg_if::cfg_if;
use leptos::*;
use pretty_bytes_rust::pretty_bytes;
use serde::{Deserialize, Serialize};
use std::fmt;

cfg_if! { if #[cfg(feature = "hydrate")] {
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

#[server(GetChainEndpoint, "/api")]
pub async fn get_chain_endpoint() -> Result<String, ServerFnError> {
    Ok(std::env::var("ENTROPY_TESTNET_ENDPOINT").unwrap_or("ws://localhost:9944".to_string()))
}

cfg_if! { if #[cfg(feature = "ssr")] {
    use entropy_testing_utils::{
        test_client::{get_api, get_rpc},
    };
    use entropy_testing_utils::chain_api::EntropyConfig;
    use subxt::{backend::legacy::LegacyRpcMethods, OnlineClient};

    /// Backend function for getting the chain API
    pub async fn get_api_rpc() -> Result<(
        OnlineClient<EntropyConfig>,
        LegacyRpcMethods<EntropyConfig>,
    ), ServerFnError> {

        let endpoint_addr = get_chain_endpoint().await?;

        // TODO a panic here means the endpoint is unreachable - deal with this gracefully
        let api = get_api(&endpoint_addr).await?;
        let rpc = get_rpc(&endpoint_addr).await?;
        Ok((api, rpc))
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

/// For diplaying sizes nicely
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

/// A table with given headings and a title
#[component]
pub fn DetailsTable(
    title: &'static str,
    headings: Vec<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <h2 class="my-4 block font-sans text-xl antialiased leading-snug tracking-normal mt-4 text-gray-700">
            {title}
        </h2>
        <div class="relative flex flex-col w-full h-full text-gray-700 bg-white shadow-md rounded-xl bg-clip-border">
            <table class="w-full text-left table-auto min-w-max">
                <thead>
                    <tr>
                        {headings
                            .into_iter()
                            .map(|heading| {
                                view! {
                                    <th class="p-4 border-b border-blue-gray-100 bg-blue-50">
                                        <p class="block font-sans text-sm antialiased font-normal leading-none text-blue-gray-900 opacity-70">
                                            {heading}
                                        </p>
                                    </th>
                                }
                            })
                            .collect::<Vec<_>>()}

                    </tr>
                </thead>
                <tbody>{children()}</tbody>
            </table>
        </div>
    }
}

/// Copyable table data
#[component]
pub fn DisplayValue(value: String, long_value: Option<String>) -> impl IntoView {
    let long_value = long_value.unwrap_or(value.clone());
    let (long_value, _set_long_value) = create_signal(long_value);
    let copy = move |_| {
        cfg_if! { if #[cfg(feature = "hydrate")] {
            #[cfg(web_sys_unstable_apis)]
            wasm_bindgen_futures::spawn_local(async move {
                let window = web_sys::window().unwrap();
                match window.navigator().clipboard() {
                    Some(clipboard) => {
                        let promise = clipboard.write_text(&long_value.get_untracked());
                        let _result = wasm_bindgen_futures::JsFuture::from(promise)
                            .await
                            .unwrap();
                        log::info!("Copied to clipboard");
                    }
                    None => {
                        log::warn!("Failed to copy to clipboard");
                    }
                }
            });
        }}
    };
    view! {
        <td

            title=move || format!("Click to copy {}", long_value.get())
            on:click=copy
        >
            <p class="block font-sans text-sm antialiased font-normal leading-normal text-blue-gray-900">
                <code class="hover:font-extrabold p-4">{value}</code>
            </p>
        </td>
    }
}

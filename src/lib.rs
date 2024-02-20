pub mod app;
pub mod chain_api;
pub mod error_template;
pub mod program;
pub mod registered_account;
pub mod validator;

use leptos::*;
use pretty_bytes_rust::pretty_bytes;
use serde::{Deserialize, Serialize};
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/endpoint.rs"));

use crate::chain_api::{get_api, get_rpc, EntropyConfig};
use subxt::{backend::legacy::LegacyRpcMethods, OnlineClient};
pub async fn get_api_rpc(
) -> Result<(OnlineClient<EntropyConfig>, LegacyRpcMethods<EntropyConfig>), ServerFnError> {
    let api = get_api(ENTROPY_NETWORK_ENDPOINT).await?;
    let rpc = get_rpc(ENTROPY_NETWORK_ENDPOINT).await?;
    Ok((api, rpc))
}

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
        #[cfg(web_sys_unstable_apis)]
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            match window.navigator().clipboard() {
                Some(clipboard) => {
                    let promise = clipboard.write_text(&long_value.get_untracked());
                    let _result = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                    log::info!("Copied to clipboard");
                }
                None => {
                    log::warn!("Failed to copy to clipboard");
                }
            }
        });
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

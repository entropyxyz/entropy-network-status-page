use crate::HexVec;
use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};
use subxt::utils::AccountId32;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredAccount {
    pub account_id: AccountId32,
    pub key_visibility: String,
    pub verifying_key: HexVec,
    pub program_pointers: Vec<String>,
    pub program_modification_account: String,
}

#[component]
pub fn RegisteredAccount(account: RegisteredAccount) -> impl IntoView {
    view! {
        <tr>
            <td>{account.account_id.to_string()}</td>
            <td>{account.key_visibility}</td>
            <td>{account.program_modification_account}</td>
            <DisplayValue value={account.verifying_key.to_string()} long_value={format!("{:?}", account.verifying_key)} />
            <td>{account.program_pointers}</td>
        </tr>
    }
}

#[component]
pub fn DisplayValue(value: String, long_value: String) -> impl IntoView {
    let (long_value, _set_long_value) = create_signal(long_value);
    let copy = move |_| {
        cfg_if! { if #[cfg(feature = "hydrate")] {
            log::info!("Copying...");
            use wasm_bindgen_futures::spawn_local;
            #[cfg(web_sys_unstable_apis)]
            spawn_local(async move {

            log::info!("about to copy...");
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
        <td class="hover:bg-gray-200" title={move || format!("Click to copy {}", long_value.get())} on:click=copy><code>{value}</code></td>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use entropy_testing_utils::{
            chain_api::entropy::runtime_types::pallet_relayer::pallet::RegisteredInfo,
        };
        use entropy_shared::KeyVisibility;

        impl RegisteredAccount {
            fn new(account_id: AccountId32, registered_info: RegisteredInfo) -> RegisteredAccount {
                RegisteredAccount {
                    account_id,
                    key_visibility: match registered_info.key_visibility.0 {
                        KeyVisibility::Public => "Public",
                        KeyVisibility::Permissioned => "Permissioned",
                        KeyVisibility::Private(_) => "Private",
                    }.to_string(),
                    verifying_key: HexVec(registered_info.verifying_key.0),
                    program_pointers: registered_info.programs_data.0.into_iter().map(|program_instance| format!("{}", program_instance.program_pointer)).collect(),
                    program_modification_account: registered_info.program_modification_account.to_string(),
                }
            }
        }
    }
}

#[server(GetRegisteredAccounts, "/api")]
pub async fn get_registered_accounts() -> Result<Vec<RegisteredAccount>, ServerFnError> {
    use crate::get_api_rpc;
    use entropy_testing_utils::test_client::get_accounts;

    let (api, rpc) = get_api_rpc().await;

    let accounts = get_accounts(&api, &rpc)
        .await
        .unwrap() // TODO
        .into_iter()
        .map(|(account_id, registered_info)| RegisteredAccount::new(account_id, registered_info))
        .collect();

    Ok(accounts)
}

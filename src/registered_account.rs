use leptos::*;
use serde::{Deserialize, Serialize};
use subxt::utils::AccountId32;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredAccount {
    pub account_id: AccountId32,
    pub key_visibility: String,
    pub verifying_key: String,
    pub program_pointers: Vec<String>,
    pub program_modification_account: String,
}

#[component]
pub fn RegisteredAccount(account: RegisteredAccount) -> impl IntoView {
    view! {
        <tr class="hover:bg-gray-200">
            <td>{account.account_id.to_string()}</td>
            <td>{account.key_visibility}</td>
            <td>{account.program_modification_account}</td>
            <td>{account.verifying_key}</td>
            <td>{account.program_pointers}</td>
        </tr>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use entropy_testing_utils::{
            chain_api::entropy::runtime_types::pallet_relayer::pallet::RegisteredInfo,
            test_client::{get_api, get_rpc},
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
                    verifying_key: hex::encode(registered_info.verifying_key.0),
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

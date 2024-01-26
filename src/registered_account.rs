use crate::{DisplayValue, HexVec};
use leptos::*;
use serde::{Deserialize, Serialize};
use subxt::utils::AccountId32;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredAccount {
    pub account_id: AccountId32,
    pub key_visibility: (String, String),
    pub verifying_key: HexVec,
    pub program_pointers: Vec<String>,
    pub program_modification_account: String,
}

#[component]
pub fn RegisteredAccount(account: RegisteredAccount) -> impl IntoView {
    view! {
        <tr class="hover:bg-gray-200">
            <DisplayValue value=account.account_id.to_string() long_value=None/>
            <KeyVisibility key_visibility=account.key_visibility.0 color=account.key_visibility.1/>
            <DisplayValue value=account.program_modification_account long_value=None/>
            <DisplayValue
                value=account.verifying_key.to_string()
                long_value=Some(format!("{:?}", account.verifying_key))
            />
            <td class="p-4">
                <p class="block font-sans text-sm antialiased font-normal leading-normal text-blue-gray-900">
                    {account.program_pointers}
                </p>
            </td>
        </tr>
    }
}

#[component]
pub fn KeyVisibility(key_visibility: String, color: String) -> impl IntoView {
    let style = format!("relative grid items-center px-2 py-1 font-sans text-xs font-bold text-{}-900 uppercase rounded-md select-none whitespace-nowrap bg-{}-500/20", &color, &color);
    view! {
        <td class="px-4">
            <div class=style>{key_visibility}</div>
        </td>
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
                        KeyVisibility::Public => ("Public".to_string(), "green".to_string()),
                        KeyVisibility::Permissioned => ("Permissioned".to_string(), "amber".to_string()),
                        KeyVisibility::Private(_) => ("Private".to_string(), "red".to_string()),
                    },
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

    let (api, rpc) = get_api_rpc().await?;

    let accounts = get_accounts(&api, &rpc)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .into_iter()
        .map(|(account_id, registered_info)| RegisteredAccount::new(account_id, registered_info))
        .collect();

    Ok(accounts)
}

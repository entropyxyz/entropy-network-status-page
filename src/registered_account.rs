use crate::{
    chain_api::{entropy::runtime_types::pallet_relayer::pallet::RegisteredInfo, EntropyConfig},
    get_api_rpc, DisplayValue, HexVec,
};
use anyhow::anyhow;
use entropy_shared::KeyVisibility;
use leptos::*;
use parity_scale_codec::Decode;
use serde::{Deserialize, Serialize};
use subxt::{backend::legacy::LegacyRpcMethods, utils::AccountId32, OnlineClient};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredAccount {
    pub account_id: AccountId32,
    pub key_visibility: (String, String),
    pub verifying_key: HexVec,
    pub program_pointers: Vec<String>,
    pub program_modification_account: String,
}

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
            program_pointers: registered_info
                .programs_data
                .0
                .into_iter()
                .map(|program_instance| format!("{}", program_instance.program_pointer))
                .collect(),
            program_modification_account: registered_info.program_modification_account.to_string(),
        }
    }
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

pub async fn get_registered_accounts() -> Result<Vec<RegisteredAccount>, ServerFnError> {
    let (api, rpc) = get_api_rpc().await?;

    let accounts = get_accounts(&api, &rpc)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .into_iter()
        .map(|(account_id, registered_info)| RegisteredAccount::new(account_id, registered_info))
        .collect();

    Ok(accounts)
}

/// Get info on all registered accounts
pub async fn get_accounts(
    api: &OnlineClient<EntropyConfig>,
    rpc: &LegacyRpcMethods<EntropyConfig>,
) -> anyhow::Result<Vec<(AccountId32, RegisteredInfo)>> {
    let block_hash = rpc
        .chain_get_block_hash(None)
        .await?
        .ok_or_else(|| anyhow!("Error getting block hash"))?;
    let keys = Vec::<()>::new();
    let storage_address = subxt::dynamic::storage("Relayer", "Registered", keys);
    let mut iter = api.storage().at(block_hash).iter(storage_address).await?;
    let mut accounts = Vec::new();
    while let Some(Ok((storage_key, account))) = iter.next().await {
        let decoded = account.into_encoded();
        let registered_info = RegisteredInfo::decode(&mut decoded.as_ref())?;
        let key: [u8; 32] = storage_key[storage_key.len() - 32..].try_into()?;
        accounts.push((AccountId32(key), registered_info))
    }
    Ok(accounts)
}

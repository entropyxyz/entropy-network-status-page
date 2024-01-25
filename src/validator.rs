use crate::{DisplayValue, HexVec};
use leptos::*;
use serde::{Deserialize, Serialize};
use subxt::utils::AccountId32;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Validator {
    tss_account: AccountId32,
    x25519_public_key: HexVec,
    endpoint: String,
}

#[component]
pub fn Validator(validator: Validator) -> impl IntoView {
    view! {
        <tr class="hover:bg-gray-200">
            <DisplayValue value={validator.tss_account.to_string()} long_value={None} />
            <DisplayValue value={validator.x25519_public_key.to_string()} long_value={Some(format!("{:?}", validator.x25519_public_key))} />
            <DisplayValue value={validator.endpoint} long_value={None} />
        </tr>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use entropy_testing_utils::{
            chain_api::entropy::runtime_types::pallet_staking_extension::pallet::ServerInfo,
        };

        impl Validator {
            fn new(_stash_account: AccountId32, server_info: ServerInfo<AccountId32>) -> Validator {
                Validator {
                    tss_account: server_info.tss_account,
                    x25519_public_key: HexVec(server_info.x25519_public_key.to_vec()),
                    endpoint: String::from_utf8(server_info.endpoint).unwrap(),
                }
            }
        }
    }
}

#[server(GetValidators, "/api")]
pub async fn get_validators() -> Result<Vec<Validator>, ServerFnError> {
    use crate::get_api_rpc;
    use anyhow::anyhow;
    use entropy_testing_utils::chain_api::EntropyConfig;
    use parity_scale_codec::Decode;
    use subxt::{backend::legacy::LegacyRpcMethods, Config, OnlineClient};

    async fn get_validators_internal(
        api: &OnlineClient<EntropyConfig>,
        rpc: &LegacyRpcMethods<EntropyConfig>,
    ) -> anyhow::Result<
        Vec<(
            AccountId32,
            ServerInfo<<EntropyConfig as Config>::AccountId>,
        )>,
    > {
        let block_hash = rpc
            .chain_get_block_hash(None)
            .await?
            .ok_or_else(|| anyhow!("Error getting block hash"))?;
        let keys = Vec::<()>::new();
        let storage_address = subxt::dynamic::storage("StakingExtension", "ThresholdServers", keys);
        let mut iter = api.storage().at(block_hash).iter(storage_address).await?;
        let mut validators = Vec::new();
        while let Some(Ok((storage_key, account))) = iter.next().await {
            let decoded = account.into_encoded();
            let server_info: ServerInfo<<EntropyConfig as Config>::AccountId> =
                ServerInfo::decode(&mut decoded.as_ref())?;
            let key: [u8; 32] = storage_key[storage_key.len() - 32..].try_into()?;
            validators.push((AccountId32(key), server_info))
        }
        Ok(validators)
    }

    let (api, rpc) = get_api_rpc().await;

    let validators = get_validators_internal(&api, &rpc)
        .await
        .unwrap() // TODO
        .into_iter()
        .map(|(stash_account, server_info)| Validator::new(stash_account, server_info))
        .collect();

    Ok(validators)
}

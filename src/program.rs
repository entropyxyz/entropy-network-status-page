use crate::{display_bytes, DisplayValue};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub hash: String,
    pub deployer: String,
    pub ref_counter: u128,
    pub size: usize,
    pub configurable: bool,
    pub name: Option<String>,
}

#[component]
pub fn Program(program: Program) -> impl IntoView {
    view! {
        <tr class="hover:bg-gray-200">
            <DisplayValue
                value=program.hash.to_string()
                long_value=Some(format!("{:?}", program.hash))
            />
            <DisplayValue value=program.deployer long_value=None/>
            <td class="p-4">
                <p class="block font-sans text-sm antialiased font-normal leading-normal text-blue-gray-900">
                    {program.ref_counter}
                </p>
            </td>
            <td class="p-4">
                <p class="block font-sans text-sm antialiased font-normal leading-normal text-blue-gray-900">
                    {display_bytes(program.size as u64)}
                </p>
            </td>
            <td class="p-4">
                <p class="block font-sans text-sm antialiased font-normal leading-normal text-blue-gray-900">
                    {program.configurable}
                </p>
            </td>
            <td class="p-4">
                <p class="block font-sans text-sm antialiased font-normal leading-normal text-blue-gray-900">
                    {program.name.unwrap_or_default()}
                </p>
            </td>
        </tr>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use entropy_testing_utils::{
            chain_api::entropy::runtime_types::pallet_programs::pallet::ProgramInfo,
        };
        use subxt::utils::{AccountId32, H256};
        use cargo_metadata::Package;
        const PROGRAM_METADATA_SERVICE: &str = "http://127.0.0.1:3000";

        impl Program {
            fn new(hash: H256, program_info: ProgramInfo<AccountId32>, metadata: Option<Package>) -> Program {
                let name = metadata.map(|m| m.name);
                Program {
                    hash: hash.to_string(),
                    deployer: program_info.deployer.to_string(),
                    ref_counter: program_info.ref_counter,
                    size: program_info.bytecode.len(),
                    // TODO: If configuration interface is json we could display it. Waiting till
                    // we have an example of a program with a configuration interface
                    configurable: !program_info.configuration_interface.is_empty(),
                    name,
                }
            }
        }

        /// Attempt to get program's metadata from the http service. If there is any problem,
        /// return None
        async fn get_program_metadata(hash: H256) -> Option<Package> {
            let response_string = reqwest::get(format!(
                "{}/program/{}",
                PROGRAM_METADATA_SERVICE,
                hex::encode(hash)
            ))
            .await
            .ok()?
            .text()
            .await
            .ok()?;

            let package: Package = serde_json::from_str(&response_string).ok()?;
            Some(package)
        }
    }
}

#[server(GetStoredPrograms, "/api")]
pub async fn get_stored_programs() -> Result<Vec<Program>, ServerFnError> {
    use crate::get_api_rpc;
    use entropy_testing_utils::test_client::get_programs;
    use futures::stream::{self, StreamExt};

    let (api, rpc) = get_api_rpc().await?;

    let programs_iter = get_programs(&api, &rpc)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .into_iter()
        .map(|(hash, program_info)| async move {
            let metadata = get_program_metadata(hash).await;
            Program::new(hash, program_info, metadata)
        });

    // Only allow 3 concurrent http requests at a time (TODO could speed things up by increasing
    // this)
    let programs_stream = stream::iter(programs_iter).buffer_unordered(3);
    Ok(programs_stream.collect::<Vec<Program>>().await)
}

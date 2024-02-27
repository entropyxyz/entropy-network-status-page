use crate::chain_api::entropy::runtime_types::pallet_programs::pallet::ProgramInfo;
use crate::chain_api::EntropyConfig;
use crate::get_api_rpc;
use crate::{display_bytes, DisplayValue};
use anyhow::anyhow;
use cargo_metadata::Package;
use futures::stream::{self, StreamExt};
use leptos::*;
use parity_scale_codec::Decode;
use serde::{Deserialize, Serialize};
use subxt::{
    backend::legacy::LegacyRpcMethods,
    utils::{AccountId32, H256},
    Config, OnlineClient,
};

const PROGRAM_METADATA_SERVICE: &str = "http://127.0.0.1:3000";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub hash: String,
    pub deployer: String,
    pub ref_counter: u128,
    pub size: usize,
    pub configurable: bool,
    pub metadata: Option<ProgramMetadata>,
}

impl Program {
    fn new(
        hash: H256,
        program_info: ProgramInfo<AccountId32>,
        metadata: Option<Package>,
    ) -> Program {
        Program {
            hash: hash.to_string(),
            deployer: program_info.deployer.to_string(),
            ref_counter: program_info.ref_counter,
            size: program_info.bytecode.len(),
            // TODO: If configuration interface is json we could display it. Waiting till
            // we have an example of a program with a configuration interface
            configurable: !program_info.configuration_interface.is_empty(),
            metadata: metadata.map(|m| ProgramMetadata::new(m)),
        }
    }
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
                    <ProgramMetadata metadata=program.metadata/>
                </p>
            </td>
        </tr>
    }
}

pub async fn get_stored_programs() -> Result<Vec<Program>, ServerFnError> {
    let (api, rpc) = get_api_rpc().await?;

    let programs_iter = get_programs(&api, &rpc)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .into_iter()
        // .map(|(hash, program_info)| Program::new(hash, program_info))
        // .collect();
        .map(|(hash, program_info)| async move {
            let metadata = get_program_metadata(hash).await;
            Program::new(hash, program_info, metadata)
        });

    // Only allow 3 concurrent http requests at a time (TODO could speed things up by increasing
    // this)
    let programs_stream = stream::iter(programs_iter).buffer_unordered(3);
    Ok(programs_stream.collect::<Vec<Program>>().await)
}

/// Get details of all stored programs
pub async fn get_programs(
    api: &OnlineClient<EntropyConfig>,
    rpc: &LegacyRpcMethods<EntropyConfig>,
) -> anyhow::Result<Vec<(H256, ProgramInfo<<EntropyConfig as Config>::AccountId>)>> {
    let block_hash = rpc
        .chain_get_block_hash(None)
        .await?
        .ok_or_else(|| anyhow!("Error getting block hash"))?;
    let keys = Vec::<()>::new();
    let storage_address = subxt::dynamic::storage("Programs", "Programs", keys);
    let mut iter = api.storage().at(block_hash).iter(storage_address).await?;
    let mut programs = Vec::new();
    while let Some(Ok((storage_key, program))) = iter.next().await {
        let decoded = program.into_encoded();
        let program_info: ProgramInfo<<EntropyConfig as Config>::AccountId> =
            ProgramInfo::decode(&mut decoded.as_ref())?;
        let hash: [u8; 32] = storage_key[storage_key.len() - 32..].try_into()?;
        programs.push((H256(hash), program_info));
    }
    Ok(programs)
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub docker_image: Option<String>,
}

impl ProgramMetadata {
    fn new(package: Package) -> ProgramMetadata {
        ProgramMetadata {
            name: package.name,
            version: package.version.to_string(),
            description: package.description,
            license: package.license,
            repository: package.repository,
            docker_image: get_docker_image_name_from_metadata(&package.metadata),
        }
    }
}

#[component]
pub fn ProgramMetadata(metadata: Option<ProgramMetadata>) -> impl IntoView {
    match metadata {
        Some(metadata) => view! {
            <ul>
                <ProgramMetadataItem name="Name">{metadata.name}</ProgramMetadataItem>
                <ProgramMetadataItem name="Version">{metadata.version}</ProgramMetadataItem>
                {if let Some(description) = metadata.description {
                    view! {
                        <span>
                            <ProgramMetadataItem name="Description">
                                {description}
                            </ProgramMetadataItem>
                        </span>
                    }
                } else {
                    view! { <span></span> }
                }}

                {if let Some(license) = metadata.license {
                    view! {
                        <span>
                            <ProgramMetadataItem name="License">{license}</ProgramMetadataItem>
                        </span>
                    }
                } else {
                    view! { <span></span> }
                }}

                {if let Some(repository) = metadata.repository {
                    view! {
                        <span>
                            <ProgramMetadataItem name="Repository">
                                <a
                                    href=&repository
                                    target="_blank"
                                    class="underline text-blue-600 hover:text-blue-800 visited:text-purple-600"
                                >
                                    {repository}
                                </a>
                            </ProgramMetadataItem>
                        </span>
                    }
                } else {
                    view! { <span></span> }
                }}

                {if let Some(docker_image) = metadata.docker_image {
                    view! {
                        <span>
                            <ProgramMetadataItem name="Docker image">
                                {docker_image}
                            </ProgramMetadataItem>
                        </span>
                    }
                } else {
                    view! { <span></span> }
                }}

            </ul>
        },
        None => view! { <ul></ul> },
    }
}

#[component]
pub fn ProgramMetadataItem(name: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <strong>{format!("{}: ", name)}</strong>
            {children()}
        </li>
    }
}

/// We expect there to be a docker image given in the Cargo.toml file like so:
/// ```toml
/// [package.metadata.entropy-program]
/// docker-image = "peg997/build-entropy-programs:version0.1"
/// ```
fn get_docker_image_name_from_metadata(metadata: &serde_json::value::Value) -> Option<String> {
    if let serde_json::value::Value::Object(m) = metadata {
        if let Some(serde_json::value::Value::Object(p)) = m.get("entropy-program") {
            if let Some(serde_json::value::Value::String(image_name)) = p.get("docker-image") {
                return Some(image_name.clone());
            }
        }
    }
    None
}

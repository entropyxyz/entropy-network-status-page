use crate::{display_bytes, DisplayValue};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub hash: String,
    pub stored_by: String,
    pub ref_counter: u128,
    pub size: usize,
    pub configurable: bool,
}

#[component]
pub fn Program(program: Program) -> impl IntoView {
    view! {
        <tr class="hover:bg-gray-200 text-right">
            <DisplayValue value={program.hash.to_string()} long_value={Some(format!("{:?}", program.hash))} />
            <DisplayValue value={program.stored_by} long_value={None} />
            <td class="px-4">{program.ref_counter}</td>
            <td class="px-4">{display_bytes(program.size as u64)}</td>
            <td class="px-4">{program.configurable}</td>
        </tr>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use entropy_testing_utils::{
            chain_api::entropy::runtime_types::pallet_programs::pallet::ProgramInfo,
        };
        use subxt::utils::{AccountId32, H256};

        impl Program {
            fn new(hash: H256, program_info: ProgramInfo<AccountId32>) -> Program {
                Program {
                    hash: hash.to_string(),
                    stored_by: program_info.program_modification_account.to_string(),
                    ref_counter: program_info.ref_counter,
                    size: program_info.bytecode.len(),
                    configurable: !program_info.program_type_definition.is_empty(),
                }
            }
        }
    }
}

#[server(GetStoredPrograms, "/api")]
pub async fn get_stored_programs() -> Result<Vec<Program>, ServerFnError> {
    use crate::get_api_rpc;
    use entropy_testing_utils::test_client::get_programs;

    let (api, rpc) = get_api_rpc().await?;

    let programs = get_programs(&api, &rpc)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .into_iter()
        .map(|(hash, program_info)| Program::new(hash, program_info))
        .collect();

    Ok(programs)
}

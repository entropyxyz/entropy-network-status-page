use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub hash: String,
    pub stored_by: String,
    pub ref_counter: u128,
    pub size: usize,
}

#[component]
pub fn Program(program: Program) -> impl IntoView {
    view! {
        <tr>
            <td>{program.hash}</td>
            <td>{program.stored_by}</td>
            <td>{program.ref_counter}</td>
            <td>{program.size}</td>
        </tr>
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use entropy_testing_utils::{
            chain_api::entropy::runtime_types::pallet_programs::pallet::ProgramInfo,
            test_client::{get_api, get_rpc},
        };
        use subxt::utils::{AccountId32, H256};

        impl Program {
            fn new(hash: H256, program_info: ProgramInfo<AccountId32>) -> Program {
                Program {
                    hash: hash.to_string(),
                    stored_by: program_info.program_modification_account.to_string(),
                    ref_counter: program_info.ref_counter,
                    size: program_info.bytecode.len(),
                }
            }
        }
    }
}

#[server(GetStoredPrograms, "/api")]
pub async fn get_stored_programs() -> Result<Vec<Program>, ServerFnError> {
    use entropy_testing_utils::test_client::get_programs;

    let endpoint_addr =
        std::env::var("ENTROPY_TESTNET").unwrap_or("ws://localhost:9944".to_string());

    let api = get_api(&endpoint_addr).await?;
    let rpc = get_rpc(&endpoint_addr).await?;

    let programs = get_programs(&api, &rpc)
        .await
        .unwrap() // TODO
        .into_iter()
        .map(|(hash, program_info)| Program::new(hash, program_info))
        .collect();

    Ok(programs)
}

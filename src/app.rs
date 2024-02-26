use crate::{
    error_template::{AppError, ErrorTemplate},
    program::{get_stored_programs, Program},
    registered_account::{get_registered_accounts, RegisteredAccount},
    validator::{get_validators, Validator},
    DetailsTable,
};
use leptos::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text=format!("Entropy {} Status Page", crate::ENTROPY_NETWORK_NAME)/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let accounts = create_resource(|| (), move |_| get_registered_accounts());
    let programs = create_resource(|| (), move |_| get_stored_programs());
    let validators = create_resource(|| (), move |_| get_validators());
    let loading = move || view! { <p>"Loading..."</p> };
    view! {
        <div class="container mx-auto">
            <h1 class="text-2xl my-4">
                {format!("Entropy {} Status Page", crate::ENTROPY_NETWORK_NAME)}
            </h1>
            <p class="text-sm text-blue-gray-900">
                "Chain endpoint: " <code>{crate::ENTROPY_NETWORK_ENDPOINT}</code>
            </p>
            <Transition fallback=loading>
                {move || {
                    let existing_accounts = {
                        move || {
                            accounts
                                .get()
                                .map(move |accounts| match accounts {
                                    Err(e) => {
                                        view! {
                                            <pre class="error">"server error: " {e.to_string()}</pre>
                                        }
                                            .into_view()
                                    }
                                    Ok(accounts) => {
                                        if accounts.is_empty() {
                                            view! {
                                                <tr>
                                                    <td>"No registered accounts."</td>
                                                </tr>
                                            }
                                                .into_view()
                                        } else {
                                            accounts
                                                .into_iter()
                                                .map(move |account| {
                                                    view! { <RegisteredAccount account/> }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };
                    view! {
                        <DetailsTable
                            title="Registered Entropy Accounts"
                            headings=vec![
                                "Account ID",
                                "Access Mode",
                                "Program Modification Account",
                                "Verifying Key",
                                "Programs",
                            ]
                        >

                            {existing_accounts}
                        </DetailsTable>
                    }
                }}

            </Transition>
            <Transition fallback=loading>
                {move || {
                    let stored_programs = {
                        move || {
                            programs
                                .get()
                                .map(move |programs| match programs {
                                    Err(e) => {
                                        view! {
                                            <pre class="error">"server error: " {e.to_string()}</pre>
                                        }
                                            .into_view()
                                    }
                                    Ok(programs) => {
                                        if programs.is_empty() {
                                            view! {
                                                <tr>
                                                    <td>"No stored programs."</td>
                                                </tr>
                                            }
                                                .into_view()
                                        } else {
                                            programs
                                                .into_iter()
                                                .map(move |program| {
                                                    view! { <Program program/> }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };
                    view! {
                        <DetailsTable
                            title="Programs"
                            headings=vec![
                                "Hash",
                                "Stored by Account ID",
                                "Times Used",
                                "Size",
                                "Configurable?",
                                "Metadata",
                            ]
                        >

                            {stored_programs}
                        </DetailsTable>
                    }
                }}

            </Transition>

            <Transition fallback=move || {
                view! { <p>"loading..."</p> }
            }>
                {move || {
                    let current_validators = {
                        move || {
                            validators
                                .get()
                                .map(move |validators| match validators {
                                    Err(e) => {
                                        view! {
                                            <pre class="error">"server error: " {e.to_string()}</pre>
                                        }
                                            .into_view()
                                    }
                                    Ok(validators) => {
                                        if validators.is_empty() {
                                            view! {
                                                <tr>
                                                    <td>"No validators."</td>
                                                </tr>
                                            }
                                                .into_view()
                                        } else {
                                            validators
                                                .into_iter()
                                                .map(move |validator| {
                                                    view! { <Validator validator/> }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };
                    view! {
                        <DetailsTable
                            title="Validators"
                            headings=vec!["TSS Account ID", "X25519 Public Key", "Socket Address"]
                        >
                            {current_validators}
                        </DetailsTable>
                    }
                }}

            </Transition>
        </div>
    }
}

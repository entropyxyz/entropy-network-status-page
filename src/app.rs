use crate::{
    error_template::{AppError, ErrorTemplate},
    program::{get_stored_programs, Program},
    registered_account::{get_registered_accounts, RegisteredAccount},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/entropy-testnet-web-ui.css"/>

        <Title text="Entropy Testnet Web UI"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
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
    view! {
        <h1>"Entropy Testnet Web UI"</h1>
        <h2>"Registered entropy accounts"</h2>
            <Transition fallback=move || view! {<p>"loading..."</p> }>
        {move || {
                     let existing_accounts = {
                         move || {
                             accounts.get()
                                    .map(move |accounts| match accounts {
                                        Err(e) => {
                                            view! { <pre class="error">"server error: " {e.to_string()}</pre>}.into_view()
                                        }
                                        Ok(accounts) => {
                                            if accounts.is_empty() {
                                                view! { <p>"no registered accounts."</p> }.into_view()
                                            } else {
                                                accounts
                                                    .into_iter()
                                                    .map(move |account| {
                                                        view! { <RegisteredAccount account />
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }
                        };

                        view! {
                            <table>
                                <tr>
                                  <th>"Account ID"</th>
                                  <th>"Access Mode"</th>
                                  <th>"Program Modification Account"</th>
                                  <th>"Verifying Key"</th>
                                  <th>"Programs"</th>
                                </tr>
                                {existing_accounts}
                            </table>
                        }
                    }
                }
            </Transition>

        <h2>"Programs"</h2>
            <Transition fallback=move || view! {<p>"loading..."</p> }>
        {move || {
                     let stored_programs = {
                         move || {
                             programs.get()
                                    .map(move |programs| match programs {
                                        Err(e) => {
                                            view! { <pre class="error">"server error: " {e.to_string()}</pre>}.into_view()
                                        }
                                        Ok(programs) => {
                                            if programs.is_empty() {
                                                view! { <p>"No stored programs."</p> }.into_view()
                                            } else {
                                                programs
                                                    .into_iter()
                                                    .map(move |program| {
                                                        view! { <Program program />
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }
                        };

                        view! {
                            <table>
                                <tr>
                                  <th>"Hash"</th>
                                  <th>"Stored by Account ID"</th>
                                  <th>"Times used"</th>
                                  <th>"Size"</th>
                                </tr>
                                {stored_programs}
                            </table>
                        }
                    }
                }
            </Transition>
    }
}

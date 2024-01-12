use crate::{
    error_template::{AppError, ErrorTemplate},
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
        <Stylesheet id="leptos" href="/pkg/try-leptos-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
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
    view! {
        <h1>"Registered Entropy Accounts"</h1>
            <Transition fallback=move || view! {<p>"Loading..."</p> }>
        {move || {
                     let existing_accounts = {
                         move || {
                             accounts.get()
                                    .map(move |accounts| match accounts {
                                        Err(e) => {
                                            view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                        }
                                        Ok(accounts) => {
                                            if accounts.is_empty() {
                                                view! { <p>"No registered accounts."</p> }.into_view()
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
                                  <th>"Account Id"</th>
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
    }
}

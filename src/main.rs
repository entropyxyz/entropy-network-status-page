#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use axum_server::tls_rustls::RustlsConfig;
    use entropy_network_status_page::{app::*, fileserv::file_and_error_handler};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::path::PathBuf;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    match std::env::var("TLS_CERT_LOCATION") {
        Ok(location) => {
            // optional: spawn a second server to redirect http requests to this server
            tokio::spawn(redirect_http_to_https(addr.ip(), 80, 443));

            let config = RustlsConfig::from_pem_file(
                PathBuf::from(&location).join("fullchain.pem"),
                PathBuf::from(&location).join("privkey.pem"),
            )
            .await
            .unwrap();

            log::info!("listening on https://{}", &addr);
            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        Err(_) => {
            log::info!("listening on http://{}", &addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}

#[cfg(feature = "ssr")]
async fn redirect_http_to_https(ip_addr: std::net::IpAddr, http_port: u16, https_port: u16) {
    use axum::{
        extract::Host,
        handler::HandlerWithoutStateExt,
        http::{StatusCode, Uri},
        response::Redirect,
    };
    fn make_https(
        host: String,
        uri: axum::http::Uri,
        http_port: u16,
        https_port: u16,
    ) -> Result<axum::http::Uri, axum::BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&http_port.to_string(), &https_port.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(axum::http::Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, http_port, https_port) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = std::net::SocketAddr::from((ip_addr, http_port));
    log::debug!("listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(redirect.into_make_service())
        .await
        .unwrap();
    // axum::serve(listener, redirect.into_make_service())
    //     .await
    //     .unwrap();
}

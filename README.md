# entropy-network-status-page 

A simple website showing status information about the Entropy Testnet

To build you need cargo-leptos:

```bash
cargo install cargo-leptos
```

Start a development server with:

```bash
cargo leptos watch
```

Compiling for release:

```bash
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo leptos build --release
```

will generate server binary in target/server/release and site package in target/site

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
entropy-network-status-page
site/
```
Set the following environment variables (updating for your project as needed):
```text
ENTROPY_TESTNET_ENDPOINT="ws://something:9944"
LEPTOS_OUTPUT_NAME="entropy-network-status-page"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## HTTPS support

If the environment variable `TLS_CERT_LOCATION` is present, it will search that directory for the TLS certificate and private key in PEM format, which should be named `fullchain.pem` and `privkey.pem` respectively.

The `LEPTOS_SITE_ADDR` should have the port you want to run https: `<ipaddress>:443`.

The server will also open port 80 and redirect traffic there to https.

# entropy-testnet-web-ui 

A simple website showing status information about the Entropy Testnet

This is work in progress. It currently only displays the details of registered Entropy users. 

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
entropy-testnet-web-ui
site/
```
Set the following environment variables (updating for your project as needed):
```text
ENTROPY_TESTNET_ENDPOINT="ws://something:9944"
LEPTOS_OUTPUT_NAME="entropy-testnet-web-ui"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

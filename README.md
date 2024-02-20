# entropy-network-status-page 

A simple website showing status information about an Entropy network.

## Requirements

To build you need [trunk](https://trunkrs.dev):

Install trunk from source with:

```bash
cargo install --locked trunk
```

Or from a release binary with:
```bash
cargo binstall trunk
```

## Setting an Entropy network

At build time, the following environment variables are read: 
- `ENTROPY_NETWORK_ENDPOINT` - Should be the chain endpoint with port and protocol, eg: `"ws://something:9944"` (defaults to `"ws://localhost:9944"`).
- `ENTROPY_NETWORK_NAME` - Should be a name for the network, used in the page title, eg: `"Testnet"` (defaults to `"Local Devnet"`).

## Development

Start a development server with:

```bash
trunk serve
```

## Building for production

```bash
RUSTFLAGS=--cfg=web_sys_unstable_apis trunk build --release
```

This will put the built website into `./dist`. This directory can be copied to a server and served statically.

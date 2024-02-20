//! This pulls the Entropy network name and endpoint from environment variables at build time
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("endpoint.rs");
    let out_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(out_file);

    let endpoint_address =
        env::var_os("ENTROPY_NETWORK_ENDPOINT").unwrap_or("ws://localhost:9944".into());
    writer
        .write(
            format!(
                "pub const ENTROPY_NETWORK_ENDPOINT: &str = \"{}\";\n",
                endpoint_address.to_str().unwrap()
            )
            .as_bytes(),
        )
        .unwrap();

    let network_name = env::var_os("ENTROPY_NETWORK_NAME").unwrap_or("Local Devnet".into());
    writer
        .write(
            format!(
                "pub const ENTROPY_NETWORK_NAME: &str = \"{}\";\n",
                network_name.to_str().unwrap()
            )
            .as_bytes(),
        )
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=ENTROPY_NETWORK_ENDPOINT");
    println!("cargo:rerun-if-env-changed=ENTROPY_NETWORK_NAME");
}

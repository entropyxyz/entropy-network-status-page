#!/bin/bash -

# Automations for generating a static site build of the HTML/CSS/JS
# generated from the Leptos compilation (`cargo leptos build`).
#
# This simply aids in putting the static Web files in their own
# branch for easily publication/deployment.

main () {
    which -s cargo
    if [ 0 -ne $? ]; then
        echo "Please install the Cargo package manager first."
        exit 1
    fi

    cargo install --list | grep -q "cargo-leptos"
    if [ 0 -ne $? ]; then
        cargo install cargo-leptos
    fi
    npm install # Ensure `node_modules` is updated, for Tailwind SCSS.

    CARGO_NET_GIT_FETCH_WITH_CLI=true RUSTFLAGS="--cfg=web_sys_unstable_apis" \
        cargo leptos build "$@"
}

main "$@"

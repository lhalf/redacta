set shell := ["bash", "-euc"]

build:
    cargo build --release

check:
    cargo fmt --check --all
    cargo clippy --all-targets -- -Dwarnings

test:
    cargo test

package: build
    tar -czf target/redacta.tar.gz --directory target/release redacta

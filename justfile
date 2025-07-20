set shell := ["bash", "-euc"]

build:
    cargo build --release

check:
    cargo fmt --check --all
    cargo clippy --all-targets -- -Dwarnings

test: unit-test perf-test

unit-test:
    cargo test --lib

perf-test: build
    cargo test --test "performance" -- --nocapture --color always

package: build
    tar -czf target/redacta.tar.gz --directory target/release redacta

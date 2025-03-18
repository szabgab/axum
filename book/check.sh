#!/usr/bin/bash -ex

cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all
cargo sort --workspace --grouped

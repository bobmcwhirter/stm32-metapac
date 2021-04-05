#!/usr/bin/env bash

set -euxo pipefail

rm -rf src
mkdir src

cargo run --manifest-path ../svd2rust/Cargo.toml -- generate --dir regs
mv lib.rs src/lib.rs

cargo fmt
cargo doc
rm -rf docs
mv target/doc docs
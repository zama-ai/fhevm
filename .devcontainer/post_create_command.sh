#!/bin/bash

set -euo pipefail

# Package manager dependencies.
sudo apt update
sudo apt install -y protobuf-compiler build-essential libssl-dev pkg-config openssl vim

# Cargo dependencies.
cargo install sqlx-cli

# Install the Rust toolchain.
RUST_VERSION=$(cat toolchain.txt)
rustup toolchain install $RUST_VERSION
rustup component add --toolchain $RUST_VERSION rustfmt
rustup component add --toolchain $RUST_VERSION clippy

# Foundry.
curl --proto '=https' --tlsv1.2 -sSfL https://foundry.paradigm.xyz | bash
$HOME/.foundry/bin/foundryup -i v1.3.2

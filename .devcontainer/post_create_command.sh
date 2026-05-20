#!/bin/bash

set -euo pipefail

# Package manager dependencies.
sudo apt update
sudo apt install -y protobuf-compiler build-essential libssl-dev pkg-config openssl vim git-lfs postgresql-client cmake

# Cargo dependencies.
cargo install sqlx-cli

# Install the Rust toolchain.
RUST_VERSION=$(grep 'channel' coprocessor/fhevm-engine/rust-toolchain.toml | awk -F' = ' '{print $2}' | tr -d '"')
rustup toolchain install $RUST_VERSION
rustup component add --toolchain $RUST_VERSION rustfmt
rustup component add --toolchain $RUST_VERSION clippy

# Foundry.
curl --proto '=https' --tlsv1.2 -sSfL https://foundry.paradigm.xyz | bash
$HOME/.foundry/bin/foundryup -i v1.7.1

# AikidoSec safe-chain.
curl --proto '=https' --tlsv1.2 -fsSL https://github.com/AikidoSec/safe-chain/releases/download/1.5.3/install-safe-chain.sh | sh

# Claude Code.
curl --proto '=https' --tlsv1.2 -fsSL https://claude.ai/install.sh | bash

#!/bin/bash

# Package manager dependencies.
sudo apt update
sudo apt install -y protobuf-compiler build-essential libssl-dev pkg-config openssl

# Cargo dependencies.
cargo install sqlx-cli

# Foundry.
curl --proto '=https' --tlsv1.2 -sSfL https://foundry.paradigm.xyz | bash
$HOME/.foundry/bin/foundryup -i v1.3.2

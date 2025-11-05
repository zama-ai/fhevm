#!/bin/bash

# Package manager dependencies.
sudo apt update
sudo apt install -y protobuf-compiler

# Cargo dependencies.
cargo install sqlx-cli

# Foundry.
curl --proto '=https' --tlsv1.2 -sSfL https://foundry.paradigm.xyz | bash
source ~/.bashrc
foundryup -i 1.3.2-stable

# zbc-solidity

A Solidity library for interacting with the Zama Blockchain

# Guide development

Install dependencies (Soldiity libraries and dev tools

```
npm install
```

Note: Solidity files are formatted with prettier.

# Demo test

This repository includes a python script (see [demo_test.py](demo_test.py)) that automates a sequence of steps simulating deployment and interaction with an encrypted ERC20 contract.

In order to test the `reencrypt` logic, a small Rust tool is included in the [decrypt](decrypt) folder. This tool should be compiled before execution of the python script with

```bash
cargo +nightly build --release --features tfhe/aarch64-unix
```

or

```bash
cargo build --release --features tfhe/x86_64-unix
```

depending on your architecture.

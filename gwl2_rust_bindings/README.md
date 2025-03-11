# Rust bindings for Gateway L2 contracts

## Contracts' bindings

This crate exposes modules to interact with the Gateway L2 contracts, using the `sol!` macro of the `alloy_sol_types`
crate.

The ABI of the contracts, stored in the `abi` folder, are used to generate these modules.

## Update check

The `python3 abi_update.py check` command will perform the following operations:

- check that the versions in the `package.json` and `Cargo.toml` are the same
- recompile the ABI of the current Gateway L2 contracts, and check for ABI files that need to be updated in the crate

This command is run automatically as a pre-commit hook, and in the CI of the repository.

If the `python3 abi_update.py check` command raises an error, the `python3 abi_update.py update` command can be used to
fix it.

## What is not automated

If a new contract is written, the script will check that its ABI has been added to the crate's `abi` folder.

However, in order to create the binding for the contract, it is still required to write the following code in
`contract.rs` (or `interfaces.rs` for a new interface):

```rust
sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    NewContractName,
    "abi/NewContractName.abi"
);
```

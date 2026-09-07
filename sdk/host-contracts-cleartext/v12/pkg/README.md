# @fhevm/host-contracts-cleartext

FHEVM host contracts cleartext Solidity contracts. This is generation **v12** of the FHEVM protocol, published as `@fhevm/host-contracts-cleartext@0.12.0`:
the version's minor number is the protocol generation, so pin it with a caret (`^0.12.0`) to stay on this generation.

The package holds a cleartext implementation of the FHEVM host contracts: the same interfaces and addresses as the
production stack, with FHE operations computed on plaintext values, for local development and testing.

| directory    | contents                                                              |
| ------------ | --------------------------------------------------------------------- |
| `src/`       | Solidity sources: host contracts, cleartext executors, proxies        |
| `abi/`       | Generated ABIs                                                        |
| `templates/` | Solidity templates consumed by the Hardhat plugins                    |
| `ts/`        | TypeScript deploy, upgrade and address helpers (`import` / `require`) |
| `forge/`     | Foundry support files                                                 |

```sh
npm install @fhevm/host-contracts-cleartext@^0.12.0
```

Part of the [zama-ai/fhevm](https://github.com/zama-ai/fhevm) monorepo. Licensed under BSD-3-Clause-Clear; see `LICENSE`.

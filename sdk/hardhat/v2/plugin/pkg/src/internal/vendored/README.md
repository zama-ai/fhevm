# vendored

Byte-identical copies of upstream reference code. Do not edit or reformat — a diff against the source
is a bug, so anything this project needs on top belongs in the importing module instead.

| File                   | Source                                                                                               |
| ---------------------- | ---------------------------------------------------------------------------------------------------- |
| `ethersEthereumLib.ts` | `sdk/host-contracts-cleartext/v13/test/ts/utils/ethersEthereumLib.ts`                                |
| `operatorsPrices.ts`   | `sdk/common-vendored/src/operatorsPrices.ts` (itself verbatim from `fhevm/library-solidity/codegen`) |
| `priceTypes.ts`        | `sdk/common-vendored/src/priceTypes.ts`                                                              |

Re-syncing is a plain copy. There is no header to re-add and no import to rewrite: the file already
names its own provenance on line 1, and it imports `@fhevm/host-contracts-cleartext/ts` by package
name, which `pkg/package.json` declares as a dependency.

```sh
cp sdk/host-contracts-cleartext/v13/test/ts/utils/ethersEthereumLib.ts \
   sdk/hardhat/v2/pkg/src/internal/vendored/ethersEthereumLib.ts
```

This directory is excluded from eslint (`eslint.config.js`) and prettier (`.prettierignore`). That is
load-bearing: eslint's `prefer-nullish-coalescing` autofix had silently rewritten one statement here,
which no test or compiler would ever flag.

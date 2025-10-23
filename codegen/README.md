# Config file

`generateHCULimit` (boolean, optional): `true` to generate HCULimit.sol. (default: false)
`shuffle` (boolean, optional): `true` to shuffle the tests. (default: false)
`shuffleWithPseuseRand` (boolean, optional): `true` to shuffle using a deterministic random sequence. (default: `false`)
`publicDecrypt` (boolean, optional): `true` to generate tests with publicly decryptable handles, `false` to generate tests with user decryptable handles. (default: `false`)
`numberOfTestSplits` (positive integer, required): the number of test splits to generate
`noLib` (boolean, optional): `true` to disable any FHEVM library contracts such as `FHE.sol`, `Impl.sol` ... This is required for e2e tests for example (default: `false`)
`directories.baseDir` (string, required): The path the target directory used as the base directory for the generated tests. If the path is relative, the absolute path is resolved relatively to the config file.
`directories.typesDir` (string, optional): The path to the types directory (generated typechain files).
`directories.libDir` (string, optional): The path to the FHEVM library (required if noLib is `true`).
`directories.overloadsDir` (string, optional): If `--overloads` option is not used, this is the path to the directory where the overloads.json file is located. (default: `overloads`)
`directories.contractsDir` (string, optional): The path to the contracts directory (where to save `HCULimit.sol`), (default: `contracts`)

# Generate host-contracts tests

```sh
cd codegen
./codegen.mjs --overloads ./overloads/host-contracts.json --config ./codegen.host-contracts.config.json --debug
```

# Generate library-solidity tests

```sh
cd codegen
./codegen.mjs --overloads ./overloads/library-solidity.json --config ./codegen.library-solidity.config.json --debug
```

# Generate e2e tests

```sh
cd codegen
./codegen.mjs --overloads ./overloads/e2e.json --config ./codegen.e2e.config.json --debug
```

# Dry Run

Use the `--dry-run --debug` options to check code generation in dry run mode.

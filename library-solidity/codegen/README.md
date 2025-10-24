# Install

```sh
npm install
```

# Config file

- `generateHCULimit` (boolean, optional): `true` to generate HCULimit.sol. (default: `false`)
- `shuffle` (boolean, optional): `true` to shuffle the tests. (default: `false`)
- `shuffleWithPseuseRand` (boolean, optional): `true` to shuffle using a deterministic random sequence. (default: `false`)
- `publicDecrypt` (boolean, optional): `true` to generate tests with publicly decryptable handles, `false` to generate tests with user decryptable handles. (default: `false`)
- `numberOfTestSplits` (positive integer, required): the number of test splits to generate. (default: `12`)
- `noLib` (boolean, optional): `true` to disable any FHEVM library contracts such as `FHE.sol`, `Impl.sol` ... This is required for e2e tests for example (default: `false`)
- `directories.baseDir` (string, optional): The path the target directory used as the base directory for the generated tests. If the path is relative, the absolute path is resolved relatively to the config file. (default: `cwd`)
- `directories.fheTypeDir` (string, optional): The path to the directory where `FheType.sol` is located. If not specified, use `directories.libDir`. If the path is relative, the absolute path is resolved relatively to the config file.
- `directories.libDir` (string, optional): The path to the FHEVM lib directory where the `FHE.sol`, `Impl.sol` are located (default: `lib`). If the path is relative, the absolute path is resolved relatively to the config file.
- `directories.overloadsDir` (string, optional): If `--overloads` option is not used, this is the path to the directory where the `overloads.json` file is located. If the path is relative, the absolute path is resolved relatively to the config file. (default: `overloads`)
- `directories.contractsDir` (string, optional): The path to the contracts directory (where to save `HCULimit.sol`). If the path is relative, the absolute path is resolved relatively to the config file. (default: `contracts`)

# Help

```sh
cd codegen
./codegen.mjs overloads --help
./codegen.mjs lib --help
```

# Generate overloads only

```sh
cd codegen
./codegen.mjs overloads /path/to/your/new/overloads.json
```

# Generate host-contracts tests

```sh
cd codegen
./codegen.mjs lib --overloads ./overloads/host-contracts.json --config ./codegen.host-contracts.config.json --verbose
```

or

```sh
cd codegen
npm run codegen:host-contracts
```

# Generate library-solidity tests

```sh
cd codegen
./codegen.mjs lib --overloads ./overloads/library-solidity.json --config ./codegen.library-solidity.config.json --verbose
```

or

```sh
cd codegen
npm run codegen:library-solidity
```

# Generate e2e tests

```sh
cd codegen
./codegen.mjs lib --overloads ./overloads/e2e.json --config ./codegen.e2e.config.json --verbose
```

or

```sh
cd codegen
npm run codegen:e2e
```

# Dry Run

Use the `--dry-run --verbose` options to check code generation in dry run mode.

# Verification scripts

`verify-e2e.sh`, `verify-host-contracts.sh`, `verify-library-solidity.sh` are meant to be deleted. These are temporary test scripts to make sure generated code is identical to the current codegen setup.

# Use codegen in every FHEVM package

```sh
cd library-solidity
npm run codegen
```

```sh
cd host-contracts
npm run codegen
```

```sh
cd tests-suite/e2e
npm run codegen
```

# Install

```sh
npm install
```

# Codegen Config File

`codgen` requires a `codegen.config.json` file as argument. Here is an example of codegen config file used to generate e2e tests:

```json
{
  "__comment": "All relative paths are resolved against directories.baseDir",
  "baseDir": ".",
  "noLib": true,
  "noHostContracts": true,
  "tests": {
    "numberOfTestSplits": 96,
    "overloads": "../../library-solidity/codegen/overloads/e2e.json",
    "publicDecrypt": true,
    "shuffle": true,
    "shuffleWithPseuseRand": true,
    "solidity": {
      "__comment": "All imports paths are relative to outDir",
      "outDir": "./contracts/operations",
      "imports": ["@fhevm/solidity/lib/FHE.sol", ["E2ECoprocessorConfig", "../E2ECoprocessorConfigLocal.sol"]],
      "parentContractName": "E2ECoprocessorConfig"
    },
    "typescript": {
      "__comment": "All imports paths are relative to outDir",
      "outDir": "./test/fhevmOperations",
      "imports": [
        ["instance", "../instance"],
        ["signers", "../signers"],
        ["typechain", "../../types"]
      ]
    }
  }
}
```

# Codegen Config File Documentation

- `baseDir` (string, optional): The path the target directory used as the base directory. If the path is relative, the absolute path is resolved relatively to the config file. (default: config file directory)
- `noLib` (boolean, optional): `true` to disable FHEVM library contracts generation (`FHE.sol`, `Impl.sol`, `FheType.sol`). (default: `false`)
- `noTest` (boolean, optional): `true` to disable Tests generation (default: `false`)
- `noHostContracts` (boolean, optional): `true` to disable host contracts generation (`HCULimit.sol`) (default: `false`)
- `tests.shuffle` (boolean, optional): `true` to shuffle the tests. (default: `false`)
- `tests.shuffleWithPseuseRand` (boolean, optional): `true` to shuffle using a deterministic random sequence. (default: `false`)
- `tests.publicDecrypt` (boolean, optional): `true` to generate tests with publicly decryptable handles, `false` to generate tests with user decryptable handles. (default: `false`)
- `tests.numberOfTestSplits` (positive integer, required): the number of test splits to generate. (default: `12`)
- `tests.overloads` (string, optional): If `--overloads` option is not used, this is the path to the directory where the `overloads.json` file is located. If the path is relative, the absolute path is resolved relatively to the `baseDir`. (default: `./overloads.json`)
- `tests.solidity` contains all the parameters related to Solidity test code generation.
- `tests.typescript` contains all the parameters related to Typescript test code generation.
- `lib.fheTypeDir` (string, optional): The path to the directory where `FheType.sol` is located. If not specified, use `lib.outDir`. If the path is relative, the absolute path is resolved relatively to `baseDir`. (default: `./lib`)
- `lib.outDir` (string, optional): The path to the FHEVM lib directory where the `FHE.sol`, `Impl.sol` are generated (default: `lib`). If the path is relative, the absolute path is resolved relatively to `baseDir`. (default: `./lib`)
- `hostContracts.outDir` (string, optional): The path to the contracts directory (where to save `HCULimit.sol`). If the path is relative, the absolute path is resolved relatively to the `baseDir`. (default: `./contracts`)

# Overloads generation

Random overloads are generated using the `overloads` CLI command. These resulting JSON files are located in a centralized folder: `<root>/library-solidity/codegen/overloads/.`. Three distinct files are available: `library-solidity.json`, `host-contracts.json`, and `e2e.json`. While these files share an identical structure, they contain different sets of random values for each package's specific testing and code generation needs.

Each package's configuration file points directly to its corresponding overload file for resolution.

# Modifying lib Solidity Files (Using Templates)

### Using Template or Typescript code

If you need to change the behavior of FHEVM library files — `FHE.sol`, `Impl.sol`, or `FheType.sol` — you have two options:

- If your modification does **not** involve dynamically generated code, simply edit the associated template file located in `<root>/library-solidity/codegen/templates/.` (e.g., `FHE.sol-template`, `Impl.sol-template` or `FheType.sol-template`).
- If your modification **does** involve dynamically generated code, you can change the generated code by editing the corresponding ts file `templateFHEDotSol.ts`, `templateImplDotSol.ts`, `templateFheTypeDotSol.ts`.

### Template Comments

To add comments that are removed during the final code generation step (clean-up comments), prefix the comment line with the special marker `//$$`:

```js
//$$ This comment will not appear in the final generated .sol file.
```

⚠️ Warning: It seems that there is a bug when a comment is empty.

### Template Synthax Highlighting

To enable proper Solidity syntax coloring for the template files (which makes editing much easier), add the following entry to your VSCode settings.json (Workspace or User settings):

```json
"files.associations": {
    "*.sol-template": "solidity"
}
```

# Help

```sh
cd codegen
./codegen.mjs overloads --help
./codegen.mjs lib --help
```

# Generate overloads only

Use the `overloads` command. It is not required to re-generate overloads each time the tests or the libs are modified.

```sh
cd codegen
./codegen.mjs overloads /path/to/your/new/overloads.json
```

```sh
# or by using the path of overloads.json specified in the config
cd codegen
./codegen.mjs overloads --config ./codegen.config.json
```

# Generate lib with tests

Use the `lib` command.

```sh
cd codegen
./codegen.mjs lib --config /path/to/codegen.config.json --verbose
```

or

```sh
# if you want to use specific overloads
cd codegen
./codegen.mjs lib --overloads /path/to/my/overloads.json --config /path/to/codegen.config.json --verbose
```

# Dry Run

Use the `--dry-run` options to check code generation in dry run mode.

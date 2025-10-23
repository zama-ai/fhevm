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

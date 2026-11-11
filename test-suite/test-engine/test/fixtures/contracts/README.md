# Fixture contracts

Contracts used by the Chain module tests. They have no fhEVM dependency, and their compiled
artifacts are committed so that no compiler runs in the tests.

| Contract      | Artifact       | Compiler                                     |
| ------------- | -------------- | -------------------------------------------- |
| `Counter.sol` | `Counter.json` | solc 0.8.24, optimizer on (200 runs), cancun |

`Counter.json` uses the Hardhat artifact format (`hh-sol-artifact-1`). To rebuild it after changing
the source, compile with the Foundry image of `test/integration/chain/compose.yaml`:

```sh
mkdir -p /tmp/counter/src && cp test/fixtures/contracts/Counter.sol /tmp/counter/src/
docker run --rm -v /tmp/counter:/work -w /work --entrypoint forge <foundry image> \
  build --use 0.8.24 --optimize --optimizer-runs 200
```

then copy `abi`, `bytecode.object` and `deployedBytecode.object` from
`/tmp/counter/out/Counter.sol/Counter.json` into `Counter.json`.

`../compiled-contracts/contracts/Counter.sol/Counter.json` is a copy laid out as Hardhat writes
it, for the `--contracts-dir` tests (`this.engine.contracts`); keep both in sync.

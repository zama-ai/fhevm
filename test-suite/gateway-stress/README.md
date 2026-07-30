# Gateway Stress-Test Tool

## Introduction

A simple tool to send a configurable number of parallel decryption requests (public, user, or
RFC-016 user-v2 decrypts at the time of writing), at a given frequency and for a specified
duration.

## Table of Contents
- [Introduction](#introduction)
- [Build](#build)
- [Configuration](#configuration)
- [Run](#run)
  - [Stress testing](#stress-testing)
  - [Benchmarking](#benchmarking)
- [Tracing](#tracing)
- [Local e2e setup](#local-e2e-setup)
  - [Generating handles with `gen_handles.ts`](#generating-handles-with-gen_handlests)

## Build

You can build the tool by running the `cargo build --release` command in the
`test-suite/gateway-stress` directory.

Alternatively, you can run the manual `gateway-stress-tool-docker-build` workflow to trigger the
build of the Docker images for the tool.

## Configuration

To configure the tool, you can must use a configuration file (TOML format).

Every configuration option is documented in the example [configuration file](config/config.toml).

Some of the configuration fields can be overridden via the CLI:
- `tests_duration`
- `tests_interval`
- `parallel_requests`
- `sequential`
- `id_counter_start` (via `--id-counter-start`, DB path only)

## Run

### Stress testing

Once the `gateway-stress` binary has been built, you can run the following commands:

```bash
# Display CLI help
./gateway-stress help

# Run public decryption stress test using the Gateway chain
./gateway-stress -c config/config.toml gw -t public

# Run user decryption stress test using the Gateway chain
./gateway-stress -c config/config.toml gw -t user

# Run RFC-016 user-v2 decryption stress test using the Gateway chain
./gateway-stress -c config/config.toml gw -t user-v2

# Override number of parallel requests during the stress tests session
./gateway-stress -c config/config.toml -p 10 gw public

# Run public decryption stress test by inserting requests in Connectors' DBs
./gateway-stress -c config/config.toml db -t public

# Run RFC-016 user-v2 decryption stress test by inserting requests in Connectors' DBs
./gateway-stress -c config/config.toml db -t user-v2

# NOTE: legacy `user` decryption is NOT supported over the `db`/`bench-db` path. The KMS connector
# needs the request's on-chain tx_hash to run the ACL check, which direct DB inserts can't provide.
# Use `-t user-v2` (RFC-016, verified straight from the payload) or the `gw` path for legacy `user`.

# Don't clear Connectors' DBs before and after running stress tests (not recommended)
./gateway-stress -c config/config.toml db -t public --skip_clear-db
```

Or directly from `test-suite/gateway-stress` directory:

```bash
cargo run -- -c config/config.toml gw -t public
cargo run -- -c config/config.toml gw -t user
```

### Benchmarking

The `benchmark` command take a CSV file in input (and the global config file as well).
Each line of this CSV represent a burst of decryption to benchmark, which is composed of:
- The number of parallel requests in the burst (1st column)
- The number of time we must measure this burst (2nd column)
- The type of decryption in the burst (`public`, `user`, or `user_v2`)

See the [templates](./templates) folder for examples.

It will then run the benchmark and stores the results (average and standard deviation of latency
and throughput) for each burst in a CSV file.

```bash
# Run a benchmarking session using `templates/small_bench.csv` as input and store the global
# results in `/tmp/bench.csv` (decryption requests sent using the Gateway chain)
./gateway-stress -c config/config.toml bench-gw -i templates/small_bench.csv -o /tmp/bench.csv

# Same, but also store each burst result in `tmp/full.csv`
./gateway-stress -c config/config.toml bench-gw -i templates/small_bench.csv -o /tmp/bench.csv -r /tmp/full.csv

# Run a benchmarking session using `templates/small_bench.csv` as input and store the global
# results in `/tmp/bench.csv` (decryption requests inserted in Connectors' DB)
./gateway-stress -c config/config.toml bench-db -i templates/small_bench.csv -o /tmp/bench.csv

# Same, but also store each burst result in `tmp/full.csv`
./gateway-stress -c config/config.toml bench-db -i templates/small_bench.csv -o /tmp/bench.csv -r /tmp/full.csv
```

## Tracing

This tool aims to output only essential information regarding the status of the test. The main
observation of the test should be done in Grafana or within our infrastructure, not via this tool.

However, this tool uses the `tracing` crate, and if you are facing issues during a stress test
session, you can get more logs by configuring the `RUST_LOG` environment variable. Example:

```bash
# Enabling "DEBUG" traces of the stress test tool alone
RUST_LOG="gateway_stress=debug" ./gateway-stress -c config/config.toml gw -t public

# Enabling "DEBUG" traces of the stress test tool and of the alloy crate
RUST_LOG="gateway_stress=debug,alloy=debug" ./gateway-stress -c config/config.toml gw -t public

# Enabling "DEBUG" traces for all crates used by the stress test tool
RUST_LOG="debug" ./gateway-stress -c config/config.toml gw -t public
```

## Local e2e setup

To play with the tool in a local e2e setup, follow these steps:

- from the root of the `fhevm` repo: `cd test-suite/fhevm`
- deploy the e2e setup using the `./fhevm-cli deploy` command
- generate ciphertext handles with the [`gen_handles.ts`](../e2e/scripts/gen_handles.ts) script (see below)
- update `allowed_contract` and the `[[public_ct]]` / `[[user_ct]]` sections of the
  [gateway-stress config](../gateway-stress/config/config.toml) with the values the script prints
- run the tool. Ex:
  - `cd ../gateway-stress`
  - `cargo run -- -c config/config.toml -p 1 -d "1s" gw -t user`

### Generating handles with `gen_handles.ts`

`test-suite/e2e/scripts/gen_handles.ts` produces handles through the **real on-chain input flow**, so each handle is committed and ACL-authorized by construction.

The script runs **inside the test-suite e2e container**. It only needs a private key:

```bash
# Use the same `private_key` as in config/config.toml, 0x-prefixed
docker exec \
  -e PRIVATE_KEY=0xe746bc71f6bee141a954e6a49bc9384d334e393a7ea1e70b50241cb2e78e9e4c \
  fhevm-test-suite-e2e-debug \
  bash -c 'npx hardhat run scripts/gen_handles.ts --network staging'
```

The key matters: the script calls the host contract as that signer, so `msg.sender` (and thus the
ACL-authorized address) matches the `userAddress` gateway-stress signs with at decrypt time. On a
local stack the script also, for that same key:

- tops up host- and gateway-chain gas via `anvil_setBalance` when the balance is 0
- mints $ZAMA and approves `ProtocolPayment` for an unbounded allowance, so on-chain decryption
  requests can pay the per-request fee instead of reverting with `ERC20InsufficientAllowance`

It ends by printing the values to copy into the config:

```
=== gateway-stress config values ===
allowed_contract = "0x..."

[[public_ct]]
handle = "0x..."

[[user_ct]]
handle = "0x..."
```

Notes:

- Set `GEN_HANDLES_CONTRACT_ADDRESS=0x...` to reuse an already-deployed `SmokeTestInput` instead of deploying a new one.

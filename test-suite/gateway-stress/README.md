# Gateway Stress-Test Tool

## Introduction

A simple tool to send a configurable number of parallel decryption requests (public or user
decrypts at the time of writing), at a given frequency and for a specified duration.

## Table of Contents
- [Introduction](#introduction)
- [Build](#build)
- [Configuration](#configuration)
- [Run](#run)
  - [Stress testing](#stress-testing)
  - [Benchmarking](#benchmarking)
- [Tracing](#tracing)
- [Bonus: Generating handles via coprocessor stress-test-generator](#bonus-generating-handles-via-coprocessor-stress-test-generator)

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

# Override number of parallel requests during the stress tests session
./gateway-stress -c config/config.toml -p 10 gw public

# Run public decryption stress test by inserting requests in Connectors' DBs
./gateway-stress -c config/config.toml db -t public

# Run user decryption stress test by inserting requests in Connectors' DBs
./gateway-stress -c config/config.toml db -t user

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
- The type of decryption in the burst (`public` or `user`)

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

## Bonus: Generating handles via coprocessor stress-test-generator

To use this tool, you would need already existing handles to decrypt. You could use coprocessor's
`stress-test-generator` tool to generate these handles.

The tool is located at `coprocessor/fhevm-engine/stress-test-generator` in the `fhevm` repo.
Then, look at the `README.md` and gather all the environment variable values needed (default
values work only for e2e setup).

```bash
export EVGEN_DB_URL="TODO"
export ACL_CONTRACT_ADDRESS="TODO"
# ...
EVGEN_SCENARIO=data/minitest_003_generate_handles_for_decryption.csv make run
```

This will generate the `data/handles_for_pub_decryption` and `handles_for_usr_decryption` files.

Make sure that the 6th column of the `EVGEN_SCENARIO` file match the `allowed_contract` value of
this tool's configuration, and that the 7th column match the wallet address used by the tool.

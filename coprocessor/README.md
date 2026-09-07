## Introduction
**FHEVM Coprocessor** provides the execution service for FHE computations.

It includes a **Coprocessor** service [FHEVM-coprocessor](docs/getting_started/fhevm/coprocessor/coprocessor_backend.md). The Coprocessor
itself consists of multiple microservices, e.g. for FHE compute, input verify, transaction sending, listening to events, etc.

## Main features

- An **Executor** service for [FHEVM-native](docs/getting_started/fhevm/native/executor.md)
- A **Coprocessor** service for [FHEVM-coprocessor](docs/getting_started/fhevm/coprocessor/coprocessor_backend.md)

_Learn more about FHEVM Coprocessor features in the [documentation](docs)._
<br></br>

## Table of Contents

- [Introduction](#introduction)
- [Main Features](#main-features)
- [Getting Started](#getting-started)
  - [Generating Keys](#generating-keys)
  - [Coprocessor](#coprocessor)
    - [Dependencies](#dependences)
    - [Installation](#installation)
    - [Services Configuration](#services-configuration)
      - [tfhe-worker](#tfhe-worker)
      - [cli](#cli)
      - [host-listener](#host-listener)
      - [gw-listener](#gw-listener)
      - [sns-worker](#sns-worker)
      - [zkproof-worker](#zkproof-worker)
      - [transaction-sender](#transaction-sender)
- [Resources](#resources)
  - [Documentation](#documentation)
  - [FHEVM Demo](#fhevm-demo)
- [Support](#support)

## Getting started

### Generating keys

For testing purposes a set of keys can be generated as follows:

```
$ cd fhevm-engine/fhevm-engine-common
$ cargo run generate-keys
```

The keys are stored by default in `fhevm-engine/fhevm-keys`.

### Coprocessor

#### Dependences

- `docker-compose`
- `rust`
- `sqlx-cli` (install with `cargo install sqlx-cli`)
- `anvil` (for testing, installation manual https://book.getfoundry.sh/getting-started/installation)

#### Installation

```
$ cd fhevm-engine/coprocessor
$ cargo install --path .
```

#### Services Configuration

##### tfhe-worker

```bash
$ tfhe_worker --help
Usage: tfhe_worker [OPTIONS]

Options:
      --run-bg-worker
          Run the background worker
      --generate-fhe-keys
          Generate fhe keys and exit
      --work-items-batch-size <WORK_ITEMS_BATCH_SIZE>
          Work items batch size [default: 10]
      --tenant-key-cache-size <TENANT_KEY_CACHE_SIZE>
          Tenant key cache size [default: 32]
      --coprocessor-fhe-threads <COPROCESSOR_FHE_THREADS>
          Coprocessor FHE processing threads [default: 8]
      --tokio-threads <TOKIO_THREADS>
          Tokio Async IO threads [default: 4]
      --pg-pool-max-connections <PG_POOL_MAX_CONNECTIONS>
          Postgres pool max connections [default: 10]
      --metrics-addr <METRICS_ADDR>
          Prometheus metrics server address [default: 0.0.0.0:9100]
      --database-url <DATABASE_URL>
          Postgres database url. If unspecified DATABASE_URL environment variable is used
```

```bash
$ cli --help
Usage: cli <COMMAND>

Commands:
  insert-tenant  Inserts tenant into specified database
  smoke-test     Coprocessor smoke test
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

For more details on configuration, please check [Coprocessor Configuration](docs/getting_started/fhevm/coprocessor/configuration.md)

For AWS RDS/PostgreSQL IAM authentication, keep `DATABASE_URL` passwordless, for example
`postgresql://coprocessor@my-db.cluster-xyz.eu-west-2.rds.amazonaws.com:5432/coprocessor`,
and set `DATABASE_IAM_AUTH_ENABLED=true`. `DATABASE_IAM_REGION` and
`DATABASE_SSL_ROOT_CERT_PATH` should also be set so the runtime can sign tokens for the correct
region and enforce `verify-full` TLS with the expected CA bundle.

##### host-listener

```bash
$ host_listener --help
Usage: host_listener [OPTIONS] --acl-contract-address <ACL_CONTRACT_ADDRESS> --tfhe-contract-address <TFHE_CONTRACT_ADDRESS>

Options:
      --url <URL>
          [default: ws://0.0.0.0:8545]
      --acl-contract-address <ACL_CONTRACT_ADDRESS>
      --tfhe-contract-address <TFHE_CONTRACT_ADDRESS>
      --kms-generation-address <KMS_GENERATION_ADDRESS>
          [default: ]
      --database-url <DATABASE_URL>
          [default: postgresql://postgres:postgres@localhost:5432/coprocessor]
      --start-at-block <START_AT_BLOCK>
          Can be negative from last block
      --end-at-block <END_AT_BLOCK>
          End catchup at this block (can be negative from last block)
      --catchup-margin <CATCHUP_MARGIN>
          Catchup margin relative the last seen block [default: 5]
      --catchup-paging <CATCHUP_PAGING>
          Catchup paging size in number of blocks [default: 100]
      --initial-block-time <INITIAL_BLOCK_TIME>
          Initial block time, refined on each block [default: 12]
      --log-level <LOG_LEVEL>
          [default: INFO]
      --health-port <HEALTH_PORT>
          Health check port [default: 8080]
      --dependence-cache-size <DEPENDENCE_CACHE_SIZE>
          Pre-computation dependence chain cache size [default: 10000]
      --dependence-by-connexity
          Dependence chain are connected components
      --dependence-cross-block
          Dependence chain are across blocks
      --dependent-ops-max-per-chain <DEPENDENT_OPS_MAX_PER_CHAIN>
          Max dependent ops per chain before slow-lane (0 disables; startup promotes all chains to fast) [default: 0]
      --reorg-maximum-duration-in-blocks <REORG_MAXIMUM_DURATION_IN_BLOCKS>
          Maximum duration in blocks to detect reorgs [default: 50]
      --service-name <SERVICE_NAME>
          service name in OTLP traces [env: OTEL_SERVICE_NAME=] [default: host-listener]
      --catchup-finalization-in-blocks <CATCHUP_FINALIZATION_IN_BLOCKS>
          Maximum number of blocks to wait before a block is finalized [default: 20]
      --only-catchup-loop
          Run only catchup loop without real-time subscription
      --catchup-loop-sleep-secs <CATCHUP_LOOP_SLEEP_SECS>
          Sleep duration in seconds between catchup loop iterations [default: 60]
      --timeout-request-websocket <TIMEOUT_REQUEST_WEBSOCKET>
          Timeout in seconds for RPC calls over websocket [default: 15]
  -h, --help
          Print help
  -V, --version
          Print version
```

`host_listener_consumer` can take its broker address from `--url`,
`--broker-url`, or the `BROKER_URL` environment variable.

##### gw-listener

```bash
$ gw_listener --help
Usage: gw_listener [OPTIONS] --gw-url <GW_URL> --input-verification-address <INPUT_VERIFICATION_ADDRESS>

Options:
      --database-url <DATABASE_URL>
          
      --database-pool-size <DATABASE_POOL_SIZE>
          [default: 16]
      --verify-proof-req-database-channel <VERIFY_PROOF_REQ_DATABASE_CHANNEL>
          [default: event_zkpok_new_work]
      --gw-url <GW_URL>
          
  -i, --input-verification-address <INPUT_VERIFICATION_ADDRESS>
          
      --error-sleep-initial-secs <ERROR_SLEEP_INITIAL_SECS>
          [default: 1]
      --error-sleep-max-secs <ERROR_SLEEP_MAX_SECS>
          [default: 10]
      --health-check-port <HEALTH_CHECK_PORT>
          [default: 8080]
      --metrics-addr <METRICS_ADDR>
          Prometheus metrics server address [default: 0.0.0.0:9100]
      --health-check-timeout <HEALTH_CHECK_TIMEOUT>
          [default: 4s]
      --provider-max-retries <PROVIDER_MAX_RETRIES>
          [default: 4294967295]
      --provider-retry-interval <PROVIDER_RETRY_INTERVAL>
          [default: 4s]
      --log-level <LOG_LEVEL>
          [default: INFO]
      --get-logs-poll-interval <GET_LOGS_POLL_INTERVAL>
          [default: 1s]
      --get-logs-block-batch-size <GET_LOGS_BLOCK_BATCH_SIZE>
          [default: 100]
      --service-name <SERVICE_NAME>
          gw-listener service name in OTLP traces [default: gw-listener]
  -h, --help
          Print help
  -V, --version
          Print version
```

##### transaction-sender

```bash
$ transaction_sender --help
Usage: transaction_sender [OPTIONS] --input-verification-address <INPUT_VERIFICATION_ADDRESS> --ciphertext-commits-address <CIPHERTEXT_COMMITS_ADDRESS> --gateway-url <GATEWAY_URL>

Options:
  -i, --input-verification-address <INPUT_VERIFICATION_ADDRESS>
          
  -c, --ciphertext-commits-address <CIPHERTEXT_COMMITS_ADDRESS>
          
  -g, --gateway-url <GATEWAY_URL>
          
  -s, --signer-type <SIGNER_TYPE>
          [default: private-key] [possible values: private-key, aws-kms]
  -p, --private-key <PRIVATE_KEY>
          
  -d, --database-url <DATABASE_URL>
          
      --database-pool-size <DATABASE_POOL_SIZE>
          [default: 10]
      --database-polling-interval-secs <DATABASE_POLLING_INTERVAL_SECS>
          [default: 1]
      --verify-proof-resp-database-channel <VERIFY_PROOF_RESP_DATABASE_CHANNEL>
          [default: event_zkpok_computed]
      --add-ciphertexts-database-channel <ADD_CIPHERTEXTS_DATABASE_CHANNEL>
          [default: event_ciphertexts_uploaded]
      --verify-proof-resp-batch-limit <VERIFY_PROOF_RESP_BATCH_LIMIT>
          [default: 128]
      --verify-proof-resp-max-retries <VERIFY_PROOF_RESP_MAX_RETRIES>
          [default: 6]
      --verify-proof-remove-after-max-retries
          
      --add-ciphertexts-batch-limit <ADD_CIPHERTEXTS_BATCH_LIMIT>
          [default: 10]
      --add-ciphertexts-max-retries <ADD_CIPHERTEXTS_MAX_RETRIES>
          [default: 2147483647]
      --error-sleep-initial-secs <ERROR_SLEEP_INITIAL_SECS>
          [default: 1]
      --error-sleep-max-secs <ERROR_SLEEP_MAX_SECS>
          [default: 300]
      --txn-receipt-timeout-secs <TXN_RECEIPT_TIMEOUT_SECS>
          [default: 10]
      --required-txn-confirmations <REQUIRED_TXN_CONFIRMATIONS>
          [default: 0]
      --review-after-unlimited-retries <REVIEW_AFTER_UNLIMITED_RETRIES>
          [default: 30]
      --provider-max-retries <PROVIDER_MAX_RETRIES>
          [default: 4294967295]
      --provider-retry-interval <PROVIDER_RETRY_INTERVAL>
          [default: 4s]
      --health-check-port <HEALTH_CHECK_PORT>
          [default: 8080]
      --metrics-addr <METRICS_ADDR>
          Prometheus metrics server address [default: 0.0.0.0:9100]
      --health-check-timeout <HEALTH_CHECK_TIMEOUT>
          [default: 4s]
      --log-level <LOG_LEVEL>
          [default: INFO]
      --gas-limit-overprovision-percent <GAS_LIMIT_OVERPROVISION_PERCENT>
          [default: 300]
      --graceful-shutdown-timeout <GRACEFUL_SHUTDOWN_TIMEOUT>
          [default: 8s]
      --service-name <SERVICE_NAME>
          service name in OTLP traces [default: txn-sender]
      --metric-host-txn-latency <METRIC_HOST_TXN_LATENCY>
          Prometheus metrics: coprocessor_host_txn_latency_seconds [default: 0.1:60.0:0.1]
      --metric-zkproof-txn-latency <METRIC_ZKPROOF_TXN_LATENCY>
          Prometheus metrics: coprocessor_zkproof_txn_latency_seconds [default: 0.1:60.0:0.1]
  -h, --help
          Print help
  -V, --version
          Print version
```

When using the `private-key` signer type, the `-p, --private-key <PRIVATE_KEY>` option becomes mandatory.

When using the `aws-kms` signer type, standard `AWS_*` environment variables are supported, e.g.:
 - **AWS_REGION**
 - **AWS_ACCESS_KEY_ID** (i.e. username)
 - **AWS_SECRET_ACCESS_KEY** (i.e. password)
 - etc.

## GPU-enabled images

The three services that do FHE work — `tfhe-worker`, `sns-worker`, `zkproof-worker` — can also be published as GPU images, built by `.github/workflows/coprocessor-gpu-docker-build.yml`. The other services have no GPU path and stay CPU-only.

**A GPU image targets one compute capability, and its tag says which.** Tags look like:

```
ghcr.io/zama-ai/fhevm/coprocessor/sns-worker:v0.15.0-cuda12.2-sm90
                                             ^version ^toolkit  ^compute capability
```

That is not decoration. `tfhe-cuda-backend` picks its CUDA architectures from the device present when it compiles:

| condition | architectures |
| --- | --- |
| `MULTI_ARCH` cargo feature | 75, 80, 86, 89 — no 90, so no H100 |
| a device is visible | `native`, i.e. that device only |
| no device | 70, behind a CMake warning |

`docker build` exposes no GPU (buildx has no `--gpus`), so building these images the way the CPU images are built would silently produce `sm_70` binaries for a fleet of H100s. The workflow therefore compiles on a GPU runner, where CMake sees the device and builds `native`, then packages the result with `coprocessor/fhevm-engine/Dockerfile.gpu`. The capability is read off the device with `nvidia-smi` rather than inferred from the runner profile, so the tag cannot claim something the binary does not have.

**Consequence for deployment:** an `sm_90` image runs on H100 and not on L40, and vice versa. Pick the tag that matches the hardware; there is deliberately no floating `:gpu` tag, because "some GPU" is exactly the ambiguity that would put an unrunnable image into a cluster.

### The runner is not a free choice

Instances are chosen with the same `provider::profile (hardware)` string the GPU benchmark job uses, parsed by `ci/parse_benchmark_profile.py`, which maps it onto a slab backend (`terraform` for Scaleway, `hyperstack` for Hyperstack) and refuses a profile that is not in `ci/slab.toml` before any instance is requested.

The default is **`scaleway::single-h100 (H100-1-80G)`**: Scaleway H100s are far more available than Hyperstack's, and production is H100. Only single-GPU profiles are offered — a compile gains nothing from eight GPUs and would hold a scarce multi-GPU instance for the length of a tfhe-rs CUDA build.

`hyperstack::l40` exists for L40 deployments and is **not** a substitute for an H100 build — the difference is functional, not just tuning:

- CMake bakes the detected capability into a `CUDA_ARCH` compile definition, and `tfhe-cuda-backend` gates real code on it. `#if CUDA_ARCH >= 900` appears throughout the programmable bootstrap — the core FHE operation — including the thread-block-cluster (`tbc`) paths that only Hopper provides. Build on an L40 and `CUDA_ARCH=890`, so those paths are *compiled out*.
- The cubins are architecture-specific anyway, so an L40-built image would not load on an H100 even if the code were identical.

So an L40 build is a valid artifact for L40 hardware and a wrong one for anything else. The run summary says so explicitly whenever the capability is not 90.

**Manual trigger only.** There is no tag or push trigger: a GPU build occupies a scarce GPU runner for the length of a tfhe-rs CUDA compile and produces an artifact valid for exactly one device class, which is not something to fire automatically.

### What is in the image

The runtime base is the same Chainguard pin the CPU images use, `cgr.dev/zama.ai/glibc-dynamic`, so a GPU worker is no more privileged and no larger a supply-chain surface than its CPU sibling. That base is distroless — no package manager, and no shell either — so the one library it lacks, `libcudart.so.12`, is **copied in** from the toolkit that compiled the binary. `libstdc++` and `libgcc_s` are not copied: the base already provides them, and adding an older copy alongside risks shadowing the newer one.

Glibc works out in both directions, which is worth knowing because the two providers differ: the Scaleway image is Ubuntu 24.04 (glibc 2.39) and Hyperstack's is 22.04 (2.35), while the production base ships **glibc 2.43** — read out of a published CPU image. Either build therefore runs on it, and the exec check below is what would catch it if that ever stopped being true.

Measured on a real build: **107 MB**, against 437 MB for an Ubuntu CUDA `-base` plus apt, and 3.65 GB for the CUDA `-runtime` flavor, which bundles cuBLAS, cuFFT and cuSPARSE that a worker never links.

The images set **no `CMD`**. One Dockerfile serves three services, and a Dockerfile cannot interpolate a build argument into an exec-form `CMD`; the alternatives were a shell the distroless base does not have, or a second copy of a 55 MB binary. Nothing is lost, because the stack already names the binary explicitly for the CPU images too — `command: [sns_worker, --database-url=…]` in compose — and the binary keeps its own name on `PATH`.

### Guards before anything is pushed

- **The binaries must link `libcudart`.** A CPU build links no CUDA runtime at all, so this distinguishes a real GPU build from one where `--features gpu` was dropped somewhere in the chain.
- **Each image must actually `exec`**, checked with `--version`, which needs no database and no device. This is what catches a glibc or missing-library mismatch between the build host and the runtime base — verified in both directions during development: exit 1 against a base whose glibc was too old, exit 0 against the Chainguard base.

### Running it

Dispatch only, with `push` defaulting to off so a run can be inspected before anything reaches the registry.

For testing without cgr.dev credentials, the `runtime-base` input offers Chainguard's public `cgr.dev/chainguard/glibc-dynamic:latest`, which is the same image family as the production pin; the cgr.dev login step is skipped for it. No image from any other registry enters the pipeline: `libcudart` is copied out of the toolkit that compiled the binary, so the shipped runtime and the compiler are the same version by construction, and the CUDA release in the tag is read from `nvcc` rather than taken from an input that could disagree with it.

## Telemetry Style Guide (Tracing + OTEL)

Use `tracing` spans as the default telemetry API.

### Rules

1. Use function/span names as the operation name.
   - Do not add an `operation = "..."` span field.
2. Do not attach high-cardinality identifiers to span attributes.
   - Do not put `txn_id`, `transaction_hash`, or `handle` on spans.
   - If needed for debugging, log these values in events/log lines.
3. For async work, instrument futures with `.instrument(...)`.
   - Do not keep `span.enter()` guards alive across `.await`.
4. Set OTEL error status on error exits.
   - Logging an error is not enough for trace error visibility.
5. Keep span fields low-cardinality and useful for aggregation.
   - Good examples: `request_id`, counts, booleans, retry bucket, chain id.

### Preferred snippets

```rust
#[tracing::instrument(skip_all)]
async fn process_proof(...) -> anyhow::Result<()> {
    // business logic
    Ok(())
}
```

```rust
use tracing::Instrument;

let db_insert_span = tracing::info_span!("db_insert", request_id);
async {
    sqlx::query("UPDATE ...").execute(pool).await?;
    Ok::<(), sqlx::Error>(())
}
.instrument(db_insert_span.clone())
.await?;
```

```rust
use tracing_opentelemetry::OpenTelemetrySpanExt;

if let Err(err) = do_work().instrument(span.clone()).await {
    span.context().span().set_status(opentelemetry::trace::Status::error(err.to_string()));
    return Err(err.into());
}
```


## Resources

### Documentation

Full, comprehensive documentation is available here: [https://docs.zama.ai/fhevm](https://docs.zama.ai/fhevm).

### FHEVM Demo

A complete demo showcasing an integrated FHEVM blockchain and KMS (Key Management System) is available here: [https://github.com/zama-ai/fhevm-test-suite/](https://github.com/zama-ai/fhevm-test-suite/).


## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../docs/.gitbook/assets/support-banner-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="../docs/.gitbook/assets/support-banner-light.png">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.

[![GitHub stars](https://img.shields.io/github/stars/zama-ai/fhevm?style=social)](https://github.com/zama-ai/fhevm/)

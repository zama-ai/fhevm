## Introduction
**FHEVM Coprocessor** provides the execution service for FHE computations.

It includes a **Coprocessor** service [FHEVM-coprocessor](docs/getting_started/fhevm/coprocessor/coprocessor_backend.md). The Coprocessor
itself consists of multiple microservices, e.g. for FHE compute, input verify, transaction sending, listenting to events, etc.

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
      --run-server
          Run the API server
      --run-bg-worker
          Run the background worker
      --generate-fhe-keys
          Generate fhe keys and exit
      --server-maximum-ciphertexts-to-schedule <SERVER_MAXIMUM_CIPHERTEXTS_TO_SCHEDULE>
          Server maximum ciphertexts to schedule per batch [default: 5000]
      --server-maximum-ciphertexts-to-get <SERVER_MAXIMUM_CIPHERTEXTS_TO_GET>
          Server maximum ciphertexts to serve on get_cihpertexts endpoint [default: 5000]
      --work-items-batch-size <WORK_ITEMS_BATCH_SIZE>
          Work items batch size [default: 10]
      --tenant-key-cache-size <TENANT_KEY_CACHE_SIZE>
          Tenant key cache size [default: 32]
      --maximimum-compact-inputs-upload <MAXIMIMUM_COMPACT_INPUTS_UPLOAD>
          Maximum compact inputs to upload [default: 10]
      --maximum-handles-per-input <MAXIMUM_HANDLES_PER_INPUT>
          Maximum compact inputs to upload [default: 255]
      --coprocessor-fhe-threads <COPROCESSOR_FHE_THREADS>
          Coprocessor FHE processing threads [default: 8]
      --tokio-threads <TOKIO_THREADS>
          Tokio Async IO threads [default: 4]
      --pg-pool-max-connections <PG_POOL_MAX_CONNECTIONS>
          Postgres pool max connections [default: 10]
      --server-addr <SERVER_ADDR>
          Server socket address [default: 127.0.0.1:50051]
      --metrics-addr <METRICS_ADDR>
          Prometheus metrics server address [default: 0.0.0.0:9100]
      --database-url <DATABASE_URL>
          Postgres database url. If unspecified DATABASE_URL environment variable is used
      --coprocessor-private-key <COPROCESSOR_PRIVATE_KEY>
          Coprocessor private key file path. Private key is in plain text 0x1234.. format [default: ./coprocessor.key]
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

##### host-listener

```bash
$ host_listener --help
Usage: host_listener [OPTIONS]

Options:
      --url <URL>                                      [default: ws://0.0.0.0:8746]
      --ignore-tfhe-events
      --ignore-acl-events
      --acl-contract-address <ACL_CONTRACT_ADDRESS>
      --tfhe-contract-address <TFHE_CONTRACT_ADDRESS>
      --database-url <DATABASE_URL>
      --start-at-block <START_AT_BLOCK>                Can be negative from last block
      --end-at-block <END_AT_BLOCK>
  -h, --help                                           Print help
  -V, --version                                        Print version
```

##### gw-listener

```bash
$ gw_listener --help
Usage: gw_listener [OPTIONS] --gw-url <GW_URL> --input-verification-address <INPUT_VERIFICATION_ADDRESS>

Options:
      --database-url <DATABASE_URL>
          
      --database-pool-size <DATABASE_POOL_SIZE>
          [default: 16]
      --verify-proof-req-database-channel <VERIFY_PROOF_REQ_DATABASE_CHANNEL>
          [default: verify_proof_requests]
      --gw-url <GW_URL>
          
  -i, --input-verification-address <INPUT_VERIFICATION_ADDRESS>
          
      --error-sleep-initial-secs <ERROR_SLEEP_INITIAL_SECS>
          [default: 1]
      --error-sleep-max-secs <ERROR_SLEEP_MAX_SECS>
          [default: 10]
  -h, --help
          Print help
  -V, --version
          Print version
```

For more info, please check [gw-listener README](fhevm-engine/gw-listener/README.md)

##### sns-worker

```bash
$ sns_worker --help
Usage: sns_worker [OPTIONS] --pg-listen-channel <PG_LISTEN_CHANNEL> --pg-notify-channel <PG_NOTIFY_CHANNEL>

Options:
      --work-items-batch-size <WORK_ITEMS_BATCH_SIZE>
          Work items batch size [default: 4]
      --pg-listen-channel <PG_LISTEN_CHANNEL>
          NOTIFY/LISTEN channel for database that the worker listen to
      --pg-notify-channel <PG_NOTIFY_CHANNEL>
          NOTIFY/LISTEN channel for database that the worker notify to
      --pg-polling-interval <PG_POLLING_INTERVAL>
          Polling interval in seconds [default: 60]
      --pg-pool-connections <PG_POOL_CONNECTIONS>
          Postgres pool connections [default: 10]
      --database-url <DATABASE_URL>
          Postgres database url. If unspecified DATABASE_URL environment variable is used
      --keys-file-path <KEYS_FILE_PATH>
          KeySet file. If unspecified the the keys are read from the database (not implemented)
      --service-name <SERVICE_NAME>
          sns-executor service name in OTLP traces (not implemented) [default: sns-executor]
  -h, --help
          Print help
  -V, --version
          Print version
```

##### zkproof-worker

```bash
$ zkproof_worker --help
Usage: zkproof_worker [OPTIONS]

Options:
  -d, --database-url <DATABASE_URL>

      --database-pool-size <DATABASE_POOL_SIZE>
          [default: 10]
      --database-polling-interval-secs <DATABASE_POLLING_INTERVAL_SECS>
          [default: 5]
  -v, --verify-proof-req-database-channel <VERIFY_PROOF_REQ_DATABASE_CHANNEL>
          [default: verify_proof_resquests]
  -t, --tokio-blocking-threads <TOKIO_BLOCKING_THREADS>
          [default: 16]
      --error-sleep-initial-secs <ERROR_SLEEP_INITIAL_SECS>
          [default: 1]
      --error-sleep-max-secs <ERROR_SLEEP_MAX_SECS>
          [default: 10]
  -h, --help
          Print help
  -V, --version
          Print version
```

##### transaction-sender

```bash
$ transaction_sender --help
Usage: transaction_sender [OPTIONS] --input-verification-address <INPUT_VERIFICATION_ADDRESS> --ciphertext-commits-address <CIPHERTEXT_COMMITS_ADDRESS> --multichain-acl-address <MULTICHAIN_ACL_ADDRESS> --gateway-url <GATEWAY_URL>

Options:
  -i, --input-verification-address <INPUT_VERIFICATION_ADDRESS>
          
  -c, --ciphertext-commits-address <CIPHERTEXT_COMMITS_ADDRESS>
          
  -m, --multichain-acl-address <MULTICHAIN_ACL_ADDRESS>
          
  -g, --gateway-url <GATEWAY_URL>
          
  -s, --signer-type <SIGNER_TYPE>
          [default: private-key] [possible values: private-key, aws-kms]
  -p, --private-key <PRIVATE_KEY>
          
  -d, --database-url <DATABASE_URL>
          
      --database-pool-size <DATABASE_POOL_SIZE>
          [default: 10]
      --database-polling-interval-secs <DATABASE_POLLING_INTERVAL_SECS>
          [default: 5]
      --verify-proof-resp-database-channel <VERIFY_PROOF_RESP_DATABASE_CHANNEL>
          [default: verify_proof_responses]
      --add-ciphertexts-database-channel <ADD_CIPHERTEXTS_DATABASE_CHANNEL>
          [default: add_ciphertexts]
      --allow-handle-database-channel <ALLOW_HANDLE_DATABASE_CHANNEL>
          [default: event_allowed_handle]
      --verify-proof-resp-batch-limit <VERIFY_PROOF_RESP_BATCH_LIMIT>
          [default: 128]
      --verify-proof-resp-max-retries <VERIFY_PROOF_RESP_MAX_RETRIES>
          [default: 3]
      --verify-proof-remove-after-max-retries
          
      --add-ciphertexts-batch-limit <ADD_CIPHERTEXTS_BATCH_LIMIT>
          [default: 10]
      --allow-handle-batch-limit <ALLOW_HANDLE_BATCH_LIMIT>
          [default: 10]
      --allow-handle-max-retries <ALLOW_HANDLE_MAX_RETRIES>
          [default: 10]
      --add-ciphertexts-max-retries <ADD_CIPHERTEXTS_MAX_RETRIES>
          [default: 15]
      --error-sleep-initial-secs <ERROR_SLEEP_INITIAL_SECS>
          [default: 1]
      --error-sleep-max-secs <ERROR_SLEEP_MAX_SECS>
          [default: 16]
      --txn-receipt-timeout-secs <TXN_RECEIPT_TIMEOUT_SECS>
          [default: 10]
      --required-txn-confirmations <REQUIRED_TXN_CONFIRMATIONS>
          [default: 0]
      --review-after-unlimited-retries <REVIEW_AFTER_UNLIMITED_RETRIES>
          [default: 30]
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

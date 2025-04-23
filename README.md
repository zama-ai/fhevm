<p align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://private-user-images.githubusercontent.com/1384478/421481269-6173e401-7c1b-4911-9731-ca2eb436e85f.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NDIzNzM0NjcsIm5iZiI6MTc0MjM3MzE2NywicGF0aCI6Ii8xMzg0NDc4LzQyMTQ4MTI2OS02MTczZTQwMS03YzFiLTQ5MTEtOTczMS1jYTJlYjQzNmU4NWYucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI1MDMxOSUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNTAzMTlUMDgzMjQ3WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9Y2QxMzBhMGJlY2UyMTAwYTg4NTFkOGM5MWRkZGJlYmZiMDgyNzNiYjQ5OTM4MWI5MzA5NGU0ZmI4NWFhNWZlNSZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QifQ.YViSBhLRoakk-dPU_lPcV3xDGvPUYqmzqo5eOyJsEWs">
  <source media="(prefers-color-scheme: light)" srcset="https://private-user-images.githubusercontent.com/1384478/421481269-6173e401-7c1b-4911-9731-ca2eb436e85f.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NDIzNzM0NjcsIm5iZiI6MTc0MjM3MzE2NywicGF0aCI6Ii8xMzg0NDc4LzQyMTQ4MTI2OS02MTczZTQwMS03YzFiLTQ5MTEtOTczMS1jYTJlYjQzNmU4NWYucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI1MDMxOSUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNTAzMTlUMDgzMjQ3WiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9Y2QxMzBhMGJlY2UyMTAwYTg4NTFkOGM5MWRkZGJlYmZiMDgyNzNiYjQ5OTM4MWI5MzA5NGU0ZmI4NWFhNWZlNSZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QifQ.YViSBhLRoakk-dPU_lPcV3xDGvPUYqmzqo5eOyJsEWs">
  <img width=600 alt="Zama fheVM">
</picture>
</p>

<!-- markdown-link-check-disable -->

<hr/>

<p align="center">
<a href="https://github.com/zama-ai/fhevm-solidity/blob/main/fhevm-whitepaper.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm-backend"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="https://github.com/zama-ai/fhevm-solidity/releases">
    <img src="https://img.shields.io/github/v/release/zama-ai/fhevm?style=flat-square"></a>
  <a href="https://github.com/zama-ai/fhevm-solidity/blob/main/LICENSE">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square"></a>
  <a href="https://slsa.dev"><img alt="SLSA 3" src="https://slsa.dev/images/gh-badge-level3.svg" /></a>
</p>

## About

### What is fheVM Backend

**fheVM Backend** provides the execution service for FHE computations.

It includes a **Coprocessor** service [fhEVM-coprocessor](https://docs.zama.ai/fhevm-backend/getting-started/fhevm/fhevm-coprocessor). The Coprocessor
itself consists of multiple microservices, e.g. for FHE compute, input verify, transaction sending, listenting to events, etc.

- An **Executor** service for [fheVM-native](https://docs.zama.ai/fhevm-backend/getting-started/fhevm/fhevm-native)
- A **Coprocessor** service for [fheVM-coprocessor](https://docs.zama.ai/fhevm-backend/getting-started/fhevm/fhevm-coprocessor)

_Learn more about fheVM-backend features in the [documentation](https://docs.zama.ai/fhevm-backend)._
<br></br>

## Table of Contents

- **[Getting started](#getting-started)**
  - [Generating keys](#generating-keys)
  - [Coprocessor](#coprocessor)
- **[Resources](#resources)**
  - [Documentation](#documentation)
  - [fheVM Demo](#fhevm-demo)
- **[Working with fheVM-backend](#working-with-fhevm-backend)**
  - [Citations](#citations)
  - [Contributing](#contributing)
  - [License](#license)
- **[Support](#support)**
  <br></br>

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

##### coprocessor

```bash
$ coprocessor --help
Usage: coprocessor [OPTIONS]

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

##### fhevm-listener

```bash
$ fhevm_listener --help
Usage: fhevm_listener [OPTIONS]

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
Usage: transaction_sender [OPTIONS] --input-verification-address <INPUT_VERIFICATION_ADDRESS> --ciphertext-commits-address <CIPHERTEXT_COMMITS_ADDRESS> --multichain-acl-address <MULTICHAIN_ACL_ADDRESS> --gateway-url <GATEWAY_URL> --private-key <PRIVATE_KEY> 

Options:
  -i, --input-verification-address <INPUT_VERIFICATION_ADDRESS>
          
  -c, --ciphertext-commits-address <CIPHERTEXT_COMMITS_ADDRESS>
          
  -m, --multichain-acl-address <MULTICHAIN_ACL_ADDRESS>
          
  -g, --gateway-url <GATEWAY_URL>
          
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
          [default: 15]
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
  -h, --help
          Print help
  -V, --version
          Print version
```

More details on configuration can be found in the [documentation](https://docs.zama.ai/fhevm-backend/getting-started/fhevm/fhevm-coprocessor/configuration).

## Resources

### Documentation

Full, comprehensive documentation is available here: [https://docs.zama.ai/fhevm-backend](https://docs.zama.ai/fhevm-backend).

### fheVM Demo

A complete demo showcasing an integrated fheVM blockchain and KMS (Key Management System) is available here: [https://github.com/zama-ai/fhevm-devops/](https://github.com/zama-ai/fhevm-devops/).

## Working with fheVM-backend

### Citations

To cite fheVM or the whitepaper in academic papers, please use the following entries:

```text
@Misc{fheVM,
title={{Private smart contracts on the EVM using homomorphic encryption}},
author={Zama},
year={2023},
note={\url{https://github.com/zama-ai/fhevm}},
}
```

```text
@techreport{fheVM,
author = "Morten Dahl, Clément Danjou, Daniel Demmler, Tore Frederiksen, Petar Ivanov,
Marc Joye, Dragos Rotaru, Nigel Smart, Louis Tremblay Thibault
",
title = "Confidential EVM Smart Contracts using Fully Homomorphic Encryption",
institution = "Zama",
year = "2023"
}
```

### Contributing

There are two ways to contribute to the Zama fheVM:

- [Open issues](https://github.com/zama-ai/fhevm-backend/issues/new/choose) to report bugs and typos, or to suggest new ideas
- Request to become an official contributor by emailing hello@zama.ai.

Becoming an approved contributor involves signing our Contributor License Agreement (CLA)). Only approved contributors can send pull requests, so please make sure to get in touch before you do!
<br></br>

### License

This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE) for more details.

#### FAQ

**Is Zama’s technology free to use?**

> Zama’s libraries are free to use under the BSD 3-Clause Clear license only for development, research, prototyping, and experimentation purposes. However, for any commercial use of Zama's open source code, companies must purchase Zama’s commercial patent license.
>
> Everything we do is open source and we are very transparent on what it means for our users, you can read more about how we monetize our open source products at Zama in [this blog post](https://www.zama.ai/post/open-source).

**What do I need to do if I want to use Zama’s technology for commercial purposes?**

> To commercially use Zama’s technology you need to be granted Zama’s patent license. Please contact us at hello@zama.ai for more information.

**Do you file IP on your technology?**

> Yes, all Zama’s technologies are patented.

**Can you customize a solution for my specific use case?**

> We are open to collaborating and advancing the FHE space with our partners. If you have specific needs, please email us at hello@zama.ai.

<p align="right">
  <a href="#table-of-contents" > ↑ Back to top </a>
</p>

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/fhevm-solidity/assets/157474013/e249e1a8-d724-478c-afa8-e4fe01c1a0fd">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/fhevm-solidity/assets/157474013/a72200cc-d93e-44c7-81a8-557901d8798d">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

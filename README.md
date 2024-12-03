<p align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/b07e7e65-12b2-4048-b5de-35e169ed96e4">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/c0fab5b1-adef-4db4-9607-fa0a793acaf8">
  <img width=600 alt="Zama fhEVM">
</picture>
</p>

<!-- markdown-link-check-disable -->

<hr/>

<p align="center">
<a href="https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm-backend"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="https://github.com/zama-ai/fhevm/releases">
    <img src="https://img.shields.io/github/v/release/zama-ai/fhevm?style=flat-square"></a>
  <a href="https://github.com/zama-ai/fhevm/blob/main/LICENSE">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program">
    <!-- markdown-link-check-disable-next-line -->
    <img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square"></a>
  <a href="https://slsa.dev"><img alt="SLSA 3" src="https://slsa.dev/images/gh-badge-level3.svg" /></a>
</p>


## About

### What is fhEVM-backend

**fhEVM-backend** provides the execution service for FHE computations.

It includes:
- An **Executor** service for [fhEVM-native](https://docs.zama.ai/fhevm-backend/getting-started/fhevm/fhevm-native) 
- A **Coprocessor** service for [fhEVM-coprocessor](https://docs.zama.ai/fhevm-backend/getting-started/fhevm/fhevm-coprocessor)

*Learn more about fhEVM-backend features in the [documentation](https://docs.zama.ai/fhevm-backend).*
<br></br>

## Table of Contents
- **[Getting started](#getting-started)**
   - [Generating keys](#generating-keys)
   - [Executor](#executor)
   - [Coprocessor](#coprocessor)
- **[Resources](#resources)**
   - [Documentation](#documentation)
   - [fhEVM Demo](#fhevm-demo)
- **[Working with fhEVM-backend](#working-with-fhevm-backend)**
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

### Executor

#### Installation

```
$ cd fhevm-engine/executor
$ cargo install --path .
```

#### Configuration

Once installed, the executor can be started with the following configuration options:
```
$ executor --help
Usage: executor [OPTIONS] --fhe-keys-directory <FHE_KEYS_DIRECTORY>

Options:
      --tokio-threads <TOKIO_THREADS>
          [default: 4]
      --fhe-compute-threads <FHE_COMPUTE_THREADS>
          [default: 32]
      --policy-fhe-compute-threads <POLICY_FHE_COMPUTE_THREADS>
          [default: 32]
      --server-addr <SERVER_ADDR>
          [default: 127.0.0.1:50051]
      --fhe-keys-directory <FHE_KEYS_DIRECTORY>
          directory for fhe keys, target directory expected to contain files named: sks (server evaluation key), pks (compact public key), pp (public key params)
  -h, --help
          Print help
  -V, --version
          Print version
```

More details on configuration can be found in the [documentation](https://docs.zama.ai/fhevm-backend/getting-started/fhevm/fhevm-native/configuration).

### Coprocessor

#### Dependences

- `docker-compose`
- `rust`
- `sqlx-cli` (install with `cargo install sqlx-cli`)

#### Installation

```
$ cd fhevm-engine/coprocessor
$ cargo install --path .
```

#### Configuration

```
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

More details on configuration can be found in the [documentation](https://docs.zama.ai/fhevm-backend/getting-started/fhevm/fhevm-coprocessor/configuration).

## Resources

### Documentation

Full, comprehensive documentation is available here: [https://docs.zama.ai/fhevm-backend](https://docs.zama.ai/fhevm-backend).

### fhEVM Demo

A complete demo showcasing an integrated fhEVM blockchain and KMS (Key Management System) is available here: [https://github.com/zama-ai/fhevm-devops](https://github.com/zama-ai/fhevm-devops).


## Working with fhEVM-backend

### Citations

To cite fhEVM or the whitepaper in academic papers, please use the following entries:

```text
@Misc{fhEVM,
title={{Private smart contracts on the EVM using homomorphic encryption}},
author={Zama},
year={2023},
note={\url{https://github.com/zama-ai/fhevm}},
}
```

```text
@techreport{fhEVM,
author = "Morten Dahl, Clément Danjou, Daniel Demmler, Tore Frederiksen, Petar Ivanov,
Marc Joye, Dragos Rotaru, Nigel Smart, Louis Tremblay Thibault
",
title = "Confidential EVM Smart Contracts using Fully Homomorphic Encryption",
institution = "Zama",
year = "2023"
}
```

### Contributing

There are two ways to contribute to the Zama fhEVM:

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
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/e249e1a8-d724-478c-afa8-e4fe01c1a0fd">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/a72200cc-d93e-44c7-81a8-557901d8798d">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.

<p align="right">
  <a href="#about" > ↑ Back to top </a>
</p>

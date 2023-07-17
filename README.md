# fhEVM

A Solidity library for interacting with an fhEVM blockchain.

## Install

```bash
npm install fhevm
```

or

```bash
yarn install fhevm
```

## Usage

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "fhevm/lib/TFHE.sol";

contract Counter {
  euint32 counter;

  function add(bytes calldata encryptedValue) public {
    euint32 value = TFHE.asEuint32(encryptedValue);
    counter = TFHE.add(counter, value);
  }

  function getCounter(bytes32 publicKey) returns (bytes memory) {
    return TFHE.reencrypt(counter, publicKey);
  }
}
```

See our documentation on [https://docs.zama.ai/homepage/](https://docs.zama.ai/homepage/) for more examples.

## Development Guide

Install dependencies (Solidity libraries and dev tools)

```bash
npm install
```

Note: Solidity files are formatted with prettier.

### Generate TFHE lib

```
npm run codegen
```

WARNING: Use this command to generate Solidity code and prettier result automatically!

### Demo test
> **_NOTE:_**  The following test works on the x86_64 architecture. To run on ARM64, edit the `ci/docker-compose.yml` file to use the correct `FHEVM_TFHE_CLI_TAG` value (e.g., `v0.1.1-arm64`).

This repository includes a python script (see [demo_test.py](demo_test.py)) that automates a sequence of steps simulating deployment and interaction with an encrypted ERC20 contract.

Because inputs must be encrypted using the blockchain global public key, a tool called [`fhevm-tfhe-cli`](https://github.com/zama-ai/fhevm-tfhe-cli) is available.

We also need the blockchain public key, please copy it to **keys/network-public-fhe-keys/pks**.

The python script accepts two arguments:

1. The private key of the an account which owns some native coins;
2. [optionnal --node_address] The node URL (default is http://host.docker.internal:8545).

To install all the required python modules, a docker containing a `fhevm-tfhe-cli` binary is available.

Run the demo test:

```
$ npm i
$ export PRIVATE_KEY=CCABB56366...
$ docker compose -f ci/docker-compose.yml run app python demo_test.py $PRIVATE_KEY
```

<br />
<details>
  <summary>Install fhevm-tfhe-cli</summary>
<br />

```
make install-fhevm-tfhe-cli
```

The binary will be available at **work_dir/fhevm-tfhe-cli/target/release/fhevm-tfhe-cli**
</details>
<br />

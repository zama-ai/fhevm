# fhEVM

A Solidity library for interacting with the Zama Blockchain

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

  constructor() {}

  function add(bytes calldata encryptedValue) public {
    euint32 value = TFHE.asEuint32(encryptedValue);
    counter = TFHE.add(world, value);
  }

  function getCounter(bytes32 publicKey) returns (bytes memory) {
    return TFHE.reencrypt(totalSupply, publicKey);
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

### Demo Test

This repository includes a python script (see [demo_test.py](demo_test.py)) that automates a sequence of steps simulating deployment and interaction with an encrypted ERC20 contract.

Because inputs must be encrypted using the blockchain global public key, a tool called zbc-fhe-tool is available.

We also need the blockchain public key, please copy it into **keys/network-public-fhe-keys/** named **pks**.

The python script accepts two arguments:

1. The private key of the main account which owns funds
2. [optionnal --node_address] The node @ (default is http://host.docker.internal:8545)

To install all the required python modules, a docker is available containing zbc-fhe-tool binary.

Run the demo test:

```
export PRIVATE_KEY=CCABB56366...
docker compose -f ci/docker-compose.yml run app python demo_test.py $PRIVATE_KEY
```

<br />
<details>
  <summary>Install zbc-fhe-tools</summary>
<br />

```
make install-zbc-fhe-tool
```

#The binary will be available at **work_dir/zbc-fhe-tool/target/release/zbc-fhe-tool**

</details>
<br />

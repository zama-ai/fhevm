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

### Test

```
npm test
```

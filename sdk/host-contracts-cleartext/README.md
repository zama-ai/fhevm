# Goal

- create a standalone package named '@fhevm/host-contracts-cleartext'
- this package offers the cleartext contracts for a specific version of the FHEVM protocol (v11, v12, v13, v14, v15, ...)
- it is meant to be used in a hardhat project (as a dependency)
- it is meant to be used in a foundry project (as a dependency)
- it should contain a forge script to allow deploy using forge
- it should contain a viem ts lib to deploy in TS
- The big change is: <fhevm>/host-contracts-cleartext/addresses/FHEVMHostAddresses.sol
  as described bellow.
  This should allow easy remapping

# Structure

<fhevm>/host-contracts-cleartext
<fhevm>/host-contracts-cleartext/package.json

```json
{
  "name": "@fhevm/host-contracts-cleartext",
  "description": "Cleartext FHEVM Host Contracts",
  "version": "0.13.0",
  "type": "module",
  ...
}
```

<fhevm>/host-contracts-cleartext/foundry.toml

<fhevm>/host-contracts-cleartext/addresses/FHEVMHostAddresses.sol

<fhevm>/host-contracts-cleartext/host-contracts/
<fhevm>/host-contracts-cleartext/host-contracts/ACL.sol
<fhevm>/host-contracts-cleartext/host-contracts/InputVerifier.sol
...

<fhevm>/host-contracts-cleartext/cleartext/CleartextFHEVMExecutor.sol
...

## Remapping

```toml
remappings = [
    "forge-std/=dependencies/forge-std-1.11.0/src/",
    "encrypted-types/=dependencies/@encrypted-types-0.0.4/",
    "@openzeppelin-contracts-5.1.0/=dependencies/@openzeppelin-contracts-5.1.0/",
    "@openzeppelin-contracts-upgradeable-5.1.0/=dependencies/@openzeppelin-contracts-upgradeable-5.1.0/",
    "@openzeppelin/contracts/=dependencies/@openzeppelin-contracts-5.1.0/",
    "@openzeppelin/contracts-upgradeable/=dependencies/@openzeppelin-contracts-upgradeable-5.1.0/",
    "@fhevm/solidity/=../../../library-solidity/",
    "@fhevm/host-contracts-addresses-0.13.0/=<relpath to dir containing addresses.sol for protocol v13>",
    "@fhevm/host-contracts-cleartext-0.13.0/=node_modules/@fhevm/host-contracts@0.13.0/",
    "@fhevm/host-contracts-cleartext/=node_modules/@fhevm/host-contracts@0.13.0/",
]
```

## addresses.sol

Replace <fhevm>/host-contracts-cleartext/addresses/FHEVMHostAddresses.sol content:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

address constant aclAdd = address(0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D);
address constant fhevmExecutorAdd = address(0xe3a9105a3a932253A70F126eb1E3b589C643dD24);
address constant kmsVerifierAdd = address(0x901F8942346f7AB3a01F6D7613119Bca447Bb030);
address constant inputVerifierAdd = address(0x36772142b74871f255CbD7A3e89B401d3e45825f);
address constant hcuLimitAdd = address(0x233ff88A48c172d29F675403e6A8e302b0F032D9);
address constant pauserSetAdd = address(0x34e3eD8472e409dbF8FDf933cA996DC75e4Be126);
```

with:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {
  ACL_ADDRESS,
  FHEVM_EXECUTOR_ADDRESS,
  KMS_VERIFIER_ADDRESS,
  INPUT_VERIFIER_ADDRESS,
  HCU_LIMIT_ADDRESS,
  PAUSER_SET_ADDRESS
} from '@fhevm/host-contracts-addresses-0.13.0/addresses.sol';

address constant aclAdd = ACL_ADDRESS;
address constant fhevmExecutorAdd = FHEVM_EXECUTOR_ADDRESS;
address constant kmsVerifierAdd = KMS_VERIFIER_ADDRESS;
address constant inputVerifierAdd = INPUT_VERIFIER_ADDRESS;
address constant hcuLimitAdd = HCU_LIMIT_ADDRESS;
address constant pauserSetAdd = PAUSER_SET_ADDRESS;
```

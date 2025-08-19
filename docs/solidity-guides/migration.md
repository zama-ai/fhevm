# Migration

This document provides instructions on migrating from FHEVM v0.6 to v0.7.

## From 0.6.x

### Package and library

The package is now `@fhevm/solidity` instead of `FHEVM` and the library name has changed from `TFHE` to `FHE`

```solidity
import { FHE } from "@fhevm/solidity";
```

### Configuration

Configuration has been renamed from `SepoliaZamaConfig` to `SepoliaConfig`.

```solidity
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";
```

Also, the function to define manually the Coprocessor has been renamed from `setFHEVM` to `setCoprocessor`, and the function to define the oracle is now integrated into `setCoprocessor`.

```solidity
import { ZamaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";
constructor () {
  FHE.setCoprocessor(ZamaConfig.getSepoliaConfig());
}
```

You can read more about [Configuration on the dedicated page](configure.md).

### Decryption Oracle

Previously, an abstract contract `GatewayCaller` was used to request decryption. It has been replaced by `FHE.requestDecryption`:

```solidity
function requestBoolInfinite() public {
  bytes32[] memory cts = new bytes32[](1);
  cts[0] = FHE.toBytes32(myEncryptedValue);
  FHE.requestDecryption(cts, this.myCallback.selector);
}
```

You can read more about [Decryption Oracle on the dedicated page](decryption/oracle.md).

### Deprecation of ebytes

`ebytes` has been deprecated and removed from FHEVM.

### Block gas limit

Block gas limit has been removed in favor of HCU (Homomorphic Complexity Unit) limit. FHEVM 0.7.0 includes two limits:

- **Sequential homomorphic operations depth limit per transaction**: Controls HCU usage for operations that must be processed in order. This limit is set to **5,000,000** HCU.
- **Global homomorphic operations complexity per transaction**: Controls HCU usage for operations that can be processed in parallel. This limit is set to **20,000,000** HCU.

You can read more about [HCU on the dedicated page](hcu.md).

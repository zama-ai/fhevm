# Migration

This document provides instructions on migrating from a previous version of FHEVM.

## From 0.7.x

### Decryption Oracle

Callbacks are now implemented using an ABI-encoded bytes value, which includes all cleartexts.

```solidity
function myCustomCallback(uint256 requestId, bytes memory cleartexts, bytes memory decryptionProof) public returns (bool) {
    /// @dev This check is used to verify that the request id is the expected one.
    require(requestId == latestRequestId, "Invalid requestId");
    FHE.checkSignatures(requestId, cleartexts, decryptionProof);

    (bool decryptedInput) = abi.decode(cleartexts, (bool));
    yBool = decryptedInput;
    isDecryptionPending = false;
    return yBool;
  }
}
```

`function setDecryptionOracle(address decryptionOracle)` is now deprecated. The decryption oracle address is now configured through function `setCoprocessor(CoprocessorConfig memory coprocessorConfig)`. For example:
```solidity
  CoprocessorConfig({
      ACLAddress: 0x687820221192C5B662b25367F70076A37bc79b6c,
      CoprocessorAddress: 0x848B0066793BcC60346Da1F49049357399B8D595,
      DecryptionOracleAddress: 0xa02Cda4Ca3a71D7C46997716F4283aa851C28812,
      KMSVerifierAddress: 0x1364cBBf2cDF5032C47d8226a6f6FBD2AFCDacAC
  });
```

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

Callbacks are now implemented using an ABI-encoded bytes value, which includes all cleartexts.

```solidity
function myCustomCallback(uint256 requestId, bytes memory cleartexts, bytes memory decryptionProof) public returns (bool) {
    /// @dev This check is used to verify that the request id is the expected one.
    require(requestId == latestRequestId, "Invalid requestId");
    FHE.checkSignatures(requestId, cleartexts, decryptionProof);

    (bool decryptedInput) = abi.decode(cleartexts, (bool));
    yBool = decryptedInput;
    isDecryptionPending = false;
    return yBool;
  }
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

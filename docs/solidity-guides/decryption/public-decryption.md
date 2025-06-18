# Public decryption

This section explains how to handle decryption in fhevm. Decryption allows plaintext data to be accessed when required for contract logic or user presentation, ensuring confidentiality is maintained throughout the process.

{% hint style="info" %}
Understanding how encryption, user decryption, and public decryption works is a prerequisite before implementation, see [Encryption, User Decryption, Public Decryption, and Computation](../../protocol/d_re_ecrypt_compute.md).
{% endhint %}

Decryption is essential in two primary cases:

1. **Smart contract logic**: A contract requires plaintext values for computations or decision-making.
2. **User interaction**: Plaintext data needs to be revealed to all users, such as revealing the decision of the vote.

To learn how decryption works see [Encryption, User decryption, Public decryption, and Computation](../../protocol/d_re_ecrypt_compute.md).

## Overview

Decryption in FHEVM is an asynchronous process that involves the Relayer and Key Management System (KMS).
Here’s an example of how to safely request decryption in a contract.

### Example: asynchronous decryption in a contract

```solidity
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract TestAsyncDecrypt is SepoliaConfig {
  ebool xBool;
  bool public yBool;
  bool isDecryptionPending;
  uint256 latestRequestId;

  constructor() {
      xBool = FHE.asEbool(true);
      FHE.allowThis(xBool);
  }

  function requestBool() public {
    require(!isDecryptionPending, "Decryption is in progress");
    bytes32[] memory cts = new bytes32[](1);
    cts[0] = FHE.toBytes32(xBool);
    uint256 latestRequestId = FHE.requestDecryption(cts, this.myCustomCallback.selector);

    /// @dev This prevents sending multiple requests before the first callback was sent.
    isDecryptionPending = true;
  }

  function myCustomCallback(uint256 requestId, bool decryptedInput, bytes[] memory signatures) public returns (bool) {
    /// @dev This check is used to verify that the request id is the expected one.
    require(requestId == latestRequestId, "Invalid requestId");
    FHE.checkSignatures(requestId, signatures);
    yBool = decryptedInput;
    isDecryptionPending = false;
    return yBool;
  }
}
```

### Next steps

Explore advanced decryption techniques and learn more about user decryption:

- [Decryption in depth](decrypt_details.md)
- [User decryption](user-decryption.md)

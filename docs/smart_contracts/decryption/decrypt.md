# Decryption

This section explains how to handle decryption in fhEVM. Decryption allows plaintext data to be accessed when required for contract logic or user presentation, ensuring confidentiality is maintained throughout the process.

{% hint style="info" %}
Understanding how encryption, decryption and reencryption works is a prerequisit before implementation, see [Encryption, Decryption, Re-encryption, and Computation](../d_re_ecrypt_compute.md).
{% endhint %}

Decryption is essential in two primary cases:

1. **Smart contract logic**: A contract requires plaintext values for computations or decision-making.
2. **User interaction**: Plaintext data needs to be revealed to all users, such as revealing the decision of the vote.

To learn how decryption works see [Encryption, Decryption, Re-encryption, and Computation](../d_re_ecrypt_compute.md)

## Overview

Decryption in fhEVM is an asynchronous process that involves the Gateway and Key Management System (KMS). Contracts requiring decryption must extend the GatewayCaller contract, which imports the necessary libraries and provides access to the Gateway.

Here’s an example of how to request decryption in a contract:

### Example: asynchronous decryption in a contract

```solidity
pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";
import { SepoliaZamaGatewayConfig } from "fhevm/config/ZamaGatewayConfig.sol";
import "fhevm/gateway/GatewayCaller.sol";

contract TestAsyncDecrypt is SepoliaZamaFHEVMConfig, SepoliaZamaGatewayConfig, GatewayCaller {
  ebool xBool;
  bool public yBool;

  constructor() {
      xBool = TFHE.asEbool(true);
      TFHE.allowThis(xBool);
  }

  function requestBool() public {
    uint256[] memory cts = new uint256[](1);
    cts[0] = Gateway.toUint256(xBool);
    Gateway.requestDecryption(cts, this.myCustomCallback.selector, 0, block.timestamp + 100, false);
  }

  function myCustomCallback(uint256 /*requestID*/, bool decryptedInput) public onlyGateway returns (bool) {
    yBool = decryptedInput;
    return yBool;
  }
```

#### Key additions to the code

1.  **Configuration imports**: The configuration contracts are imported to set up the FHEVM environment and Gateway.

    ```solidity
    import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";
    import { SepoliaZamaGatewayConfig } from "fhevm/config/ZamaGatewayConfig.sol";
    ```

2.  **`GatewayCaller` import**:\
    The `GatewayCaller` contract is imported to enable decryption requests.

    ```solidity
    import "fhevm/gateway/GatewayCaller.sol";
    ```

### Next steps

Explore advanced decryption techniques and learn more about re-encryption:

- [Decryption in depth](decrypt_details.md)
- [Re-encryption](reencryption.md)

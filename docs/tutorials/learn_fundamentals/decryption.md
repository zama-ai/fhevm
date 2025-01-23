# Decrypt for everyone

Here’s an improved version of our Counter contract, upgraded to support decryption:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";
import { SepoliaZamaGatewayConfig } from "fhevm/config/ZamaGatewayConfig.sol";
import "fhevm/gateway/GatewayCaller.sol";

/// @title EncryptedCounter3
/// @notice A contract that maintains an encrypted counter and is meant for demonstrating how decryption works
/// @dev Uses TFHE library for fully homomorphic encryption operations and Gateway for decryption
/// @custom:experimental This contract is experimental and uses FHE technology with decryption capabilities
contract EncryptedCounter3 is SepoliaZamaFHEVMConfig, SepoliaZamaGatewayConfig, GatewayCaller {
  /// @dev Decrypted state variable
  euint8 internal counter;
  uint8 public decryptedCounter;

  function incrementBy(einput amount, bytes calldata inputProof) public {
    // Convert input to euint8 and add to counter
    euint8 incrementAmount = TFHE.asEuint8(amount, inputProof);
    counter = TFHE.add(counter, incrementAmount);
    TFHE.allowThis(counter);
  }

  /// @notice Request decryption of the counter value
  function requestDecryptCounter() public {
    uint256[] memory cts = new uint256[](1);
    cts[0] = Gateway.toUint256(counter);
    Gateway.requestDecryption(cts, this.callbackCounter.selector, 0, block.timestamp + 100, false);
  }

  /// @notice Callback function for counter decryption
  /// @param decryptedInput The decrypted counter value
  /// @return The decrypted value
  function callbackCounter(uint256, uint8 decryptedInput) public onlyGateway returns (uint8) {
    decryptedCounter = decryptedInput;
    return decryptedInput;
  }

  /// @notice Get the decrypted counter value
  /// @return The decrypted counter value
  function getDecryptedCounter() public view returns (uint8) {
    return decryptedCounter;
  }
}
```

### Tests for `EncryptedCounter3`

Here’s a sample test for the Encrypted Counter contract using Hardhat:

```ts
import { awaitAllDecryptionResults, initGateway } from "../asyncDecrypt";
import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("EncryptedCounter3", function () {
  before(async function () {
    await initSigners(); // Initialize signers
    this.signers = await getSigners();
    await initGateway(); // Initialize the gateway for decryption
  });

  beforeEach(async function () {
    const CounterFactory = await ethers.getContractFactory("EncryptedCounter3");
    this.counterContract = await CounterFactory.connect(this.signers.alice).deploy();
    await this.counterContract.waitForDeployment();
    this.contractAddress = await this.counterContract.getAddress();
    this.instances = await createInstance(); // Set up instances for testing
  });

  it("should increment counter and decrypt the result", async function () {
    // Create encrypted input for amount to increment by
    const input = this.instances.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(5); // Increment by 5 as an example
    const encryptedAmount = await input.encrypt();

    // Call incrementBy with encrypted amount
    const tx = await this.counterContract.incrementBy(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();

    const tx4 = await this.counterContract.connect(this.signers.carol).requestDecryptCounter();
    await tx4.wait();

    // Wait for decryption to complete
    await awaitAllDecryptionResults();

    // Check decrypted value (should be 5: initial 0 + an increment of 5)
    const decryptedValue = await this.counterContract.getDecryptedCounter();
    expect(decryptedValue).to.equal(5);
  });
});
```

#### Key additions in testing

1.  **Initialize the Gateway**:

    ```typescript
    await initGateway(); // Initialize the gateway for decryption
    ```

2.  **Request decryption and wait for results**:

    ```typescript
    const decryptTx = await this.counterContract.requestDecryptCounter({ gasLimit: 5_000_000 });
    await decryptTx.wait();
    await awaitAllDecryptionResults();
    ```

3.  **Verify the decrypted value**:

    ```typescript
    const decryptedValue = await this.counterContract.getDecryptedCounter();
    expect(decryptedValue).to.equal(5);
    ```

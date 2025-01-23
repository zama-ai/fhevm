# Decrypt only for the user

Here’s an enhanced **Encrypted Counter** example where each user maintains their own encrypted counter. Re-encryption is used to securely share counter values with individual users.

### Encrypted counter with re-encryption

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";

/// @title EncryptedCounter4
/// @notice A contract that maintains encrypted counters for each user and is meant for demonstrating how re-encryption works
/// @dev Uses TFHE library for fully homomorphic encryption operations
/// @custom:security Each user can only access and modify their own counter
/// @custom:experimental This contract is experimental and uses FHE technology
contract EncryptedCounter4 is SepoliaZamaFHEVMConfig {
  // Mapping from user address to their encrypted counter value
  mapping(address => euint8) private counters;

  function incrementBy(einput amount, bytes calldata inputProof) public {
    // Initialize counter if it doesn't exist
    if (!TFHE.isInitialized(counters[msg.sender])) {
      counters[msg.sender] = TFHE.asEuint8(0);
    }

    // Convert input to euint8 and add to sender's counter
    euint8 incrementAmount = TFHE.asEuint8(amount, inputProof);
    counters[msg.sender] = TFHE.add(counters[msg.sender], incrementAmount);
    TFHE.allowThis(counters[msg.sender]);
    TFHE.allow(counters[msg.sender], msg.sender);
  }

  function getCounter() public view returns (euint8) {
    // Return the encrypted counter value for the sender
    return counters[msg.sender];
  }
}
```

### Frontend code of re-encryption / tests for EncryptedCounter4

Here’s a sample test to verify re-encryption functionality:

```ts
import { createInstance } from "../instance";
import { reencryptEuint8 } from "../reencrypt";
import { getSigners, initSigners } from "../signers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("EncryptedCounter4", function () {
  before(async function () {
    await initSigners(); // Initialize signers
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const CounterFactory = await ethers.getContractFactory("EncryptedCounter4");
    this.counterContract = await CounterFactory.connect(this.signers.alice).deploy();
    await this.counterContract.waitForDeployment();
    this.contractAddress = await this.counterContract.getAddress();
    this.instances = await createInstance();
  });

  it("should allow reencryption and decryption of counter value", async function () {
    const input = this.instances.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(1); // Increment by 1 as an example
    const encryptedAmount = await input.encrypt();

    // Call incrementBy with encrypted amount
    const tx = await this.counterContract.incrementBy(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();

    // Get the encrypted counter value
    const encryptedCounter = await this.counterContract.getCounter();

    const decryptedValue = await reencryptEuint8(
      this.signers,
      this.instances,
      "alice",
      encryptedCounter,
      this.contractAddress,
    );

    // Verify the decrypted value is 1 (since we incremented once)
    expect(decryptedValue).to.equal(1);
  });

  it("should allow reencryption of counter value", async function () {
    const input = this.instances.createEncryptedInput(this.contractAddress, this.signers.bob.address);
    input.add8(1); // Increment by 1 as an example
    const encryptedAmount = await input.encrypt();

    // Call incrementBy with encrypted amount
    const tx = await this.counterContract
      .connect(this.signers.bob)
      .incrementBy(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();

    // Get the encrypted counter value
    const encryptedCounter = await this.counterContract.connect(this.signers.bob).getCounter();

    const decryptedValue = await reencryptEuint8(
      this.signers,
      this.instances,
      "bob",
      encryptedCounter,
      this.contractAddress,
    );

    // Verify the decrypted value is 1 (since we incremented once)
    expect(decryptedValue).to.equal(1);
  });
});
```

#### Key additions in testing

- **`setupReencryption():`** Prepares the re-encryption process by generating keys and a signature for the user.
- **`instance.reencrypt():`** Facilitates re-encryption and local decryption of the data for testing purposes.
- **Validation:** Confirms that the decrypted counter matches the expected value.

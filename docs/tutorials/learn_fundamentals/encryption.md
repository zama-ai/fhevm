# Add encrypted inputs

Now that we have new knowledge on how to add encrypted inputs, let's upgrade our counter contract.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";

/// @title EncryptedCounter2
/// @notice A contract that maintains an encrypted counter and is meant for demonstrating how to add encrypted types
/// @dev Uses TFHE library for fully homomorphic encryption operations
/// @custom:experimental This contract is experimental and uses FHE technology
contract EncryptedCounter2 is SepoliaZamaFHEVMConfig {
  euint8 internal counter;

  function incrementBy(einput amount, bytes calldata inputProof) public {
    // Convert input to euint8 and add to counter
    euint8 incrementAmount = TFHE.asEuint8(amount, inputProof);
    counter = TFHE.add(counter, incrementAmount);
    TFHE.allowThis(counter);
  }
}
```

### Tests of the Counter contract

```ts
import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { ethers } from "hardhat";

describe("EncryptedCounter2", function () {
  before(async function () {
    await initSigners(); // Initialize signers
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const CounterFactory = await ethers.getContractFactory("EncryptedCounter2");
    this.counterContract = await CounterFactory.connect(this.signers.alice).deploy();
    await this.counterContract.waitForDeployment();
    this.contractAddress = await this.counterContract.getAddress();
    this.instances = await createInstance(); // Set up instances for testing
  });

  it("should increment by arbitrary encrypted amount", async function () {
    // Create encrypted input for amount to increment by
    const input = this.instances.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(5);
    const encryptedAmount = await input.encrypt();

    // Call incrementBy with encrypted amount
    const tx = await this.counterContract.incrementBy(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
  });
});
```

### How it works

The `EncryptedCounter2` contract builds on the previous example by adding support for encrypted inputs. Here's how it works:

1. **Encrypted state**: Like before, the contract maintains an encrypted counter state variable of type `euint8`.
2. **Encrypted input handling**: The `incrementBy` function accepts two parameters:
   - `einput amount`: An encrypted input handle representing the increment value
   - `bytes calldata inputProof`: The zero-knowledge proof validating the encrypted input
3. **Input processing**: Inside `incrementBy`:
   - The encrypted input is converted to a `euint8` using `TFHE.asEuint8()`
   - This conversion validates the proof and creates a usable encrypted value
   - The value is then added to the counter using homomorphic addition

### Limitations

While we have resolved our problem with the Counter value visibility, there is still the problem with the Access Control for the `counter`.
The counter is encrypted, but no access is granted to decrypt or view its value. Without proper ACL permissions, the counter remains inaccessible to users. To resolve this, refer to:

- [Decryption](decryption/decrypt.md)
- [Re-encryption](decryption/reencryption.md)

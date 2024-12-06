---
layout:
  title:
    visible: true
  description:
    visible: true
  tableOfContents:
    visible: true
  outline:
    visible: true
  pagination:
    visible: false
---

# Create a smart contract

This document introduces the fundamentals of writing confidential smart contracts using the fhEVM. You'll learn how to create contracts that can perform computations on encrypted data while maintaining data privacy.

In this guide, we'll walk through creating a basic smart contract that demonstrates core fhEVM concepts and encrypted operations.

## Your first smart contract

Let’s build a simple **Encrypted Counter** smart contract to demonstrate the configuration process and the use of encrypted state variables.

### Writing the contract

Create a new file called `EncryptedCounter.sol` in your `contracts/` folder and add the following code:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";

/// @title EncryptedCounter1
/// @notice A basic contract demonstrating the setup of encrypted types
/// @dev Uses TFHE library for fully homomorphic encryption operations
/// @custom:experimental This is a minimal example contract intended only for learning purposes
/// @custom:notice This contract has limited real-world utility and serves primarily as a starting point
/// for understanding how to implement basic FHE operations in Solidity
contract EncryptedCounter1 is SepoliaZamaFHEVMConfig {
  euint8 counter;
  euint8 CONST_ONE;

  constructor() {
    // Initialize counter with an encrypted zero value
    counter = TFHE.asEuint8(0);
    TFHE.allowThis(counter);
    // Save on gas by computing the constant here
    CONST_ONE = TFHE.asEuint8(1);
    TFHE.allowThis(CONST_ONE);
  }

  function increment() public {
    // Perform encrypted addition to increment the counter
    counter = TFHE.add(counter, CONST_ONE);
    TFHE.allowThis(counter);
  }
}
```

#### How it works

1.  **Configuring fhEVM**:\
    The contract inherits from `SepoliaZamaFHEVMConfig` which provides the necessary configuration for local development and testing. This configuration includes the addresses of the TFHE library and Gateway contracts.

    When deploying to different networks, you can use the appropriate configuration:

    ```solidity
    // For Sepolia testnet
    import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";
    contract MyContract is SepoliaZamaFHEVMConfig { ... }
    ```

    The configuration handles setting up:

    - TFHE library address for encrypted operations
    - Network-specific parameters

2.  **Initializing encrypted variables**:
    - The `counter` variable is set to an encrypted `0` using `TFHE.asEuint8(0)`.
    - Permissions are granted to the contract itself for the `counter` ciphertext using `TFHE.allowThis(counter)`.
    - A constant `CONST_ONE` is initialized as an encrypted value to represent the number `1`.
3.  **Encrypted operations**:\
    The `increment()` function adds the encrypted constant `CONST_ONE` to the `counter` using `TFHE.add`.

#### Limitations:

There are two notable issues with this contract:

1. **Counter value visibility**:\
   Since the counter is incremented by a fixed value, observers could deduce its value by analyzing blockchain events. To address this, see the documentation on:
   - [encryption and secure inputs](../fundamentals/inputs.md)
2. **Access control for `counter`**:\
   The counter is encrypted, but no access is granted to decrypt or view its value. Without proper ACL permissions, the counter remains inaccessible to users. To resolve this, refer to:
   - [decryption](../fundamentals/decryption/decrypt.md)
   - [re-encryption](../fundamentals/decryption/reencryption.md)

### Testing

With any contracts that you write you will need to write tests as well. You can start by using something like this as a template:

```ts
import { createInstance } from "../instance";
import { getSigners, initSigners } from "../signers";
import { ethers } from "hardhat";

describe("EncryptedCounter1", function () {
  before(async function () {
    await initSigners(); // Initialize signers
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const CounterFactory = await ethers.getContractFactory("EncryptedCounter1");
    this.counterContract = await CounterFactory.connect(this.signers.alice).deploy();
    await this.counterContract.waitForDeployment();
    this.contractAddress = await this.counterContract.getAddress();
    this.instances = await createInstance();
  });

  it("should increment the counter", async function () {
    // Perform the increment action
    const tx = await this.counterContract.increment();
    await tx.wait();
  });
});
```

#### How the tests work

The test file demonstrates key concepts for testing fhEVM smart contracts:

1. **Test setup**:
   - `before`: Initializes test signers (users) that will interact with the contract
   - `beforeEach`: Deploys a fresh instance of the contract before each test
   - Creates FHE instances for each signer to handle encryption/decryption
2. **Test structure**:

   ```ts
   describe("Contract Name", function() {
     // Setup hooks
     before(async function() { ... })
     beforeEach(async function() { ... })

     // Individual test cases
     it("should do something", async function() { ... })
   });
   ```

3. **Key components**:
   - `createInstance()`: Sets up FHE instances for each signer to handle encrypted operations
   - `getSigners()`: Provides test accounts to interact with the contract
   - `contractFactory.deploy()`: Creates a new contract instance for testing
   - `tx.wait()`: Ensures transactions are mined before continuing

## Best practices

### General best practices

- Deploy fresh contract instances for each test to ensure isolation
- Use descriptive test names that explain the expected behavior
- Handle asynchronous operations properly with async/await
- Set up proper encryption instances for testing encrypted values

### No constant nor immutable encrypted state variables

Never use encrypted types for constant or immutable state variables, even if they should actually stay constants, or else any transaction involving those will fail. This is because ciphertexts should always be stored in the privileged storage of the contract (see paragraph 4.4 of [whitepaper](../../fhevm-whitepaper.pdf)) while constant and immutable variables are just appended to the bytecode of the deployed contract at construction time.

❌ So, even if `a` and `b` should never change after construction, the following example :

```solidity
contract C {
  euint32 internal constant a = TFHE.asEuint32(42);
  euint32 internal immutable b;

  constructor(uint32 _b) {
    b = TFHE.asEuint32(_b);
    TFHE.allowThis(b);
  }
}
```

✅ Should be replaced by this snippet:

```solidity
contract C {
  euint32 internal a = TFHE.asEuint32(42);
  euint32 internal b;

  constructor(uint32 _b) {
    b = TFHE.asEuint32(_b);
    TFHE.allowThis(b);
  }
}
```

## **Next steps**

Congratulations! You’ve configured and written your first confidential smart contract. Here are some ideas to expand your knowledge:

- **Explore advanced configurations**: Customize the `FHEVMConfig` to suit specific encryption requirements.
- **Add functionalities**: Extend the contract by adding decrement functionality or resetting the counter.
- **Integrate frontend**: Learn how to decrypt and display encrypted data in a dApp using the `fhevmjs` library.

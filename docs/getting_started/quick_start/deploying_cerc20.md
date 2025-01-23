# Step 3: Deploying **ConfidentialERC20**

In this tutorial, you'll learn how to deploy a confidential token contract using Zama's **Fully Homomorphic Encryption Virtual Machine (fhEVM)**. We'll create `MyConfidentialERC20.sol` to demonstrate encrypted operations.

## 1. **Set up the contract file**

1. Open the **Remix IDE**.
2. Navigate to the **contracts** folder.
3. Create a new file using the "New File" icon or by right-clicking.
4. Name the file `MyConfidentialERC20.sol` and press Enter.

   ![File Creation](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/506d526f-7e88-4aae-aaa5-92176b03ccf8/stack_animation.webp)

## 2. **Basic contract structure**

The foundational structure includes importing Zama's libraries and connecting to Sepolia's FHEVM configuration.

Copy the following code to Remix:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import "fhevm/config/ZamaFHEVMConfig.sol";

contract MyConfidentialERC20 is SepoliaZamaFHEVMConfig {}
```

Upon saving, **Remix** automatically imports the libraries.

![Auto-import in Remix](https://ajeuwbhvhr.cloudimg.io/colony-recorder.s3.amazonaws.com/files/2025-01-16/98f850d2-b303-4ba7-89e8-9db3fba9773c/ascreenshot.jpeg)

### **Key components**

#### **`TFHE.sol`**

Provides Zama’s encrypted data types like `euint64` and secure operations such as addition and comparison.

#### **`SepoliaZamaFHEVMConfig`**

Links the contract to Sepolia's FHEVM coprocessor for real-time encrypted operations.

## 3. **Compiling the contract**

1. Go to **Solidity Compiler** in Remix.
2. Select `MyConfidentialERC20.sol`.
3. Click **Compile** and ensure there are no errors.

   ![Compile Contract](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/a4776697-ea82-4094-8e36-95f377b271d3/stack_animation.webp)

## 4. **Enhancing functionality**

We’ll now add minting capabilities to create an encrypted ERC-20 token.

Copy the following code to Remix:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import "fhevm/config/ZamaFHEVMConfig.sol";
import "fhevm-contracts/contracts/token/ERC20/extensions/ConfidentialERC20Mintable.sol";

contract MyConfidentialERC20 is SepoliaZamaFHEVMConfig, ConfidentialERC20Mintable {
  constructor(string memory name_, string memory symbol_) ConfidentialERC20Mintable(name_, symbol_, msg.sender) {}
}
```

![Mintable Token Implementation](https://ajeuwbhvhr.cloudimg.io/colony-recorder.s3.amazonaws.com/files/2025-01-16/1aba4b08-182d-40df-b2da-dfa9a51ffd29/ascreenshot.jpeg)

## What is `ConfidentialERC20Mintable`?

The **`ConfidentialERC20Mintable`** contract, part of the **`fhevm-contracts`** library, extends the ConfidentialERC20 by adding **minting capabilities**.

### **Key Features**

- **All ConfidentialERC20 functions**: Since the `ConfidentialERC20Mintable` is extending the `ConfidentialERC20` contract, it has all the functionalities that the `ConfidentialERC20` contract has.
- **Minting**: The contract owner can securely create new tokens and distribute them.

### What are `fhevm-contracts`?

The `fhevm-contracts` library is a collection of **privacy-preserving smart contracts** built specifically for the **fhEVM** (Fully Homomorphic Encryption Virtual Machine). It provides:

- **Ready-to-use confidential contracts**: Pre-built implementations of common token standards with FHE capabilities
- **Base contracts**: Foundational building blocks for creating custom confidential smart contracts
- **Extensions**: Additional features and utilities that can be added to base contracts
- **Testing utilities**: Tools to help test FHE-enabled smart contracts

The library serves as both a reference implementation and a toolkit for developers building privacy-focused applications on the fhEVM. It demonstrates best practices for implementing confidential operations while maintaining compatibility with existing Ethereum standards.

## 5. **Deployment**

### **Setup deployment**

1. Navigate to **Deploy & Run Transactions** in Remix.
2. Select "Injected Provider - MetaMask" under **Environment**.
3. Connect your MetaMask wallet to the Sepolia testnet.

   ![Setup MetaMask](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/6bbcfe82-5db4-4a6a-b58e-c21c7e0a5034/stack_animation.webp)

### **Deploy the contract**

1. Expand the **Deploy** section.
2. Fill the constructor parameters:
   - **Name**: Your token’s name (e.g., "My Private Token").
   - **Symbol**: Token symbol (e.g., "MPT").
3. Click **Deploy** and confirm the transaction in MetaMask.

   ![Deploy Contract](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/ad5f896e-a394-449e-bd6f-be37fff251a6/stack_animation.webp)

### **Post-deployment**

- Your contract address will appear under **Deployed Contracts**. Use this interface to interact with the functions.

  ![Deploy Contract](https://ajeuwbhvhr.cloudimg.io/colony-recorder.s3.amazonaws.com/files/2025-01-16/3685296f-0a0a-46bd-9c2f-4cb9320e47d3/ascreenshot.jpeg?tl_px=0,752&br_px=1719,1714&force_format=jpeg&q=100&width=1120.0&wat=1&wat_opacity=1&wat_gravity=northwest&wat_url=https://colony-recorder.s3.amazonaws.com/images/watermarks/FB923C_standard.png&wat_pad=66,466)

- View your contract on Etherscan by clicking the contract address.

  ![Deploy Contract](https://ajeuwbhvhr.cloudimg.io/colony-recorder.s3.amazonaws.com/files/2025-01-16/bb6ad6b9-f166-4fc1-9358-15363592ad46/ascreenshot.jpeg?tl_px=64,752&br_px=1784,1714&force_format=jpeg&q=100&width=1120.0&wat=1&wat_opacity=1&wat_gravity=northwest&wat_url=https://colony-recorder.s3.amazonaws.com/images/watermarks/FB923C_standard.png&wat_pad=524,392)

---

By following these steps, you’ve successfully created and deployed an encrypted ERC-20 token using Zama's fhEVM! 🎉

# 3. Deploying ConfidentialERC20

In this tutorial, you'll learn how to deploy a confidential token contract using Zama's **fhEVM**. We'll create `MyConfidentialERC20.sol` to demonstrate the essential features.

## Prerequisites

Ensure the following before deploying the smart contract:

- The **Zama Plugin** installed is installed in the Remix IDE(see [Step 1](remix.md)).
- Your **wallet** is connected to the **Sepolia testnet**(see [Step 2](connect_wallet.md)).

## Step 1. Setting up the contract file

First, let's create a file for our confidential ERC20 contract:

1. Open the **File explorer** from the side menu.
2. Navigate to the **contracts** folder.
3. Click the **Create new file** icon.
4. Name the file `MyConfidentialERC20.sol` and press Enter.

{% embed url="https://scribehow.com/embed/31__Wg2FlRX2T-WPJ1qPnLuObA?removeLogo=true&skipIntro=true" %}

## Step 2. Writing contract

### Step 2.1 Basic contract structure

The foundational structure includes importing Zama's libraries and connecting to Sepolia's fhEVM configuration.

Copy the following code in the `MyConfidentialERC20.sol` that you just created:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import "fhevm/config/ZamaFHEVMConfig.sol";

contract MyConfidentialERC20 is SepoliaZamaFHEVMConfig {}
```

It should appear as follows:

![](https://ajeuwbhvhr.cloudimg.io/colony-recorder.s3.amazonaws.com/files/2025-01-16/98f850d2-b303-4ba7-89e8-9db3fba9773c/ascreenshot.jpeg)

Remix automatically saves any changes as you type. Upon saving, it imports the following libraries:

- **`TFHE.sol`**: The core Solidity library of Zama's fhEVM. It enables encrypted data type like `euint64`, secures encrypted operations, such as addition and comparison and allows access control.
- **`SepoliaZamaFHEVMConfig`**: A configuration contract that automatically sets up the required configurations for real-time encrypted operations on the Sepolia testnet.

### Step 2.2 Enhancing the functionality

Next, we'll enhance our contract by importing the `fhevm-contracts` library.

{% hint style="info" %}
The **fhevm-contracts** is a Solidity library designed for developers to easily develop confidential smart contracts using fhEVM. It provides:

- **Ready-to-use confidential contracts**: Pre-built implementations of common token standards with FHE capabilities
- **Base contracts**: Foundational building blocks for creating custom confidential smart contracts
- **Extensions**: Additional features and utilities that can be added to base contracts
- **Testing utilities**: Tools to help test FHE-enabled smart contracts

See more details in [the fhEVM-contracts documentation](../../../smart_contracts/contracts.md).
{% endhint %}

The `fhevm-contracts` library includes the `ConfidentialERC20Mintable` contract, which is an extention of `ConfidentialERC20` with minting capabilities, providing:

- Private token transfers and encrypted balances
- Minting functionality for authorized addresses
- Full ERC20 compatibility

It inherits all base `ConfidentialERC20` features while adding secure token creation and distribution capabilities.

To use `ConfidentialERC20Mintable` contract, simply update your `MyConfidentialERC20.sol` with the following code:

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

It should appear as follows:

![](https://ajeuwbhvhr.cloudimg.io/colony-recorder.s3.amazonaws.com/files/2025-01-28/8e41246a-5041-4b29-914f-5e5442b45877/ascreenshot.jpeg?tl_px=0,0&br_px=2752,1538&force_format=jpeg&q=100&width=1120.0&wat=1&wat_opacity=0.7&wat_gravity=northwest&wat_url=https://colony-recorder.s3.us-west-1.amazonaws.com/images/watermarks/FB923C_standard.png&wat_pad=356,140)

## Step 3. Compiling the contract

Now the contract is ready, the next step is to compile it:

1. Select `MyConfidentialERC20.sol`.
2. Go to **Solidity compiler** in Remix.
3. Click **Compile**.

If successful, you will see a green checkmark on the Solidity Compiler, indicating "Compilation successful"

{% embed url="https://scribehow.com/embed/33__ScX1aJqhRMy3nnLhezsKDw?removeLogo=true&skipIntro=true" %}

## Step 4. Deploying the contract

Now the contract is ready to be deployed:

1. Make sure that the envrionment is set up properly
   1. **Envrionment:** Injected Provider - Metamask
   2. **Account:** Your wallet address
2. Expand the **Deploy** section.
3. Fill the constructor parameters:
   - **Name**: Your token’s name (e.g., "My Private Token").
   - **Symbol**: Token symbol (e.g., "MPT").
4. Click **Transact** and confirm the transaction in MetaMask.

Once successfully deployed, your contract will appear under **Deployed Contracts**. You can also view your contract on Etherscan by clicking the contract address.

{% embed url="https://scribehow.com/embed/34__FKpOfgAWTzKX_e9VlakvHA?removeLogo=true&skipIntro=true" %}

---

By following these steps, you’ve successfully created and deployed an confidential ERC-20 token using Zama's fhEVM!🎉 Let's see how the transaction works in the next chapter.

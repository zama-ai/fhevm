# FHEVM v0.9 Migration Guide

The FHEVM v0.9 release introduces significant architectural changes, primarily by removing the dependency on the Zama Oracle service and consolidating network configuration. This guide outlines the essential steps to successfully migrate your dApp to FHEVM v0.9.

## ✅ Migration Checklist

Here is a brief, ordered list of the steps required to successfully migrate your project to FHEVM v0.9:

1.  **Update Dependencies:** Upgrade all key Zama FHE packages to their **FHEVM v0.9 versions**.
2.  **Update Solidity Config:** Replace the removed `SepoliaConfig` with the unified **`EthereumConfig`**.
3.  **Update Solidity Code:** Remove all calls to the discontinued Oracle-based FHE library functions.
4.  **Re-compile & Re-deploy:** Due to new FHEVM addresses, all affected contracts must be re-compiled and re-deployed on Sepolia.
5.  **Rewrite Public Decryption Logic:** Eliminate reliance on the discontinued Zama Oracle and implement the **self-relaying** workflow using the `@zama-fhe/relayer-sdk` and `FHE.verifySignatures()`.

Follow these steps for a smooth transition to FHEVM v0.9:

## Step 1: Update Core Dependencies

Ensure your project uses the latest versions of the FHEVM development tools.

| Dependency              | Minimum Required Version | Notes                                                                 |
| :---------------------- | :----------------------- | :-------------------------------------------------------------------- |
| `@fhevm/solidity`       | `v0.9.0`                 | Contains the updated FHE library contracts.                           |
| `@zama-fhe/relayer-sdk` | `v0.3.0-5`               | **Crucial for v0.9:** Enables the new self-relaying decryption model. |
| `@fhevm/hardhat-plugin` | `v0.3.0-0`               | Latest tooling support for development and deployment.                |

## Step 2: Update Network Configuration in Solidity

The Solidity contracts now use a unified configuration contract defined in `@fhevm/solidity/config/ZamaConfig.sol`.

- **⚠️ Removal:** The `SepoliaConfig` contract is now **removed**.
- **✅ New Standard:** Update your imports and usages to use the new standard **`EthereumConfig`** contract. This change simplifies future cross-chain compatibility.

The new `EthereumConfig` abstract contract now dynamically resolves the FHEVM host addresses according to the `block.chainid`.

Replace:

```solidity
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";
```

With:

```solidity
import { EthereumConfig } from "@fhevm/solidity/config/ZamaConfig.sol";
```

You can read more about [Configuration on the dedicated page](configure.md).

## Step 3: Update Solidity Code

The Zama public decryption Oracle is discontinued. The following functions are no more available in the FHE Solidity library:

- `FHE.loadRequestedHandles`
- `FHE.requestDecryptionWithoutSavingHandles`
- `FHE.requestDecryption`
- `FHE.checkSignatures`

## Step 4: Re-compile and Re-deploy Smart Contracts

Due to fundamental changes in the FHEVM implementation and underlying infrastructure:

- **New FHEVM Addresses:** The contract addresses for core FHE components have changed.
- **Action:** You **must** re-compile your entire Solidity codebase and re-deploy all affected contracts to the **Sepolia** network.

## Step 5: Adjust Public Decryption Logic (Crucial Architectural Change)

The most significant change is the discontinuation of the Zama Oracle. This requires substantial adjustments to how your dApp handles decryption on-chain.

| Aspect                 | FHEVM v0.8 (Old Logic)                                                | FHEVM v0.9 (New Logic)                                                               |
| :--------------------- | :-------------------------------------------------------------------- | :----------------------------------------------------------------------------------- |
| **Decryption Handler** | **Zama Oracle** actively listens for requests and submits the result. | **dApp Client/User** performs the off-chain decryption (self-relaying).              |
| **Solidity Function**  | Used `FHE.requestDecryption()`.                                       | You will now create custom functions that accept the decrypted value and the proof.  |
| **Client-Side Tool**   | N/A                                                                   | **Use `@zama-fhe/relayer-sdk`** to perform the `publicDecrypt` and obtain the proof. |

> **Action:** Thoroughly review your Solidity code, dApp logic, and backend services. Any code relying on the external Oracle must be rewritten to implement the self-relaying workflow using the `@zama-fhe/relayer-sdk`.

# 📖 FHEVM Public Decryption Workflow: From Oracle-Relaying (FHEVM v0.8) to Self-Relaying (FHEVM v0.9)

This documentation outlines the architectural shift in the FHEVM Public decryption workflow, highlighting the change in responsibility from an external Oracle to the dApp's off-chain logic.

---

## 1. FHEVM v0.8 Oracle-Based Decryption

In FHEVM v0.8, the decryption process relies on a trusted **Oracle** to relay the decryption request and proof between the dApp and the Zama Key Management System (KMS). This approach abstracts the complexity but introduces an external dependency.

### Decryption Steps

| Step   | Component                    | Action                                                                                                           |
| :----- | :--------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| **1.** | **dApp (Solidity)**          | Calls `FHE.requestDecryption()` to signal a need for clear data.                                                 |
| **2.** | **Oracle**                   | Listens for the on-chain decryption request event.                                                               |
| **3.** | **Oracle (Off-chain)**       | Performs the `publicDecryption` with the Zama KMS, retrieving the **clear values** and the **decryption proof**. |
| **4.** | **Oracle**                   | Calls the user-specified dApp **callback Solidity function** with the clear values and the associated proof.     |
| **5.** | **dApp (Solidity Callback)** | Calls `FHE.checkSignatures()` to verify the authenticity of the clear values using the provided proof.           |

> **Key takeaway for v8:** The Oracle is the trusted intermediary responsible for performing the off-chain decryption and submitting the result back to the dApp contract.

---

## 2. FHEVM v0.9 Self-Relaying Decryption & dApp Responsibility

The FHEVM v0.9 architecture shifts to a **self-relaying model**, empowering the dApp client (the user) to execute the off-chain decryption and re-submission. This decentralizes the process and removes the dependency on a general-purpose Oracle.

### Example Scenario: Checking a Player's Encrypted Score

Consider a **Game contract** where Alice's final score is stored encrypted on-chain. Alice needs to prove her clear score to claim a reward.

| Step   | Component                    | Action                                                                                                                                                                                  |
| :----- | :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1.** | **Game Contract (Solidity)** | An on-chain function is called to make Alice's encrypted score **publicly decryptable**.                                                                                                |
| **2.** | **Alice (Client/Off-chain)** | Alice fetches the publicly decryptable encrypted score from the Game contract.                                                                                                          |
| **3.** | **Alice (Client/Off-chain)** | Alice or any third-party service uses the **`@zama-fhe/relayer-sdk`** to call the off-chain `publicDecrypt` function. This returns the clear score value and a **proof of decryption**. |
| **4.** | **Alice (Client/On-chain)**  | Alice calls a function on the **Game contract** with the decrypted clear score and the proof.                                                                                           |
| **5.** | **Game Contract (Solidity)** | The contract calls `FHE.checkSignatures()` to **verify the score's validity** using the provided proof.                                                                                 |
| **6.** | **Game Contract (Solidity)** | If the score is valid, the contract executes the game logic (e.g., distributing Alice's prize).                                                                                         |

> **Key takeaway for FHEVM v0.9:** Decryption is a **user-driven, off-chain process**. The dApp client is responsible for off-chain decryption, fetching the proof, and relaying the result back on-chain for verification.

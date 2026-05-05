# User decryption delegation

This document explains how to delegate user decryption rights in FHEVM. Delegation allows a user to authorize another address (such as a backend service, relayer, or smart account) to decrypt encrypted values on their behalf for specific contracts.

## Why use delegation?

In many dApp architectures, the end user does not directly interact with the decryption process. Instead, a backend service or relayer handles it. Delegation enables this pattern securely:

- A user grants a delegate the right to decrypt their values for a specific contract.
- The delegate can then perform user decryptions without the user being online.
- Delegations can be time-limited or indefinite.
- Users can revoke delegations at any time.

## Granting delegation

### Single contract delegation

```solidity
// Delegate with an expiration date (Unix timestamp)
FHE.delegateUserDecryption(delegate, contractAddress, expirationDate);

// Delegate without expiration
FHE.delegateUserDecryptionWithoutExpiration(delegate, contractAddress);
```

**Parameters**

| Parameter         | Type      | Description                                                                 |
| ----------------- | --------- | --------------------------------------------------------------------------- |
| `delegate`        | `address` | The address receiving decryption rights. Cannot be the contract address.    |
| `contractAddress` | `address` | The contract whose ciphertexts the delegate can decrypt on the user's behalf. |
| `expirationDate`  | `uint64`  | Unix timestamp after which the delegation expires.                          |

### Batch delegation

To delegate decryption rights across multiple contracts in a single call:

```solidity
address[] memory contracts = new address[](2);
contracts[0] = address(contractA);
contracts[1] = address(contractB);

// With expiration
FHE.delegateUserDecryptions(delegate, contracts, expirationDate);

// Without expiration
FHE.delegateUserDecryptionsWithoutExpiration(delegate, contracts);
```

## Revoking delegation

```solidity
// Revoke for a single contract
FHE.revokeUserDecryptionDelegation(delegate, contractAddress);

// Revoke for multiple contracts
FHE.revokeUserDecryptionDelegations(delegate, contractAddresses);
```

## Checking delegation status

### Check if a handle is user-decryptable

```solidity
bool canDecrypt = FHE.isUserDecryptable(handle, user, contractAddress);
```

Returns `true` if both the `user` and the `contractAddress` have persistent ACL permission on the handle. Returns `false` if `user == contractAddress`.

### Check if delegation is active

```solidity
bool isActive = FHE.isDelegatedForUserDecryption(delegator, delegate, contractAddress, handle);
```

Returns `true` if `delegate` has an active (non-expired) delegation from `delegator` for the given handle and contract.

### Get delegation expiration date

```solidity
uint64 expiration = FHE.getDelegatedUserDecryptionExpirationDate(delegator, delegate, contractAddress);
```

Returns the expiration timestamp. Returns `0` if no delegation exists.

## Example: delegating to a backend relayer

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import { ZamaEthereumConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract ConfidentialVault is ZamaEthereumConfig {
    mapping(address => euint64) private balances;

    function deposit(externalEuint64 encryptedAmount, bytes calldata inputProof) external {
        euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);
        balances[msg.sender] = FHE.add(balances[msg.sender], amount);
        FHE.allowThis(balances[msg.sender]);
        FHE.allow(balances[msg.sender], msg.sender);
    }

    /// @notice Allow a relayer to decrypt your balance on your behalf
    function authorizeRelayer(address relayer, uint64 expiration) external {
        FHE.delegateUserDecryption(relayer, address(this), expiration);
    }

    /// @notice Revoke a relayer's decryption rights
    function revokeRelayer(address relayer) external {
        FHE.revokeUserDecryptionDelegation(relayer, address(this));
    }
}
```

In this example, a user can authorize a relayer service to decrypt their vault balance. The relayer can then fetch the decrypted balance on behalf of the user without the user needing to be online. The delegation expires at the specified timestamp, and the user can revoke access at any time.

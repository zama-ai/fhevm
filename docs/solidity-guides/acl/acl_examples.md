# ACL examples

This page provides detailed instructions and examples on how to use and implement the ACL (Access Control List) in FHEVM. For an overview of ACL concepts and their importance, refer to the [access control list (ACL) overview](./).

## Controlling access: permanent and transient allowances

The ACL system allows you to define two types of permissions for accessing ciphertexts:

### Permanent allowance

- **Function**: `FHE.allow(ciphertext, address)`
- **Purpose**: Grants persistent access to a ciphertext for a specific address.
- **Storage**: Permissions are saved in a dedicated ACL contract, making them available across transactions.

#### Alternative Solidity syntax

You can also use method-chaining syntax for granting allowances since FHE is a Solidity library.

```solidity
using FHE for *;
ciphertext.allow(address1).allow(address2);
```

This is equivalent to calling `FHE.allow(ciphertext, address1)` followed by `FHE.allow(ciphertext, address2)`.

### Transient allowance

- **Function**: `FHE.allowTransient(ciphertext, address)`
- **Purpose**: Grants temporary access for the duration of a single transaction.
- **Storage**: Permissions are stored in transient storage to save gas costs.
- **Use Case**: Ideal for passing encrypted values between functions or contracts during a transaction.

#### Alternative Solidity syntax

Method chaining is also available for transient allowances since FHE is a Solidity library.

```solidity
using FHE for *;
ciphertext.allowTransient(address1).allowTransient(address2);
```

### Syntactic sugar

- **Function**: `FHE.allowThis(ciphertext)`
- **Equivalent To**: `FHE.allow(ciphertext, address(this))`
- **Purpose**: Simplifies granting permanent access to the current contract for managing ciphertexts.

#### Alternative Solidity syntax

You can also use method-chaining syntax for allowThis since FHE is a Solidity library.

```solidity
using FHE for *;
ciphertext.allowThis();
```

#### Make publicly decryptable

To make a ciphertext publicly decryptable, you can use the `FHE.makePubliclyDecryptable(ciphertext)` function. This grants decryption rights to anyone, which is useful for scenarios where the encrypted value should be accessible by all.

```solidity
// Grant public decryption right to a ciphertext
FHE.makePubliclyDecryptable(ciphertext);

// Or using method syntax:
ciphertext.makePubliclyDecryptable();
```

- **Function**: `FHE.makePubliclyDecryptable(ciphertext)`
- **Purpose**: Makes the ciphertext decryptable by anyone.
- **Use Case**: When you want to publish encrypted results or data.

> You can combine multiple allowance methods (such as `.allow()`, `.allowThis()`, `.allowTransient()`) directly on ciphertext objects to grant access to several addresses or contracts in a single, fluent statement.
>
> **Example**
>
> ```solidity
> // Grant transient access to one address and permanent access to another address
> ciphertext.allowTransient(address1).allow(address2);
>
> // Grant permanent access to the current contract and another address
> ciphertext.allowThis().allow(address1);
> ```

## Best practices

### Verifying sender access

When processing ciphertexts as input, it’s essential to validate that the sender is authorized to interact with the provided encrypted data. Failing to perform this verification can expose the system to inference attacks where malicious actors attempt to deduce private information.

#### Example scenario: Confidential ERC20 attack

Suppose a confidential ERC20 token has a `transfer(address to, euint64 encryptedAmount)` function that does **not** call `FHE.isSenderAllowed(encryptedAmount)`. The contract trusts whatever encrypted amount the caller passes in.

An attacker controls two accounts they own — **Account A** (funded with 100 tokens) and **Account B** — and wants to learn the balance of a victim **Account V** without ever decrypting it.

The attack:

1. The victim's encrypted balance handle is publicly readable on-chain (it lives in the contract's `balances[V]` storage). The attacker reads that handle.
2. The attacker calls `transfer(B, balances[V])` from Account A — passing the **victim's** balance handle as the `encryptedAmount`. Without `isSenderAllowed`, the contract has no way to know the attacker did not produce that handle.
3. Inside `transfer`, the contract executes `canTransfer = FHE.le(encryptedAmount, balances[A])` and conditionally moves the amount via `FHE.select`. Whether the transfer ends up actually moving tokens depends on whether `balance[V] <= 100`.
4. The attacker reads `balances[B]` after the transaction. The new handle either reflects an increase (transfer happened ⇒ `balance[V] <= 100`) or stays the same (transfer skipped ⇒ `balance[V] > 100`).

Each successful or failed transfer leaks one bit about the victim's balance. By repeating the attack with progressively different sender balances, the attacker can binary-search the victim's exact balance — all without ever obtaining a decryption.

The fix is one line: require `FHE.isSenderAllowed(encryptedAmount)` so the contract only accepts handles the sender is genuinely authorized to use.

---

#### Example: secure verification

```solidity
function transfer(address to, euint64 encryptedAmount) public {
  // Ensure the sender is authorized to access the encrypted amount
  require(FHE.isSenderAllowed(encryptedAmount), "Unauthorized access to encrypted amount.");

  // Proceed with further logic
  ...
}
```

By enforcing this check, you can safeguard against inference attacks and ensure that encrypted values are only manipulated by authorized entities.

## ACL for user decryption

If a ciphertext can be decrypted by a user, explicit access must be granted to them. Additionally, the user decryption mechanism requires the signature of a public key associated with the contract address. Therefore, a value that needs to be decrypted must be explicitly authorized for both the user and the contract.

Due to the user decryption mechanism, a user signs a public key associated with a specific contract; therefore, the ciphertext also needs to be allowed for the contract.

### Example: Secure Transfer in ConfidentialERC20

```solidity
function transfer(address to, euint64 encryptedAmount) public {
  require(FHE.isSenderAllowed(encryptedAmount), "The caller is not authorized to access this encrypted amount.");
  euint64 amount = FHE.asEuint64(encryptedAmount);
  ebool canTransfer = FHE.le(amount, balances[msg.sender]);

  euint64 newBalanceTo = FHE.add(balances[to], FHE.select(canTransfer, amount, FHE.asEuint64(0)));
  balances[to] = newBalanceTo;
  // Allow this new balance for both the contract and the owner.
  FHE.allowThis(newBalanceTo);
  FHE.allow(newBalanceTo, to);

  euint64 newBalanceFrom = FHE.sub(balances[from], FHE.select(canTransfer, amount, FHE.asEuint64(0)));
  balances[from] = newBalanceFrom;
  // Allow this new balance for both the contract and the owner.
  FHE.allowThis(newBalanceFrom);
  FHE.allow(newBalanceFrom, from);
}
```

By understanding how to grant and verify permissions, you can effectively manage access to encrypted data in your FHEVM smart contracts. For additional context, see the [ACL overview](./).

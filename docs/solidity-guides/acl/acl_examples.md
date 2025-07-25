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

Consider an **Confidential ERC20 token**. An attacker controlling two accounts, **Account A** and **Account B**, with 100 tokens in Account A, could exploit the system as follows:

1. The attacker attempts to send the target user's encrypted balance from **Account A** to **Account B**.
2. Observing the transaction outcome, the attacker gains information:
   - **If successful**: The target's balance is equal to or less than 100 tokens.
   - **If failed**: The target's balance exceeds 100 tokens.

This type of attack allows the attacker to infer private balances without explicit access.

To prevent this, always use the `FHE.isSenderAllowed()` function to verify that the sender has legitimate access to the encrypted amount being transferred.

---

#### Example: secure verification

```solidity
function transfer(address to, euint64 encryptedAmount, bytes calldata inputProof) public {
  // Ensure the sender is authorized to access the encrypted amount
  require(FHE.isSenderAllowed(encryptedAmount), "Unauthorized access to encrypted amount.");

  // Proceed with further logic
  euint64 amount = FHE.asEuint64(encryptedAmount);
  ...
}
```

By enforcing this check, you can safeguard against inference attacks and ensure that encrypted values are only manipulated by authorized entities.

## ACL for user decryption

If a ciphertext can be decrypt by a user, explicit access must be granted to them. Additionally, the user decryption mechanism requires the signature of a public key associated with the contract address. Therefore, a value that needs to be decrypted must be explicitly authorized for both the user and the contract.

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

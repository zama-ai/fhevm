# ACL examples

This page provides detailed instructions and examples on how to use and implement the ACL (Access Control List) in fhEVM. For an overview of ACL concepts and their importance, refer to the [access control list (ACL) overview](./).

---

## Controlling access: permanent and transient allowances

The ACL system allows you to define two types of permissions for accessing ciphertexts:

### Permanent allowance

- **Function**: `TFHE.allow(ciphertext, address)`
- **Purpose**: Grants persistent access to a ciphertext for a specific address.
- **Storage**: Permissions are saved in a dedicated ACL contract, making them available across transactions.

### Transient allowance

- **Function**: `TFHE.allowTransient(ciphertext, address)`
- **Purpose**: Grants temporary access for the duration of a single transaction.
- **Storage**: Permissions are stored in transient storage to save gas costs.
- **Use Case**: Ideal for passing encrypted values between functions or contracts during a transaction.

### Syntactic sugar

- **Function**: `TFHE.allowThis(ciphertext)`
- **Equivalent To**: `TFHE.allow(ciphertext, address(this))`
- **Purpose**: Simplifies granting permanent access to the current contract for managing ciphertexts.

---

### Example: granting permissions in a multi-contract setup

```solidity
import "fhevm/lib/TFHE.sol";
import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";

contract SecretGiver is SepoliaZamaFHEVMConfig {
  SecretStore public secretStore;

  constructor() {
    secretStore = new SecretStore();
  }

  function giveMySecret() public {
    // Create my secret - asEuint16 gives automatically transient allowance for the resulting handle (note: an onchain trivial encryption is not secret)
    euint16 mySecret = TFHE.asEuint16(42);

    // Allow temporarily the SecretStore contract to manipulate `mySecret`
    TFHE.allowTransient(mySecret, address(secretStore));

    // Call `secretStore` with `mySecret`
    secretStore.storeSecret(mySecret);
  }
}
```

```
contract SecretStore is SepoliaZamaFHEVMConfig {
  euint16 public secretResult;

  function storeSecret(euint16 callerSecret) public {
    // Verify that the caller has also access to this ciphertext
    require(TFHE.isSenderAllowed(callerSecret), "The caller is not authorized to access this secret.");

    // do some FHE computation (result is automatically put in the ACL transient storage)
    euint16 computationResult = TFHE.add(callerSecret, 3);

    // then store the resulting ciphertext handle in the contract storage
    secretResult = computationResult;

    // Make the temporary allowance for this ciphertext permanent to let the contract able to reuse it at a later stage or request a decryption of it
    TFHE.allowThis(secretResult); // this is strictly equivalent to `TFHE.allow(secretResult, address(this));``
  }
}
```

---

## Automatic transient allowance

Some functions automatically grant transient allowances to the calling contract, simplifying workflow. These include:

- **Type Conversion**:
  - `TFHE.asEuintXX()`, `TFHE.asEbool()`, `TFHE.asEaddress()`
- **Random Value Generation**:
  - `TFHE.randXX()`
- **Computation Results**:
  - `TFHE.add()`, `TFHE.select()`

### Example: random value generation

```solidity
function randomize() public {
  // Generate a random encrypted value with transient allowance
  euint64 random = TFHE.randEuint64();

  // Convert the transient allowance into a permanent one
  TFHE.allowThis(random);
}
```

---

## 🔧 Best practices

### Verifying sender access

When processing ciphertexts as input, it’s essential to validate that the sender is authorized to interact with the provided encrypted data. Failing to perform this verification can expose the system to inference attacks where malicious actors attempt to deduce private information.

#### Example scenario: Encrypted ERC20 attack

Consider an **Encrypted ERC20 token**. An attacker controlling two accounts, **Account A** and **Account B**, with 100 tokens in Account A, could exploit the system as follows:

1. The attacker attempts to send the target user's encrypted balance from **Account A** to **Account B**.
2. Observing the transaction outcome, the attacker gains information:
   - **If successful**: The target's balance is equal to or less than 100 tokens.
   - **If failed**: The target's balance exceeds 100 tokens.

This type of attack allows the attacker to infer private balances without explicit access.

To prevent this, always use the `TFHE.isSenderAllowed()` function to verify that the sender has legitimate access to the encrypted amount being transferred.

---

#### Example: secure verification

```solidity
function transfer(address to, euint64 encryptedAmount, bytes calldata inputProof) public {
  // Ensure the sender is authorized to access the encrypted amount
  require(TFHE.isSenderAllowed(encryptedAmount), "Unauthorized access to encrypted amount.");

  // Proceed with further logic
  euint64 amount = TFHE.asEuint64(encryptedAmount);
  ...
}
```

By enforcing this check, you can safeguard against inference attacks and ensure that encrypted values are only manipulated by authorized entities.

## ACL for reencryption

If a ciphertext can be reencrypted by a user, explicit access must be granted to them. Additionally, the reencryption mechanism requires the signature of a public key associated with the contract address. Therefore, a value that needs to be reencrypted must be explicitly authorized for both the user and the contract.

Due to the reencryption mechanism, a user signs a public key associated with a specific contract; therefore, the ciphertext also needs to be allowed for the contract.

### Example: Secure Transfer in Encrypted ERC-20

```solidity
function transfer(address to, euint64 encryptedAmount) public {
  require(TFHE.isSenderAllowed(encryptedAmount), "The caller is not authorized to access this encrypted amount.");
  euint64 amount = TFHE.asEuint64(encryptedAmount);
  ebool canTransfer = TFHE.le(amount, balances[msg.sender]);

  euint64 newBalanceTo = TFHE.add(balances[to], TFHE.select(canTransfer, amount, TFHE.asEuint64(0)));
  balances[to] = newBalanceTo;
  // Allow this new balance for both the contract and the owner.
  TFHE.allowThis(newBalanceTo);
  TFHE.allow(newBalanceTo, to);

  euint64 newBalanceFrom = TFHE.sub(balances[from], TFHE.select(canTransfer, amount, TFHE.asEuint64(0)));
  balances[from] = newBalanceFrom;
  // Allow this new balance for both the contract and the owner.
  TFHE.allowThis(newBalanceFrom);
  TFHE.allow(newBalanceFrom, from);
}
```

---

By understanding how to grant and verify permissions, you can effectively manage access to encrypted data in your fhEVM smart contracts. For additional context, see the [ACL overview](./).

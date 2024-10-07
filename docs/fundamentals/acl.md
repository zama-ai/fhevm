# Access Control List
This document explains how to prevent unauthorized addresses from accessing the contents of unauthorized ciphertexts.
This is enabled by the Access Control List (ACL) system in fhEVM, which defines which addresses have the right to manipulate the ciphertext.

## How it works?

You can configure the ACLs in two ways:

- **Permanent allowance**: `TFHE.allow(ciphertext, address)` allows a ciphertext to be used by a specific address at any time.
- **Transient allowance**: `TFHE.allowTransient(ciphertext, address)` authorizes access to the ciphertext only for the duration of the transaction.

Additionally, you can use `TFHE.allowThis(ciphertext)` as syntactic sugar for `TFHE.allow(ciphertext, address(this))`. This function is commonly used within dApp smart contracts to authorize the same contract to reuse a newly computed ciphertext handle in a future transaction.

Permanent allowance will store the ACL in a dedicated contract, while a temporary allowance will store it in [transient storage](https://eips.ethereum.org/EIPS/eip-1153), allowing developers to save gas. Transient allowance is particularly useful when calling an external function using a ciphertext as a parameter.

To illustrate, here is a simple example where one function calls another:

```solidity
import "fhevm/lib/TFHE.sol";

contract SecretGiver {
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

contract SecretStore {
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

## Automatic (transient) allowance

To simplify matters, a number of functions automatically generate temporary access (using `TFHE.allowTransient` under the hood) for the contract that calls the function. This applies to:

- `TFHE.asEuintXX()`, `TFHE.asEaddress()`, `TFHE.asEbool()`
- `TFHE.randXX()`
- All results from computations (`TFHE.add()`, `TFHE.select()`, ...)

```solidity
function randomize() {
  // Store this random value. This value is temporarily allowed.
  random = TFHE.randEuint64();

  // Permanently store the temporary access for this ciphertext.
  TFHE.allowThis(random);
}
```

## Security best practice: isSenderAllowed()

When a function receives a ciphertext (such as `ebool`, `euint8`, `eaddress`, ...), it needs to verify that the sender also has access to this ciphertext. This verification is crucial for security.

Without this check, a contract could send any ciphertext authorized for the contract and potentially exploit the function to retrieve the value. For example, an attacker could transfer someone's balance as an encrypted amount. 

If the function does not include `require(TFHE.isSenderAllowed(encryptedAmount))`, an attacker who doesn't have access to this balance could determine the value by transferring the balance between two well-funded accounts.

## ACL for reencryption

If a ciphertext must be reencrypted by a user, then explicit access must be granted to them. If this authorization is not given, the user will be unable to request a reencryption of this ciphertext. 

Due to the reencryption mechanism, a user signs a public key associated with a specific contract; therefore, the ciphertext also needs to be allowed for the contract.

Let's take, for example, a transfer in an ERC20:

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

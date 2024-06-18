# ACL

## How it works?

The fhEVM includes an ACL (Access Control List) system that allows you to define which addresses have the right to manipulate a ciphertext. This feature prevents any address from accessing the contents of any ciphertext.

These ACLs can be adjusted in two ways:

- `TFHE.allow(ciphertext, address)` Permanently, on the blockchain. This allows a ciphertext to be used by a specific address at any time.
- `TFHE.allowTransient(ciphertext, address)` Temporarily. The ciphertext is then authorized only for the duration of the transaction.

Permanent allowance will store the ACL in a dedicated contract, while a temporary allowance will store it in [transient storage](https://eips.ethereum.org/EIPS/eip-1153), allowing developers to save gas. Transient allowance is particularly useful when calling an external function using a ciphertext as a parameter.

To illustrate, here is a simple example where one function calls another:

```solidity
function giveMySecret() {
  // Create my secret (note: an onchain encryption is not secret)
  euint16 mySecret = TFHE.asEuint16(42);

  // Allow temporary the SecretStore contract to manipulate `mySecret`
  TFHE.allowTransient(mySecret, address(SecretStore));

  // Call `storeSecret` with `mySecret`
  SecretStore.storeSecret(mySecret);
}
```

```solidity
function storeSecret(callerSecret euint16) {
  // Verify that the caller has also access to this ciphertext
  require(TFHE.isSenderAllowed(callerSecret), "The caller is not authorized to access this secret.");

  // Store this ciphertext
  secret = callerSecret;

  // Store the temporary access for this ciphertext permanently
  TFHE.allow(callerSecret, address(this));
}
```

## Automatic (transient) allowance

To simplify matters, a number of functions automatically generate temporary access (using `TFHE.allowTransient`) for the contract that calls the function. This applies to:

- `TFHE.asEuintXX()`, `TFHE.asEaddress()`, `TFHE.asEbool()`
- `TFHE.randXX()`
- All results from computation (`TFHE.add()`, `TFHE.select()`, ...)

```solidity
function randomize() {
  // Store this random value. This value is temporarily allowed.
  random = TFHE.randEuint64();

  // Permanently store the temporary access for this ciphertext.
  TFHE.allow(random, address(this));
}
```

## Security best practice: isSenderAllowed()

When a function receives a ciphertext, it needs to verify that the sender also has access to this ciphertext. This is important because otherwise, a contract could send any ciphertext authorized for the contract and potentially exploit the function to retrieve the value.
For example, an attacker could transfer someone's balance as an encrypted amount. Without `require(TFHE.isSenderAllowed(encryptedAmount))`, an attacker who doesn't have access to this balance could determine the value by transferring the balance between two well-funded accounts.

## ACL for reencryption

If a ciphertext must be reencrypted by a user, then explicit access must be granted to them. If this authorization is not given, the user will be unable to request a reencryption of this ciphertext. Let's take, for example, a transfer in an ERC20:

```solidity
function transfer(address to, euint64 encryptedAmount) public {
  require(TFHE.isSenderAllowed(encryptedAmount), "The caller is not authorized to access this encrypted amount.");
  euint64 amount = TFHE.asEuint64(encryptedAmount);
  ebool canTransfer = TFHE.le(amount, balances[msg.sender]);

  euint64 newBalanceTo = TFHE.add(balances[to], TFHE.select(canTransfer, amount, TFHE.asEuint64(0)));
  balances[to] = newBalanceTo;
  // Allow this new balance for both the contract and the owner.
  TFHE.allow(newBalanceTo, address(this));
  TFHE.allow(newBalanceTo, to);

  euint64 newBalanceFrom = TFHE.sub(balances[from], TFHE.select(canTransfer, amount, TFHE.asEuint64(0)));
  balances[from] = newBalanceFrom;
  // Allow this new balance for both the contract and the owner.
  TFHE.allow(newBalanceFrom, address(this));
  TFHE.allow(newBalanceFrom, from);
}
```

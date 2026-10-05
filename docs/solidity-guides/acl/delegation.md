# Decrypt on behalf of another user

Delegation lets one account (the **delegator**) authorize another account (the **delegate**) to perform user decryption on its behalf, in the context of a specific contract. The ACL stores user decryption permissions as `(user, contractAddress)` pairs; delegation transfers the rights of `(delegator, contractAddress)` to `(delegate, contractAddress)`.

## Who is the delegator?

It depends on which API you call:

| Caller | API | Delegator (`msg.sender` to ACL) |
| --- | --- | --- |
| **EOA** (Externally Owned Account) | `IACL.delegateForUserDecryption` directly on the ACL contract | the EOA itself |
| **Smart contract** | `FHE.delegateUserDecryption` from inside a contract function | `address(this)` |

`FHE.delegateUserDecryption` cannot be used by an EOA to delegate its own rights — the EOA must call the ACL directly.

## Constraints

The ACL enforces four invariants when registering a delegation:

- `msg.sender != contractAddress`
- `msg.sender != delegate`
- `delegate != contractAddress`
- `delegate != WILDCARD_DELEGATION_ADDRESS` (see [Wildcard delegation](#wildcard-delegation))

Plus a one-delegate-or-revoke-per-block rule per `(delegator, delegate, contractAddress)` tuple.

## Pattern 1 — EOA delegates to a backend service

The user calls the ACL contract directly to delegate their own rights:

```solidity
import { IACL } from "@fhevm/solidity/lib/Impl.sol";

IACL(aclAddress).delegateForUserDecryption(relayer, vault, expirationDate);
```

After this, the relayer can user-decrypt any handle that has the `(EOA, vault)` ACL pair.

## Pattern 2 — Contract delegates its own rights

A contract delegates user-decryption rights it has been granted. `contractAddress` must be a **different** contract whose handles this contract has been allowed to access.

```solidity
import { FHE } from "@fhevm/solidity/lib/FHE.sol";
import { ZamaEthereumConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract Aggregator is ZamaEthereumConfig {
    address public immutable vault;

    constructor(address vault_) { vault = vault_; }

    function authorizeRelayer(address relayer, uint64 expirationDate) external {
        FHE.delegateUserDecryption(relayer, vault, expirationDate);
    }

    function revokeRelayer(address relayer) external {
        FHE.revokeUserDecryptionDelegation(relayer, vault);
    }
}
```

{% hint style="warning" %}
**Common mistake:** calling `FHE.delegateUserDecryption(relayer, address(this), expiration)` from inside a contract, hoping to delegate the caller user's rights. This always reverts because `msg.sender == contractAddress` violates one of the constraints listed above. Use Pattern 1 instead — the user must call the ACL directly.
{% endhint %}

## Wildcard delegation

Rather than delegating contract by contract, a delegator can grant a single delegation that covers **every** app contract. To do so, pass the sentinel address `WILDCARD_DELEGATION_ADDRESS` (`address(type(uint160).max)`, i.e. `0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF`) as `contractAddress`:

```solidity
// EOA, calling the ACL directly
address wildcard = IACL(aclAddress).WILDCARD_DELEGATION_ADDRESS();
IACL(aclAddress).delegateForUserDecryption(relayer, wildcard, expirationDate);

// Contract, through the FHE library
FHE.delegateUserDecryption(relayer, address(type(uint160).max), expirationDate);
```

How it behaves:

- **It does not bypass the ACL.** A delegate can only user-decrypt a handle if both the delegator and the handle's app contract are persistently allowed on it. The wildcard only saves you from registering one delegation per contract.
- **It combines with per-contract delegations.** A delegation is active for `(delegator, delegate, contractAddress)` if either the per-contract entry or the wildcard entry has not expired. You can mix the two, for example to give some contracts a longer expiry.
- **It is revoked on its own.** Revoking a per-contract delegation leaves the wildcard in place, and the reverse is also true. To remove the wildcard, call `revokeDelegationForUserDecryption(delegate, WILDCARD_DELEGATION_ADDRESS)` (or `FHE.revokeUserDecryptionDelegation(delegate, address(type(uint160).max))`).
- **Querying it.** `FHE.isDelegatedForUserDecryption(delegator, delegate, contractAddress, handle)` takes the wildcard into account. `FHE.getDelegatedUserDecryptionExpirationDate` returns only the entry you ask for, so pass the wildcard address to read the wildcard's expiry.
- The wildcard address cannot be used as the `delegate` (reverts with `IACL-DelegateCannotBeWildcard`).

{% hint style="danger" %}
A wildcard delegation is a high-trust grant: the delegate can decrypt everything the delegator can, in every app, now and in the future, until the delegation expires or is revoked. Prefer per-contract delegations and short expiration dates. Wallets and SDKs should warn users explicitly before they sign a wildcard delegation.
{% endhint %}

## API summary

```solidity
// Granting (caller-contract side)
FHE.delegateUserDecryption(delegate, contractAddress, expirationDate);
FHE.delegateUserDecryptionWithoutExpiration(delegate, contractAddress);
FHE.delegateUserDecryptions(delegate, contractAddresses, expirationDate);            // batch
FHE.delegateUserDecryptionsWithoutExpiration(delegate, contractAddresses);           // batch

// Revoking
FHE.revokeUserDecryptionDelegation(delegate, contractAddress);
FHE.revokeUserDecryptionDelegations(delegate, contractAddresses);                    // batch

// Querying
FHE.isDelegatedForUserDecryption(delegator, delegate, contractAddress, handle);      // active for handle? (includes wildcard)
FHE.getDelegatedUserDecryptionExpirationDate(delegator, delegate, contractAddress);  // 0 = none, max = permanent (exact entry only)

// Wildcard: pass address(type(uint160).max) as contractAddress to the delegate,
// revoke and getDelegatedUserDecryptionExpirationDate functions
FHE.isUserDecryptable(handle, user, contractAddress);                                // raw ACL check, ignores delegation
```

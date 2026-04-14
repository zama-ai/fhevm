# Allow and user-decryption delegation (host ACL)

This note describes how persistent `allow` interacts with **user-decryption delegation**, including **wildcard delegation** (RFC 017).

## Persistent allow

The host `ACL` contract stores `persistedAllowedPairs[handle][account]`. Typical confidential apps call `allow` so that both the user and the app contract are authorized on a ciphertext handle before decryption or further use.

## Per-contract delegation

A delegator may call `delegateForUserDecryption(delegate, contractAddress, expirationDate)` so that a delegate can request user decryption for handles where:

- the delegator is persistently allowed on the handle,
- the **app** `contractAddress` passed into the view is persistently allowed on the handle, and
- a non-expired delegation exists for `(delegator, delegate, contractAddress)`.

`revokeDelegationForUserDecryption(delegate, contractAddress)` clears that delegation row only.

## Wildcard delegation (`WILDCARD_CONTRACT`)

The ACL exposes `WILDCARD_CONTRACT` — `address(type(uint160).max)` (EIP-55: `0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF`). No contract can be deployed at this address.

Calling `delegateForUserDecryption(delegate, WILDCARD_CONTRACT, expirationDate)` stores a **wildcard** delegation: for any handle where the delegator **and** the app contract (the `contractAddress` argument to `isHandleDelegatedForUserDecryption`) are persistently allowed, the delegate is authorized if **either**

- a non-expired delegation exists for `(delegator, delegate, WILDCARD_CONTRACT)`, or
- a non-expired delegation exists for `(delegator, delegate, appContract)`.

Wildcard delegation does **not** remove the need for normal `allow` edges on the delegator and the app contract; it only removes the need to maintain **separate delegation transactions per app contract**.

Mixing wildcard with per-contract delegations is allowed but redundant for the same expiry. Revoking the wildcard row does not revoke unrelated per-contract rows.

## Off-chain verification (KMS Connector, Relayer)

The KMS Connector and Relayer continue to use a **single** `isHandleDelegatedForUserDecryption(delegator, delegate, contractFromRequest, handle)` call per handle; wildcard semantics are enforced entirely in the ACL. No connector change is required for RFC 017.

SDKs should treat wildcard delegation as a **high-trust** grant and warn users explicitly when building transactions or signing flows that use `WILDCARD_CONTRACT`.

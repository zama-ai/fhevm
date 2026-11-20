# Homomorphic Complexity Units ("HCU") in FHEVM

This guide explains how to use Fully Homomorphic Encryption (FHE) operations in your smart contracts on FHEVM. Understanding HCU is critical for designing efficient confidential smart contracts.

## Overview

FHE operations in FHEVM are computationally intensive compared to standard Ethereum operations, as they require complex mathematical computations to maintain privacy and security. To manage computational load and prevent potential denial-of-service attacks, FHEVM implements a metering system called **Homomorphic Complexity Units ("HCU")**.

To represent this complexity, we introduced the **Homomorphic Complexity Unit ("HCU")**. In Solidity, each FHE operation consumes a set amount of HCU based on the operational computational complexity for hardware computation. Since FHE transactions are symbolic, this helps preventing resource exhaustion outside of the blockchain.

To do so, there is a contract named `HCULimit`, which monitors HCU consumption and enforces these limits:

- **Sequential homomorphic operations depth limit per transaction**: Controls HCU usage for operations that must be processed in order. Exceeding it reverts with `HCUTransactionDepthLimitExceeded`.
- **Global homomorphic operations complexity per transaction**: Controls HCU usage for operations that can be processed in parallel. Exceeding it reverts with `HCUTransactionLimitExceeded`.
- **HCU quota**: Limits the HCU that the contracts sharing a quota can use at once (a maximum) and on average (a decay per block). Exceeding it reverts with `HCUQuotaExceeded`; a stopped quota reverts with `HCUQuotaStopped`, and a paused quota with `HCUQuotaPaused`.
- **Sequential homomorphic operations depth across transactions**: Off by default. When enabled, it caps the depth a handle carries from earlier transactions. Exceeding it reverts with `HCUCrossTxDepthLimitExceeded` (see [Depth across transactions](#depth-across-transactions)).

If any of these limits is exceeded, the transaction will revert.

## HCU limit

The current devnet has an HCU limit of **20,000,000** per transaction and an HCU depth limit of **5,000,000** per transaction. Quotas are set on-chain by governance (via `HCUConfigurator.setQuota`); you can read a contract's quota on `HCULimit` with `getContractQuota(contract)`, and that quota's maximum, decay per block and current usage with `getQuota(quotaId)`. If any HCU limit is exceeded, the transaction will revert.

To resolve a per-transaction revert, you must do one of the following:

- Refactor your code to reduce the number of FHE operations in your transaction.
- Split your FHE operations across multiple independent transactions.

### Quotas and transaction estimation

Every contract that calls `FHEVMExecutor` belongs to one quota. Here, the contract is the dApp contract address passed by `FHEVMExecutor`, not the user's EOA. Contracts start in the default quota (`0`). A contract moves to another quota when governance assigns it (`assignQuota`) or when that quota's admin grants it (`grantQuota`).

On a chain where the 0.14 allowlist was active, an allowlisted contract nobody assigned yet computes in quota `1`, shared at the old per-block cap. If it was also on the block HCU whitelist, it computes in quota `2`, unmetered. `getContractQuota` returns that quota, and only governance moves such a contract. A contract sent back to the default quota leaves the allowlist and stays stopped.

A quota has a maximum and a decay per block, like the depth limit across transactions below. Every FHE operation adds its HCU to the quota's usage, usage decreases by the decay every block, and an operation fits while usage stays at or below the maximum. The maximum sets the largest burst; the decay sets the sustained rate. A quota whose maximum is `type(uint48).max` is not metered. A quota's maximum and decay are both 0 (stopped) or both non-zero.

Unlike the per-transaction limits, a quota depends on **other transactions in the same quota**, not just on your own. As a result:

- A transaction that estimates successfully (e.g. via `eth_estimateGas` or a static call) can still revert with `HCUQuotaExceeded` at execution time if other FHE-heavy transactions in the same quota are mined ahead of it.
- This revert is **non-deterministic from the sender's perspective**: it is a function of block-inclusion ordering, not of the transaction's own payload.

If you hit `HCUQuotaStopped(account, quotaId)`, the quota's maximum is 0, often because the contract is still in a stopped default quota. Retrying fails until the contract is moved to another quota or governance resizes this one.

If you hit `HCUQuotaPaused(account, quotaId)`, a pauser paused the quota. Retrying fails until governance unpauses it.

If you hit `HCUQuotaExceeded(account, quotaId, retryAfterBlocks)`, `retryAfterBlocks` is the wait for the operation that failed, as for the depth limit: wait at least that many blocks, simulate again, and repeat until the simulation passes. It is `type(uint256).max` when one operation alone exceeds the quota's maximum; then retrying fails until the maximum is raised.

The checks run in this order: `HCUQuotaPaused`, `HCUQuotaStopped`, `HCUQuotaExceeded`, then the per-transaction limits and the depth across transactions.

Options include:

- **Retry** the transaction after `retryAfterBlocks`, when the quota has room again.
- **Reduce the HCU footprint** of the transaction so it is more likely to fit alongside others (same techniques as for per-transaction reverts).
- **Ask for your contract to join a quota**: your partner's quota admin can grant it, or governance can assign one for contracts that need more HCU.

A stopped or paused quota does not gate input verification (`FHEVMExecutor.verifyInput`), ACL permission grants, or decryption. A contract in it can still use them; it cannot perform HCU-charged FHE computation. Only a quota's stored-depth limit (see [Depth across transactions](#depth-across-transactions)) can refuse a grant.

### Quota roles

Three roles manage quotas. They call `HCUConfigurator`, which checks each change and passes it to `HCULimit`. Governance is the ACL owner, and pausers are the ACL's pausers.

| Role        | Functions                                                                                          | What it does                                                                                                                                                           |
| ----------- | -------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Governance  | `setQuota`, `setQuotaAdmin`, `assignQuota`, `pauseQuota`, `unpauseQuota`, `setQuotaMaxStoredDepth` | Sizes every quota, names each quota's admin, moves any contract to the default quota or to a quota that is not stopped, pauses and unpauses, sets stored-depth limits. |
| Quota admin | `grantQuota(quotaId, contract)`, `revokeQuota(quotaId, contract)`                                  | A partner's address: moves contracts from the default quota into its quota, and any contract of its quota back to the default quota, without a governance proposal.    |
| Pauser      | `pauseQuota(quotaId)`                                                                              | Stops every contract in a quota. The quota keeps its size and contracts, and its admin cannot move them until governance unpauses it.                                  |

A quota admin can add contracts only while the default quota is stopped (`DefaultQuotaNotStopped` otherwise), so joining a quota never leaves a contract with less room than it had. If governance later reopens the default quota, granted contracts stay in the admin's quota until the admin revokes them or governance moves them. It can send any contract of its quota back to the default quota, including one governance assigned. It cannot resize its quota or take a contract from another quota. Quotas `1` and `2` take no admin. You can read a quota's admin with `HCUConfigurator.getQuotaAdmin(quotaId)` and whether it is paused with `HCULimit.isQuotaPaused(quotaId)`.

### Stopping the default quota

Stopping the default quota (`setQuota(0, 0, 0)`) refuses computation to every contract nobody moved to another quota. It has no on-chain readiness check. Before stopping it, governance should:

1. Inventory **every live dApp contract address** that must keep computing on the chain, including contracts that compute in their constructor (grant their address before deployment).
2. Move each one to a quota with `assignQuota`, or have its quota admin grant it once the default quota is stopped.
3. Check `getContractQuota(address)` for every address in the inventory, and stop if one is still in the default quota.

The `task:setQuota` Hardhat task runs the last check: it refuses to stop the default quota unless `--required-accounts` lists the inventory and none of it computes in the default quota. If stopping it disrupts live dApps, governance can reopen it with `setQuota(0, maxHCU, decayPerBlock)`. Reopening keeps every assignment. It suspends the routing of allowlisted contracts nobody assigned: they compute in the default quota until it is stopped again, when they return to quota `1` or `2`. The task guards stopping quotas `1` and `2` the same way.

### Depth across transactions

The per-transaction depth limit restarts at every transaction. A chain of dependent transactions, each under that limit, can still build an arbitrarily long sequence of operations that the coprocessor must run one after another. This limit is off by default. When governance enables it (`setCrossTxDepthLimit(maxDepth, decayPerBlock)`, with `maxDepth` at least the per-transaction depth limit), `HCULimit` carries each handle's depth from one transaction to the next:

- A handle's carried depth is stored with each permission granted on it (`FHE.allow`). The ACL returns it with `getStoredDepth(handle, account)`.
- Carried depth decays by `decayPerBlock` every block, as the coprocessor works through the chain. The wait in seconds depends on the host chain's block time.
- Verified inputs, bridged handles and handles whose permission was granted before the limit was enabled start at depth 0.
- An operation whose result would exceed `maxDepth` reverts with `HCUCrossTxDepthLimitExceeded(retryAfterBlocks)`.

`retryAfterBlocks` is the wait for the operation that failed. Later operations of the same transaction can need more, so wait at least that many blocks, simulate again, and repeat until the simulation passes. A transaction that simulates successfully can still revert if another transaction extends the same chain first.

Governance can also give a quota a stored-depth limit with `setQuotaMaxStoredDepth(quotaId, maxStoredDepth)`, from `maxHCUDepthPerTx` to `maxDepth`. A contract in that quota cannot grant a handle deeper than the limit, with `FHE.allow` or `FHE.allowTransient`, to itself or to anyone else: the grant reverts with `HCUStoredDepthExceeded(account, quotaId, retryAfterBlocks)`, and succeeds once the handle has decayed for `retryAfterBlocks`. This limit does not refuse computing a deeper result, only granting it. So a quota can be generous in HCU but keep each value shallow. 0 means no limit beyond `maxDepth`. The default quota should keep the strictest limit: `HCUConfigurator` refuses another quota's limit while the default quota has none or below the default quota's, but does not check a later raise of the default quota's limit against the others. It also refuses a `maxHCUDepthPerTx` above the default quota's limit, so a transaction on fresh inputs can always grant its results. You can read a quota's limit with `getQuotaMaxStoredDepth(quotaId)`.

Depth spreads through every value computed from a deep handle, not only through one user's chain. In a token, a transfer computes the recipient's new balance from the sender's, so the recipient's balance inherits the sender's depth. A value that many transactions update, such as a total supply or a pool balance, can therefore keep a whole dApp waiting on `decayPerBlock`.

## HCU costs for common operations

### Boolean operations (`ebool`)

| Function name | HCU (scalar) | HCU (non-scalar) |
| ------------- | ------------ | ---------------- |
| `and`         | 22,000       | 25,000           |
| `or`          | 22,000       | 24,000           |
| `xor`         | 2,000        | 22,000           |
| `not`         | -            | 2                |
| `select`      | -            | 55,000           |
| `randEbool`   | -            | 19,000           |

---

### Unsigned integer operations

HCU increase with the bit-width of the encrypted integer type. Below are the detailed costs for various operations on encrypted types.

#### **8-bit Encrypted integers (`euint8`)**

| Function name       | HCU (scalar) | HCU (non-scalar) |
| ------------------- | ------------ | ---------------- |
| `add`               | 84,000       | 88,000           |
| `sub`               | 84,000       | 91,000           |
| `mul`               | 122,000      | 150,000          |
| `div`               | 210,000      | -                |
| `rem`               | 440,000      | -                |
| `and`               | 31,000       | 31,000           |
| `or`                | 30,000       | 30,000           |
| `xor`               | 31,000       | 31,000           |
| `shr`               | 32,000       | 91,000           |
| `shl`               | 32,000       | 92,000           |
| `rotr`              | 31,000       | 93,000           |
| `rotl`              | 31,000       | 91,000           |
| `eq`                | 55,000       | 55,000           |
| `ne`                | 55,000       | 55,000           |
| `ge`                | 52,000       | 63,000           |
| `gt`                | 52,000       | 59,000           |
| `le`                | 58,000       | 58,000           |
| `lt`                | 52,000       | 59,000           |
| `min`               | 84,000       | 119,000          |
| `max`               | 89,000       | 121,000          |
| `neg`               | -            | 79,000           |
| `not`               | -            | 9                |
| `select`            | -            | 55,000           |
| `mulDiv`            | 495,000      | 524,000          |
| `randEuint8`        | -            | 23,000           |
| `randEuint8(bound)` | -            | 23,000           |

#### **16-bit Encrypted integers (`euint16`)**

| Function name        | HCU (scalar) | HCU (non-scalar) |
| -------------------- | ------------ | ---------------- |
| `add`                | 93,000       | 93,000           |
| `sub`                | 93,000       | 93,000           |
| `mul`                | 193,000      | 222,000          |
| `div`                | 302,000      | -                |
| `rem`                | 580,000      | -                |
| `and`                | 31,000       | 31,000           |
| `or`                 | 30,000       | 31,000           |
| `xor`                | 31,000       | 31,000           |
| `shr`                | 32,000       | 123,000          |
| `shl`                | 32,000       | 125,000          |
| `rotr`               | 31,000       | 125,000          |
| `rotl`               | 31,000       | 125,000          |
| `eq`                 | 55,000       | 83,000           |
| `ne`                 | 55,000       | 83,000           |
| `ge`                 | 55,000       | 84,000           |
| `gt`                 | 55,000       | 84,000           |
| `le`                 | 58,000       | 83,000           |
| `lt`                 | 58,000       | 84,000           |
| `min`                | 88,000       | 146,000          |
| `max`                | 89,000       | 145,000          |
| `neg`                | -            | 93,000           |
| `not`                | -            | 16               |
| `select`             | -            | 55,000           |
| `mulDiv`             | 703,000      | 766,000          |
| `randEuint16`        | -            | 23,000           |
| `randEuint16(bound)` | -            | 23,000           |

#### **32-bit Encrypted Integers (`euint32`)**

| Function name        | HCU (scalar) | HCU (non-scalar) |
| -------------------- | ------------ | ---------------- |
| `add`                | 95,000       | 125,000          |
| `sub`                | 95,000       | 125,000          |
| `mul`                | 265,000      | 328,000          |
| `div`                | 438,000      | -                |
| `rem`                | 792,000      | -                |
| `and`                | 32,000       | 32,000           |
| `or`                 | 32,000       | 32,000           |
| `xor`                | 32,000       | 32,000           |
| `shr`                | 32,000       | 163,000          |
| `shl`                | 32,000       | 162,000          |
| `rotr`               | 32,000       | 160,000          |
| `rotl`               | 32,000       | 163,000          |
| `eq`                 | 82,000       | 86,000           |
| `ne`                 | 83,000       | 85,000           |
| `ge`                 | 84,000       | 118,000          |
| `gt`                 | 84,000       | 118,000          |
| `le`                 | 84,000       | 117,000          |
| `lt`                 | 83,000       | 117,000          |
| `min`                | 117,000      | 182,000          |
| `max`                | 117,000      | 180,000          |
| `neg`                | -            | 131,000          |
| `not`                | -            | 32               |
| `select`             | -            | 55,000           |
| `mulDiv`             | 1,080,000    | 1,311,000        |
| `randEuint32`        | -            | 24,000           |
| `randEuint32(bound)` | -            | 24,000           |

#### **64-bit Encrypted integers (`euint64`)**

| Function name        | HCU (scalar) | HCU (non-scalar) |
| -------------------- | ------------ | ---------------- |
| `add`                | 133,000      | 162,000          |
| `sub`                | 133,000      | 162,000          |
| `mul`                | 365,000      | 596,000          |
| `div`                | 715,000      | -                |
| `rem`                | 1,153,000    | -                |
| `and`                | 34,000       | 34,000           |
| `or`                 | 34,000       | 34,000           |
| `xor`                | 34,000       | 34,000           |
| `shr`                | 34,000       | 209,000          |
| `shl`                | 34,000       | 208,000          |
| `rotr`               | 34,000       | 209,000          |
| `rotl`               | 34,000       | 209,000          |
| `eq`                 | 83,000       | 120,000          |
| `ne`                 | 84,000       | 118,000          |
| `ge`                 | 116,000      | 152,000          |
| `gt`                 | 117,000      | 152,000          |
| `le`                 | 119,000      | 149,000          |
| `lt`                 | 118,000      | 146,000          |
| `min`                | 150,000      | 219,000          |
| `max`                | 149,000      | 218,000          |
| `neg`                | -            | 131,000          |
| `not`                | -            | 63               |
| `select`             | -            | 55,000           |
| `mulDiv`             | 1,921,000    | 2,911,000        |
| `randEuint64`        | -            | 24,000           |
| `randEuint64(bound)` | -            | 24,000           |

#### **128-bit Encrypted integers (`euint128`)**

| Function name         | HCU (scalar) | HCU (non-scalar) |
| --------------------- | ------------ | ---------------- |
| `add`                 | 172,000      | 259,000          |
| `sub`                 | 172,000      | 260,000          |
| `mul`                 | 696,000      | 1,686,000        |
| `div`                 | 1,225,000    | -                |
| `rem`                 | 1,943,000    | -                |
| `and`                 | 37,000       | 37,000           |
| `or`                  | 37,000       | 37,000           |
| `xor`                 | 37,000       | 37,000           |
| `shr`                 | 37,000       | 272,000          |
| `shl`                 | 37,000       | 272,000          |
| `rotr`                | 37,000       | 283,000          |
| `rotl`                | 37,000       | 278,000          |
| `eq`                  | 117,000      | 122,000          |
| `ne`                  | 117,000      | 122,000          |
| `ge`                  | 149,000      | 210,000          |
| `gt`                  | 150,000      | 218,000          |
| `le`                  | 150,000      | 218,000          |
| `lt`                  | 149,000      | 215,000          |
| `min`                 | 186,000      | 289,000          |
| `max`                 | 180,000      | 290,000          |
| `neg`                 | -            | 168,000          |
| `not`                 | -            | 130              |
| `select`              | -            | 57,000           |
| `randEuint128`        | -            | 25,000           |
| `randEuint128(bound)` | -            | 25,000           |

#### **256-bit Encrypted integers (`euint256`)**

| Function name         | HCU (scalar) | HCU (non-scalar) |
| --------------------- | ------------ | ---------------- |
| `and`                 | 38,000       | 38,000           |
| `or`                  | 38,000       | 38,000           |
| `xor`                 | 39,000       | 39,000           |
| `shr`                 | 38,000       | 369,000          |
| `shl`                 | 39,000       | 378,000          |
| `rotr`                | 40,000       | 375,000          |
| `rotl`                | 38,000       | 378,000          |
| `eq`                  | 118,000      | 152,000          |
| `ne`                  | 117,000      | 150,000          |
| `neg`                 | -            | 269,000          |
| `not`                 | -            | 130              |
| `select`              | -            | 108,000          |
| `randEuint256`        | -            | 30,000           |
| `randEuint256(bound)` | -            | 30,000           |

#### N-ary operations (`sum`, `isIn`)

`FHE.sum` and `FHE.isIn` are priced by the number of elements they process. The cost is bucketed: an array of `n` elements is charged the price of the smallest bucket that fits `n`. Arrays longer than the last bucket for a type are rejected (`FHECollectionSizeInvalid`).

**`sum(values)`**

| Type       | n ≤ 10  | n ≤ 30  | n ≤ 60  | n ≤ 100 |
| ---------- | ------- | ------- | ------- | ------- |
| `euint8`   | 90,900  | 127,000 | 148,000 | 159,000 |
| `euint16`  | 95,000  | 136,000 | 162,000 | 184,000 |
| `euint32`  | 116,000 | 164,000 | 205,000 | 281,000 |
| `euint64`  | 139,000 | 216,000 | 306,000 | -       |
| `euint128` | 219,000 | 355,000 | 552,000 | -       |

**`isIn(value, set)`** (n is the set size)

| Type       | n ≤ 10  | n ≤ 30  | n ≤ 60  | n ≤ 100 |
| ---------- | ------- | ------- | ------- | ------- |
| `euint8`   | 71,300  | 148,000 | 247,000 | 374,000 |
| `euint16`  | 103,000 | 218,000 | 378,000 | 605,000 |
| `euint32`  | 137,000 | 300,000 | 531,000 | 827,000 |
| `euint64`  | 218,000 | 492,000 | 879,000 | -       |
| `euint128` | 256,000 | 535,000 | 921,000 | -       |
| `eaddress` | 286,000 | 584,000 | 885,000 | -       |
| `euint256` | 321,000 | 586,000 | 943,000 | -       |

#### **Encrypted addresses (`euint160`)**

When using `eaddress` (internally represented as `euint160`), the HCU costs for equality and inequality checks and select are as follows:

| Function name | HCU (scalar) | HCU (non-scalar) |
| ------------- | ------------ | ---------------- |
| `eq`          | 115,000      | 125,000          |
| `ne`          | 115,000      | 124,000          |
| `select`      | -            | 83,000           |

For `isIn` on `eaddress`, see the n-ary table above.

## Additional Operations

| Function name    | HCU |
| ---------------- | --- |
| `cast`           | 32  |
| `trivialEncrypt` | 32  |

`mulDiv` is priced as a multiplication plus a division on the doubled width (`mulDiv` on `euint64` costs about a `mul` plus a `div` on 128 bits), which is why it is the most expensive binary operator.

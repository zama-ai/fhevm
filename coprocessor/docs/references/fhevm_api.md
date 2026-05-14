# FHEVM API specifications

## FHE library (library-solidity)

### `FHE.isPublicDecryptionResultValid`

A view function that verifies the KMS signatures on a public decryption result on-chain, without sending a transaction.

```solidity
function isPublicDecryptionResultValid(
    bytes32[] memory handlesList,
    uint256[] memory decryptedResults,
    bytes[] memory signatures
) internal view returns (bool)
```

**Parameters**

| Parameter | Type | Description |
|---|---|---|
| `handlesList` | `bytes32[]` | Ciphertext handles that were decrypted |
| `decryptedResults` | `uint256[]` | Plaintext values returned by the KMS |
| `signatures` | `bytes[]` | EIP-712 signatures from KMS nodes |

**Returns** `true` if at least a threshold number of valid KMS signatures are present; `false` otherwise.

> ⚠️ **Use with care**: Unlike `checkSignatures` (non-view), this function does **not** emit an event recording that verification happened on-chain. Prefer `checkSignatures` in state-changing contexts. Use `isPublicDecryptionResultValid` only for read-only checks (e.g. front-end validation, off-chain tooling).

**Example**

```solidity
bool valid = FHE.isPublicDecryptionResultValid(handles, results, sigs);
require(valid, "invalid decryption result");
```

---

### `FHE.fromExternal` — uninitialized handle support

`FHE.fromExternal` now accepts a zero (uninitialized) handle without reverting. A zero handle represents an unset encrypted value and is returned as a zero-valued ciphertext of the requested type. This allows contracts to safely call `fromExternal` on optional or conditionally-set handles without a prior existence check.

```solidity
// Before v0.12.0: passing a zero inputHandle would revert.
// From v0.12.0: returns a zero-valued ciphertext of the target type.
euint64 value = FHE.fromExternal(externalEuint64.wrap(inputHandle), inputProof);
```


# Verifying public decryptions on-chain

When a contract consumes the result of a public decryption, the only thing standing between it and a forged cleartext is the **decryption proof** produced by the KMS. This page explains what the proof contains, how `FHE.checkSignatures` validates it, what the KMS context is, and how to write a callback that cannot be replayed. For the end-to-end flow (mark, decrypt off-chain, submit), see [Public decryption](oracle.md).

## What the KMS signs

Each KMS node that participates in a public decryption signs an EIP-712 message over:

- the ordered list of handles that were decrypted,
- the ABI encoding of the corresponding cleartexts, in the same order,
- an `extraData` blob that identifies the KMS **context** (the set of signers and threshold in force when the decryption happened).

The `KMSVerifier` contract on the host chain recovers the signers, checks that they belong to the context, that no signer is counted twice, and that the count reaches the public decryption threshold of that context.

## Decryption proof layout

The `decryptionProof` bytes returned by the SDK are:

```text
[ numSigners (1 byte) ] [ 65-byte signature ] x numSigners [ extraData ]
```

`extraData` is version-tagged:

| First byte      | Layout                                          | Meaning                                                     |
| --------------- | ----------------------------------------------- | ----------------------------------------------------------- |
| absent or `0x00` | any                                            | Verify against the **current** KMS context.                 |
| `0x01`          | `[0x01][contextId (32 bytes)]`                  | Verify against the KMS context `contextId`.                 |
| `0x02`          | `[0x01][contextId (32 bytes)][epochId (32 bytes)]` | Same, plus the key epoch inside that context.            |

Any other version or a malformed length reverts with `UnsupportedExtraDataVersion` or `DeserializingExtraDataFail`. You never build this blob yourself: the SDK returns the proof exactly as the relayer assembled it, and you forward it unchanged.

## KMS contexts and why they matter

The set of KMS operators can change over time (a **context switch**, for example when operators are added or keys are resharded). A proof produced under context `N` stays verifiable after the protocol moves to context `N+1`, as long as context `N` has not been **destroyed** by governance. Once destroyed, its proofs no longer verify. Practical consequences:

- Do not store a proof for later. Submit it in the same transaction, or shortly after, the off-chain decryption.
- A contract that keeps decryption results must store the **cleartexts**, not the proof.
- `FHE.getContextSignersAndThresholdFromExtraData(extraData)` exposes, for auditing or monitoring, the signer set and threshold a proof will be checked against.

## The two verification functions

```solidity
function checkSignatures(
    bytes32[] memory handlesList,
    bytes memory abiEncodedCleartexts,
    bytes memory decryptionProof
) internal

function isPublicDecryptionResultValid(
    bytes32[] memory handlesList,
    bytes memory abiEncodedCleartexts,
    bytes memory decryptionProof
) internal view returns (bool)
```

| | `checkSignatures` | `isPublicDecryptionResultValid` |
| --- | --- | --- |
| On invalid proof | reverts with `InvalidKMSSignatures` | returns `false` |
| On malformed proof | reverts | reverts |
| Emits `PublicDecryptionVerified(handlesList, abiEncodedCleartexts)` | yes | no |
| Caches verification in transient storage (cheaper repeated checks) | yes | no |
| Usable in `view` / `eth_call` | no | yes |

Use `checkSignatures` in every state-changing callback. Use the `view` variant only for read-only checks such as off-chain simulation, and always `require` its return value if you do call it on-chain: forgetting the `require` silently accepts forged results.

Both functions revert when the proof is empty or too short, when fewer valid signatures than the threshold are present, or when a signature was produced by an address that is not a registered signer of the resolved context. The `view` variant additionally returns `false` when the threshold is reached only by counting a signer twice.

## Ordering

The proof is bound to the **order** of `handlesList`. `abiEncodedCleartexts` must be `abi.encode(v0, v1, ...)` with the cleartexts in the same order and with the Solidity types matching the encrypted types (`bool` for `ebool`, `uint8` for `euint8`, `address` for `eaddress`, and so on). Use `FHE.toBytes32` to build the handle list from typed handles.

## Replay protection is your job

`checkSignatures` proves that *the KMS decrypted these handles to these values*. It does not prove that the caller is entitled to trigger the callback, nor that the callback has not run before. The same `(handles, cleartexts, proof)` triple verifies as many times as it is submitted. Every callback must therefore carry its own guard:

```solidity
bool private _revealed;

function reveal(uint64 clearTotal, bytes calldata proof) external {
  require(!_revealed, "already revealed");

  bytes32[] memory handles = new bytes32[](1);
  handles[0] = FHE.toBytes32(_encryptedTotal);
  FHE.checkSignatures(handles, abi.encode(clearTotal), proof);

  _revealed = true;
  _settle(clearTotal);
}
```

Typical guards: a boolean or a status enum per request, a mapping keyed by the handle, or a nonce included in the request. Checking that the handle passed in is the one your contract expects (as above, by reading it from storage rather than from calldata) also prevents a caller from submitting a valid proof for a different, unrelated handle.

## Errors

| Error                              | Meaning                                                              |
| ---------------------------------- | -------------------------------------------------------------------- |
| `InvalidKMSSignatures()`           | `checkSignatures` failed verification.                               |
| `EmptyDecryptionProof()`           | Proof has zero length.                                               |
| `DeserializingDecryptionProofFail()` | Proof shorter than `1 + 65 * numSigners`.                          |
| `DeserializingExtraDataFail()`     | `extraData` length does not match its version.                       |
| `UnsupportedExtraDataVersion(v)`   | Unknown `extraData` version byte.                                    |
| `KmsContextNotCreated(...)`, `InvalidKmsContext(...)` | The proof references a KMS context the `ProtocolConfig` does not know or no longer serves (destroyed). |

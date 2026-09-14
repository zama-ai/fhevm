# QA finding — the SDK does not verify the response `extraData`

**Date:** 2026-09-14
**Base:** branch `main`, SDK at `sdk/js-sdk`
**Context:** raised while implementing the container half of the QA scenario
*"The SDK uses a new epoch in the same context"* (see `test-suite/fhevm/qa-kms-context-scenario-1-epoch.md`).

## Summary

There is more than one `extraData` in a user-decryption flow, and they are not checked against each
other.

| `extraData` | Where it lives | Exposed to a caller? | Verified by the SDK? |
|---|---|---|---|
| **Request** | the EIP-712 message of the decryption permit | **yes** | n/a — the SDK produces it |
| **Response, per KMS share** | one per share in the relayer response | no | **no** |
| **Response, public-decrypt** | the relayer's public-decrypt response | no | **no** — explicitly discarded |

The QA scenario clause *"the response extraData must be identical to the request extraData"* is
therefore **not assertable through the SDK's public surface today**.

This document records the finding. It is not a bug report: parts of the missing verification appear
to have been switched off deliberately, and the decision of whether to re-enable them is a product
call, not a QA one.

## 1. The request `extraData` — exposed

Built in `createUnsignedDecryptionPermitEip712V2`
(`sdk/js-sdk/src/core/kms/SignedDecryptionPermitV2-p.ts:307-333`) from the current KMS signers
context, and placed in the signed EIP-712 message.

Reachable from the public API:

```ts
// @fhevm/sdk/actions/base  (core/actions/base/index.ts:72)
const permit = await client.signUnifiedDecryptionPermit({ /* ... */ });
permit.eip712.message.extraData;   // "0x02" + contextId(32B) + epochId(32B) = 132 chars
```

Type: `KmsUserDecryptEip712V2Message` (`sdk/js-sdk/src/core/types/kms.ts:122-129`). It is a raw hex
string, not a `KmsExtraData` object — `createKmsUserDecryptEip712V2.ts:96` stores
`extraData.bytesHex`.

Note: `createKmsExtraDataFromBytesHex`, `createKmsExtraDataV2` and the `KmsExtraData` type are
**internal** (`kmsExtraData-p.ts`, not re-exported by any barrel or subpath). A consumer must decode
by slicing. The layout is documented at `kmsExtraData-p.ts:166-175`:

```
0x 02 <contextId: 64 hex chars> <epochId: 64 hex chars>
   ^^ version      chars 4..68            chars 68..132
```

## 2. The response `extraData` — received, never compared

The SDK does read it. In the user-decryption path each KMS share carries its own value
(`sdk/js-sdk/src/core/modules/relayer/module/fetchUserDecryptV2.ts:62-69`):

```ts
const share: KmsSigncryptedShare = {
  signature: r.signature,
  payload: r.payload,
  extraData: remove0x(r.extraData),
};
```

The request value is kept separately as `metadata.eip712ExtraData`
(`fetchKmsSigncryptedSharesV2-p.ts:188`). Nothing compares the two.

Concretely, every mechanism that would perform the check is absent or disabled:

- **`equalsKmsExtraData` has zero production call sites.** Defined at `kmsExtraData-p.ts:309`,
  referenced only by `kmsExtraData-p.test.ts`. Verified by repo-wide grep.
- **The response-signature verification is commented out** —
  `fetchKmsSigncryptedSharesV2-p.ts:195-218` is a `/* … */` block. That check would have bound each
  share's signature to its `extraData`.
- **The cross-share consistency check is commented out** —
  `core/modules/decrypt/module/api-p.ts:284-292`:

  ```ts
  // const firstExtraDataBytesHex = firstShare.extraData;
  // for (let i = 1; i < sharesArray.length; i++) {
  //   ... `Mismatched extraData across shares` ...
  ```

- **Public decrypt discards it explicitly** — `core/kms/publicDecrypt.ts:77-89`:

  ```ts
  // ignore returned relayer extraData as we never trust the relayer
  // extraData: relayerExtraDataBytesHex,
  ```

  The relayer does return it (`fetchPublicDecrypt.ts:44`); `PublicDecryptionProof.extraData` is
  rebuilt from on-chain state instead (`PublicDecryptionProof-p.ts:122,130,181`).

The invariant is documented as *not* holding — `KmsSigncryptedShares-p.ts:50-59`:

> - All shares may not have identical `extraData` values.
> - The shared `extraData` may not match the `extraData` derived from the associated
>   `KmsSignersContext`.

A mismatch would therefore surface, if at all, only as an opaque WASM signature/quorum failure —
never as an SDK-level `extraData` error.

Only the mock/cleartext path has real checks (`core/modules/decrypt/mock.ts:116-124` cross-share,
`:159-162` share vs payload), and neither compares against the request permit.

## 3. Not exposed either

`KmsSigncryptedShare` and `KmsSigncryptedSharesMetadata` are private types (`core/types/kms-p.ts`).
Reading them needs `getShares` / `getMetadata`, which are token-gated `@internal`
(`KmsSigncryptedShares-p.ts:199-214`). The public `KmsSigncryptedShares` interface exposes only
`tkmsVersion` and a brand (`core/types/kms.ts:177-180`), and it never reaches a public return type
anyway: `decryptValue` and `decryptValues` return `TypedValue`s
(`core/actions/decrypt/decryptValue.ts:24`).

So there is no supported way for a test to observe the response `extraData` through the SDK.

## 4. Consequence for the QA scenario

The scenario has four `Then` clauses about `extraData`:

| Clause | Assertable via the SDK? |
|---|---|
| request extraData decodes as version `0x02`, context `C`, epoch `E` | **yes** |
| the decoded epoch must not equal `E_prev` | **yes** |
| the decryption completes successfully | yes |
| **response extraData identical to the request extraData** | **no** |

### Decision taken

For now we assert **only on the request**: the `extraData` embedded in the permit must carry the
same `(context, epoch)` the orchestrator observed on chain after the rotation — i.e. the SDK and the
host must be on the same epoch. That is the behaviour the scenario actually exists to protect, and
it is fully assertable today.

The response-echo clause is **deferred and recorded here**, not silently dropped.

### If we later want the response clause

Two options, neither requiring a change to the assertion above:

1. **Bypass the SDK for that clause.** Use the raw connector HTTP client the e2e suite already has —
   `requestUnifiedUserDecrypt` in `test/sdk/connector/connectorHttp.ts`, as used by
   `test/unifiedUserDecryption/unifiedUserDecryption.ts` — which exposes the per-share response
   `extraData` directly. This tests the relayer/connector contract rather than the SDK.
2. **Change the SDK.** Re-enable the commented-out verification, or surface the response
   `extraData` on a public return type. This is a product decision; the code is present and
   deliberately inert, so it should not be flipped on from a QA ticket.

## 5. Related

| Path | Role |
|---|---|
| `test-suite/fhevm/qa-kms-context-scenario-1-epoch.md` | the profile this finding came out of |
| `test-suite/e2e/test/qa-tests.compare.md` | the wider QA gap analysis |
| `sdk/js-sdk/src/core/kms/kmsExtraData-p.ts` | extraData construction, decoding and versions |
| `sdk/js-sdk/src/core/kms/fetchKmsSigncryptedSharesV2-p.ts` | where the response shares land |

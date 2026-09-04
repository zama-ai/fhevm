# SDK Review Plan — permit versioning, thresholds, EIP-712, tests

Analysis of 15 review points against `sdk/js-sdk` (read-only investigation, no code changed).
Each point is triaged as **confirmed bug**, **decision needed**, **additive work**, or **verified-fine**,
with the evidence behind it. File:line references are as of this review.

## Contracts versions

v12 contracts: <fhevm>/sdk/js-sdk/contracts/src/v0.12.0/host-contracts
v13 contracts: <fhevm>/sdk/js-sdk/contracts/src/v0.13.0/host-contracts
v14 contracts: <fhevm>/host-contracts/contracts

## Triage overview

| #    | Point                                              | Verdict                                              |
| ---- | -------------------------------------------------- | ---------------------------------------------------- |
| 1,2  | thresholds uint256 not uint8                       | Confirmed bug                                        |
| 3    | getResolvedProtocolVersion undefined?              | Verified — not a bug                                 |
| 4    | zero addresses in permit                           | Decision (optional hardening); premises corrected    |
| 5,6  | decrypt v2/v3 routing                              | Latent risk, not an active bug                       |
| 9–15 | SDK must only produce v1 permits / v0–v1 extraData | Confirmed: invariant NOT currently guaranteed (core) |
| 7    | parse/serialize permit test                        | Additive — zero coverage today                       |
| 8    | createEip712 for Ankur                             | Additive — does not exist                            |

---

## Workstream A — The v1-only invariant (points 6, 9, 10, 11, 12, 13, 14, 15) — highest priority

**Finding: the SDK can currently produce v2 permits and v2 extraData; there is NO "experimental" flag
anywhere in `src`** (the only `experimental` hit is an unrelated worker comment,
`core/base/isomorphicWorker.ts:251`). Two independent violation paths:

- **Path A (v2 permit):** `signDecryptionPermit` router dispatches to `signDecryptionPermitV2` whenever
  resolved protocol ≥ 0.14.0 — `SignedDecryptionPermit-p.ts:138-142`. `signDecryptionPermitV2` exists and
  is publicly reachable (`SignedDecryptionPermitV2-p.ts:150`).
- **Path B (v2 extraData inside a v1 permit):** `signDecryptionPermitV1` (`SignedDecryptionPermitV1-p.ts:192`)
  calls the **generic** `readCurrentKmsSignersContext` at **line 196** (there is no V1-restricted reader),
  which falls through to the **ungated** `_readKmsSignersContext_Protocol_14_or_higher`
  (`readKmsSignersContext-p.ts:97-98,170`) whenever the on-chain KMSVerifier is ≥ v0.4.0. That builds v2
  extraData with a possibly non-zero `epochId` (`:184,203-209`). Then **line 201** `kmsSignersContextToExtraData`
  auto-selects the version from `epochId` (`KmsSignersContext-p.ts:142-145` → `kmsExtraData-p.ts:240-259`) —
  **not capped at v1**.

**Routing note (points 5, 6):** routing does **not** branch on `permit.version` — every layer branches on the
resolved protocol version at a single `0.14.0` boundary (`decryptValuesFromPairs.ts:49,57`;
`modules/relayer/module/fetchUserDecrypt.ts:28`). "v2 route"/"v3 route" = the HTTP endpoints
`v2/user-decrypt` (via `fetchUserDecryptV1`, `fetchUserDecryptV1.ts:48`) and `v3/user-decrypt`
(via `fetchUserDecryptV2`, `fetchUserDecryptV2.ts:50`). In v0.13.0 the v3 endpoint is unreachable (gated by
0.14.0) — **point 6 already holds structurally.** Point 5's premise is true (the v1 fetch route reads
`contractAddresses`, which a v2 permit message lacks — `fetchKmsSigncryptedSharesV1-p.ts:53,67,98`), but no
active misroute exists because permit-version and route move together via the shared boundary. The real
weakness: routing trusts _context_, not the _permit_.

**Plan (maps to points 10–15):**

1. **Introduce an explicit `experimental` mode flag** (does not exist yet) on the runtime/client, threaded
   into the kms context readers.
2. **Gate `_readKmsSignersContext_Protocol_14_or_higher` behind experimental**
   (`readKmsSignersContext-p.ts:97-98`); in normal mode never enter it. (point 15)
3. **Give `signDecryptionPermitV1` a v1-only signer context** — a dedicated `readCurrentKmsSignersContextV1`
   or a `maxVersion=1` constraint — so line 196 can never yield a v2 context / non-zero epoch. (points 10, 14)
4. **Cap `kmsSignersContextToExtraData` at v1** and add `assert(extraData.version <= EXTRA_DATA_V1)` inside
   `signDecryptionPermitV1` so Path B **fails loudly** instead of silently emitting v2. (points 9, 12)
   The V2 signer already has the mirror assert (`SignedDecryptionPermitV2-p.ts:165-168`).
5. **Forbid `signDecryptionPermitV2` in normal mode** — router throws for ≥ 0.14.0 unless experimental
   (`SignedDecryptionPermit-p.ts:138-142`). (point 11) Decide whether `parse`/`serialize` should still
   _accept_ v2 (`:49,63-68,163-167`) or also reject it.
6. **Harden routing** (points 5, 6): in `decryptValuesFromPairs.ts:57` and
   `modules/relayer/module/fetchUserDecrypt.ts:28`, cross-check `signedPermit.version` against the
   protocol-derived branch and throw on mismatch, rather than casting blindly.
7. Review `reconcileKmsSignersContext` (`readKmsSignersContext-p.ts:293,346-351`), which can pull a v2
   extraData from relayer input — gate it too.

**Decision needed:** the gating model — a single boolean `experimental` on the client vs. keying purely off
resolved protocol version. Points 14 ("epochId 0") and 15 ("experimental only") imply: normal mode =
v1/epoch-0 always; experimental = allow the 0.14.0 v2/v3 paths.

---

## Workstream B — Threshold type bug (points 1, 2) — confirmed, self-contained

**Finding:** on-chain and the SDK's own ABI fragments declare thresholds as **uint256**
(`core/host-contracts/abi-fragments/fragments.ts:232,484`; host `KMSVerifier.sol:202`,
`InputVerifier.sol:349`), but the TS layer types them as `Uint8Number` and validates with `isUint8`, so
`readContract` returns a `bigint` that **throws `"Invalid threshold."` for any value > 255**
(`core/base/uint.ts:143`).

- KMS: `core/types/kmsSignersContext.ts:53`, `core/host-contracts/KmsSignersContext-p.ts:39,49,124,135`,
  `core/host-contracts/getKmsContextSignersAndThresholdFromExtraData-p.ts:104-109`,
  `core/host-contracts/getKmsContextSignersAndThreshold-p.ts:90-91,120-124`.
- Coprocessor: `core/types/coprocessor.ts:66`, `core/types/coprocessorSignersContext.ts:7`,
  `core/host-contracts/CoprocessorSignersContext-p.ts:24,32,95,138`,
  `core/host-contracts/getCoprocessorContextSignersAndThreshold-p.ts:76-77,107-111`.

**Plan:** retype both thresholds to a uint256/`bigint` type, validate with `isUint256`
(`core/base/uint.ts:163`), drop the `Number(...)` narrowing (also lossy above 2^53), and switch comparisons
like `recoveredAddresses.length < threshold` to `BigInt(length) < threshold`
(`CoprocessorSignersContext-p.ts:138`). Low-frequency in practice (threshold = node count today) but a real
correctness cap.

---

## Workstream C — Zero addresses in permit (point 4) — decision, with corrections

**Premise corrections:**

- **There is no RFC 12** in the repo; only `notes/rfc/DRAFT_RFC_016.md` exists (RFC 16). It types
  `contractAddresses: readonly string[]` but says nothing about zero address / duplicates / min-count.
- The permit's contract list is enforced in **`gateway-contracts/contracts/Decryption.sol`**, NOT host
  `KMSVerifier.sol` (which only checks KMS response signatures).

**Finding:** neither the SDK nor the contract rejects the **zero address or duplicates**.

- SDK: `fetchKmsSigncryptedSharesV1-p.ts:49-59` (membership only); empty + max-10 at `:91-106`
  (`MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10`, `:43`); `SignedDecryptionPermitV1-p.ts:159-165`.
- Contract: `Decryption.sol:464,556` — empty, max-10, user/delegator-not-in-list; ct-pair-in-list at `:1289`.
  No zero-address / duplicate check.

No SDK-vs-chain divergence; a zero address is semantically useless (ACL `isAllowed` fails downstream) rather
than a security hole.

**Decision:** (a) leave as-is (matches chain), or (b) add early SDK hardening — reject zero-address and
duplicate entries in `assertPermitV1IncludesContractAddresses` and the permit builder. Lean (b) — cheap,
fail-fast — but optional.

---

## Workstream D — Tests + createEip712 (points 7, 8) — additive

**Point 7 — zero permit serialize/parse coverage today** (only a commented-out block in
`index.hello.test.ts:150-199`). Functions: `serializeSignedDecryptionPermitToJSON`
(`SignedDecryptionPermit-p.ts:32`, `instanceof` guard `:45`), `parseSignedDecryptionPermit` (`:149`, version
default→1 `:157`, unknown throws `:167`); public wrappers in `actions/chain/serializeSignedDecryptionPermit.ts:27`
and `actions/chain/parseSignedDecryptionPermit.ts:32`.

Add:

- **Unit** (`src/core/kms/SignedDecryptionPermit-p.test.ts`): serialize rejects non-Impl input; parse version
  dispatch (unknown→throw, absent→v1); malformed-input rejection (`SignedDecryptionPermitV1-p.ts:277-306`);
  publicKey-mismatch (`:289-294`).
- **Round-trip** (in `test/fheTest/**`, needs chain): `parse(serialize(permit))` deep-equals for v1 self +
  v1 delegated (+ v2 only if experimental kept).
- **Flag:** `ParseSignedDecryptionPermitParameters` (`actions/chain/parseSignedDecryptionPermit.ts:10`) names
  the field `serializedPermit` and accepts **object only**, violating `.claude/naming.md`
  (`serialized: string | Record<...>`); a JSON _string_ currently fails. Lock or fix.

**Point 8 — `createEip712` does not exist.** Internal builders exist but are unexported:
`createKmsUserDecryptEip712V1` (`core/kms/createKmsUserDecryptEip712V1.ts:38`), `...V2`
(`createKmsUserDecryptEip712V2.ts:39`), delegated, domain. RFC003 (`notes/rfc/RFC003.md:256-362`) specifies
public `createUserDecryptEIP712` / `createDelegatedUserDecryptEIP712` that "construct the EIP-712 typed data
without signing."

Plan: add a **decrypt-tier public action** `createUserDecryptEip712(fhevm, params)` (+ delegated) that resolves
protocol version + chain fields (`verifyingContractAddressDecryption`, `chainId`), resolves `extraData`
(fetch on-chain or accept as param), dispatches to the internal builders, returns typed data without signing.
Per Workstream A, in normal mode it builds **V1** typed data. Name must be `createEip712`/`createUserDecryptEip712`
(casing rule, `.claude/naming.md`).

**Decision needed:** confirm with Ankur whether the desired shape is RFC003's signature and whether `extraData`
is caller-supplied or fetched on-chain.

---

## Workstream E — Verified, no action (point 3)

`getResolvedProtocolVersion` (`core/runtime/CoreFhevm-p.ts:619`) is correctly typed
`ProtocolVersionResolution | undefined` and **all 7 call sites handle undefined** (throw a clear error, or
fall back in `core/runtime/resolveFhevmVersions-p.ts:50-58`); tests assert the undefined path
(`ProtocolVersionResolver-p.test.ts`). Nothing to fix.

---

## Recommended sequencing

1. **A** first — correctness/safety invariant (shipping v2 permits/extraData in v0.13.0 would break
   decryption) and the largest surface. Its experimental-flag decision unblocks the v2 questions in C/D.
2. **B** in parallel — isolated, mechanical.
3. **D** after A (tests encode the v1-only invariant; `createEip712` honors it).
4. **C** last — optional hardening.

## Open decisions before implementing

1. The **experimental-gating model** for Workstream A.
2. Whether **createEip712** follows RFC003's signature with caller-supplied `extraData`.

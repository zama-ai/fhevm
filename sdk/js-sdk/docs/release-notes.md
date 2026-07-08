# Release Notes

## v1.2.0-alpha.1 (2026-07-08)

### For SDK users

#### Added

- **Cleartext client support for `forge-fhevm` v1 (legacy) deployments.** The cleartext client
  (`createFhevmCleartextClient`/`createFhevmCleartextBaseClient` and friends) now works against
  local `forge-fhevm` v1 stacks in addition to the current cleartext host-contract set. It
  detects a v1 deployment by its well-known single KMS signer address and, when detected,
  reconstructs public-decrypt, user-decrypt, and input-proof/coprocessor-signature results
  off-chain (reading raw plaintexts from `CleartextFHEVMExecutor.plaintexts` and recomputing the
  EIP-712 digests locally) instead of relying on the `Cleartext*KMSVerifier`/`CleartextInputVerifier`
  views that a plain `forge-fhevm` deployment doesn't expose. See the new
  `test/scripts/HOWTO_RUN_FORGE_FHEVM_V1.md` for how to set up a local `forge-fhevm` v1 stack for
  testing.

#### Fixed

- **`euint32`/`euint256` clear values weren't validated against their correct upper bound.**
  `asClearValueType()` looked up the max-value table with the FHE type name (`'euint32'`,
  `'euint256'`) instead of the corresponding clear value-type name (`'uint32'`, `'uint256'`), so
  the lookup missed and no upper-bound check was applied for those two types. This affected the
  clear value returned by `decryptValue()`/`decryptValues()`.

### Internal

- Refactored KMS `extraData` encode/decode/validation (`fromKmsExtraData`, `toKmsExtraData`,
  `assertIsKmsExtraData`) from free functions in `kmsExtraData.ts` into a validated
  `KmsExtraDataImpl` class in `kmsExtraData-p.ts`, with matching updates across
  `KmsSignersContext-p.ts`, `readKmsSignersContext-p.ts`, `KmsSigncryptedShares-p.ts`,
  `SignedDecryptionPermitV1-p.ts`/`V2-p.ts`, `PublicDecryptionProof-p.ts`, and the
  `createKmsUserDecryptEip712*`/`createKmsDelegatedUserDecryptEip712V1` helpers, plus new unit
  tests (`kmsExtraData-p.test.ts`).
- Pinned a specific `fhevm` repo version for running localstack tests below protocol v0.14, and
  added a large batch of unit/fheTest coverage: `errors/utils.test.ts`, `object.test.ts`,
  `trustedValue.test.ts`, `ClearValue.test.ts`, `FhevmHandle.test.ts`, ethers/viem
  `clientDecrypt` cleartext tests, and fheTest coverage for `decryptValuesFromPairs` /
  `canDecryptValue(s)` / `canDecryptValuesFromPairs`.
- Fixed the cleartext ethers/viem runtimes (`getCleartextEthersRuntime`/`getCleartextViemRuntime`)
  re-creating the ethereum/relayer modules on every call even when a cached runtime already
  existed.
- README: fixed documentation link and package-name references (points at
  `github.com/zama-ai/fhevm` instead of the old `relayer-sdk` repo, `@fhevm/sdk` instead of
  `@zama-fhe/relayer-sdk`).

---

## v1.1.0-alpha.7 (2026-07-03)

### For SDK users

#### Changed

- **`signDecryptionPermit()` is now a single unified call.** The separate self/delegated
  overloads are gone — there is only one signature now:

  ```ts
  signDecryptionPermit(fhevm, parameters: SignDecryptionPermitParameters): Promise<SignedDecryptionPermit>
  ```

  Pass `delegatorAddress` to get a delegated permit, omit it for a self permit. The
  `SignSelfDecryptionPermitParameters` / `SignDelegatedDecryptionPermitParameters` types no
  longer exist — use `SignDecryptionPermitParameters` for both cases.

- **Breaking: `durationDays` → `durationSeconds`.** `SignDecryptionPermitParameters` no longer
  takes a whole-number-of-days duration. Replace:

  ```ts
  // Before
  await signDecryptionPermit(fhevm, { ..., durationDays: 7 });

  // After
  await signDecryptionPermit(fhevm, { ..., durationSeconds: 7 * 24 * 60 * 60 });
  ```

- **`SignedDecryptionPermit` always exposes `isDelegated: boolean` directly**, instead of it
  being implicit in which permit type (`SignedSelfDecryptionPermit` vs
  `SignedDelegatedDecryptionPermit`) you got back.

- **Permits now carry a `version` field** (`1` or `2`) reflecting the on-chain protocol they
  were created against. You generally don't need to branch on this yourself — it's used
  internally when serializing/parsing/verifying a permit — but keep in mind that a permit
  signed against one protocol version isn't interchangeable with a permit for another.

#### Removed

The following low-level exports were dropped from the public API surface. If your code
imports any of these directly, switch to the higher-level actions listed:

| Removed export                                                                                  | Use instead                          |
| ----------------------------------------------------------------------------------------------- | ------------------------------------ |
| `createKmsUserDecryptEip712`, `createKmsDelegatedUserDecryptEip712`                             | `signDecryptionPermit()`             |
| `verifyKmsUserDecryptEip712`, `verifyKmsDelegatedUserDecryptEip712`                             | `signDecryptionPermit()`             |
| `createKmsEip712Domain`, `createCoprocessorEip712Domain`                                        | `signDecryptionPermit()`             |
| `readKmsSignersContext`, `readCoprocessorSignersContext`                                        | _(internal — no public replacement)_ |
| `createVerifiedInputProofFromRawBytes`, `verifyInputProof`, `fetchVerifiedInputProof`           | `encryptValue()` / `encryptValues()` |
| `verifyZkProofCoprocessorSignatures`, `verifyHandlesCoprocessorSignatures`                      | _(internal — no public replacement)_ |
| `fetchKmsSigncryptedShares`, `decryptKmsSigncryptedShares`                                      | `decryptValue()` / `decryptValues()` |
| `resolveChainId`                                                                                | `resolveFhevmConfig()`               |
| `readFhevmExecutorContractData`, `readKmsVerifierContractData`, `readInputVerifierContractData` | _(internal — no public replacement)_ |

#### Fixed

- **Faster pickup of KMS/coprocessor signer changes.** The client-side cache for KMS and
  coprocessor signer/context data now expires after 15 minutes instead of 24 hours, so signer
  rotations on-chain are reflected sooner. (Immutable contract-version lookups keep the 24h
  cache.)
- Relayer error responses now recognize additional labels that previously fell through as
  unrecognized errors: `not_allowed_on_host_acl` (400), and `insufficient_balance` /
  `insufficient_allowance` (503).

> **Note:** `docs/decryption.md`, `docs/migration.md`, `docs/getting-started.md`, and
> `docs/api-reference.md` still show the old `durationDays` / `SignedSelfDecryptionPermit` /
> `SignedDelegatedDecryptionPermit` API and need to be updated separately.

### Internal

- Implemented RFC-016 (unified EIP-712 decryption request): permit creation, EIP-712 typed
  data, and signature verification are now split into protocol-versioned implementations
  (`SignedDecryptionPermitV1-p.ts` for protocol ≤ v0.13, `SignedDecryptionPermitV2-p.ts` for
  protocol ≥ v0.14) behind the single `signDecryptionPermit` / `SignedDecryptionPermit-p.ts`
  dispatch layer. Same split applied to `fetchKmsSigncryptedShares` (`...V1-p.ts` / `...V2-p.ts`)
  and the KMS EIP-712 type/message definitions in `core/types/kms.ts`.
- Renamed internal `EIP712`-cased identifiers to `Eip712` for consistent casing across the
  codebase.
- Added `scripts/list-sdk-exports.mjs`, a dev tool that statically lists every publicly exported
  name per entry point declared in `src/package.json` — used to audit the public API surface
  (see the "public api cleanup" change above).
- Converted `test/scripts/fhevm-info.sh` to `test/scripts/fhevm-info.mjs` (JavaScript).
- Added a `--localstacks-only` option to `test/scripts/dod.mjs`.
- Various logging cleanups: reduced noisy `logger.warn` calls, removed two large JSON
  config dumps from logs, slightly improved fheTest logging.
- Removed an unnecessary atomic wait in `initAliceFheTestHandles` (test setup).
- Fixed two build warnings.
- Updated the e2e test suite for the new unified permit API.
- Restored rollup/esbuild platform binaries that had gone missing from the workspace lockfile.

---

## v1.1.0-alpha.6 (2026-06-29)

### For SDK users

#### Changed

- Bumped the bundled TFHE WASM runtime from `1.6.1` to `1.6.2` (`initTfheModule`'s default
  `tfheVersion` and the `HYPER_WASM_SOLVER_CONFIG` compatibility matrix), keeping the SDK
  aligned with the latest coprocessor-compatible TFHE build.

#### Fixed

- **Fixed Next.js/Turbopack support.** Turbopack couldn't statically analyze the SDK's dynamic
  `import('node:worker_threads')`, so it silently replaced the call with a stub that threw
  `Cannot find module 'unknown'` — disabling multi-threaded TFHE in Next.js server components
  without any visible error. The SDK now guards every Node built-in import with
  bundler-specific magic comments (`@vite-ignore`, `webpackIgnore`, `turbopackIgnore`) so Vite,
  webpack, and Turbopack all leave the import untouched, restoring multi-threaded TFHE under
  Turbopack/Next.js. A new `docs/runtime-compatibility.md` documents the full support matrix
  (browsers, Node, Bun, Deno, Electron, Next.js CSR/SSR, and why Vercel Edge/Cloudflare Workers
  remain unsupported).
- Fixed WASM decompression on runtimes where `DecompressionStream` exists as a global but
  throws on construction (notably the Next.js Edge Runtime): the SDK now probes usability by
  constructing the stream rather than trusting `typeof`, and falls back to a bundled pure-JS
  inflater when it isn't actually usable — keeping the compressed embedded WASM loadable on
  affected runtimes and older browsers (Firefox older than 113, Safari older than 16.4).
- Fixed `_assertIsKmsUserDecryptEip712Base` incorrectly validating delegated user-decrypt
  EIP-712 requests against the plain `kmsUserDecryptEip712Types` schema instead of
  `kmsDelegatedUserDecryptEip712Types`, which caused valid `DelegatedUserDecryptRequestVerification`
  payloads to be rejected.
- Fixed `toTransportKeyPair` requiring a `tkmsVersion` property that is no longer part of the
  expected input shape, which caused an unnecessary validation failure.

### Internal

- Added a new Playwright-based `test/browser-next/` suite and `test/infra/` harness to exercise
  the SDK inside real Next.js (Turbopack/webpack, CSR/SSR) builds, catching the
  bundler-analysis regressions above.
- Reworked internal environment/runtime detection into a new `core/base/environment.ts` module
  (`isNodeLike`, `isBrowserLike`, `getNodeBuffer`, `getNodeFs`, `getNodeUrl`, `getNodeWorker`,
  `supportsWebWorkerApi`, `supportsDecompressionStream`) consolidating capability probes
  previously scattered across `isomorphicWorker.ts` and `wasm.ts`.
- Added a pure-JS inflater (`core/base/inflate.ts`) and a new `Sha256VerificationError` error
  type as internal building blocks for the WASM-loading fixes above.
- Rewrote `isomorphicWorker.ts`'s worker-backend selection (`resolveWorkerApi`) and the
  encrypt/decrypt module `init-p.ts` initialization paths to use the new environment probes
  instead of ad hoc runtime checks.
- Pinned `conventional-changelog-conventionalcommits` to `^7` in the PR-lint CI workflow to fix
  a broken commitlint install.
- Moved the JS SDK npm provenance publish job to a GitHub-hosted (`ubuntu-latest`) runner;
  self-hosted runners were rejected by npm provenance publishing with `E422`.
- Miscellaneous formatting (`.prettierrc.yml`) and dead-code cleanup ahead of merging to main.

---

## v1.1.0-alpha.5 (2026-06-24)

### For SDK users

#### Added

- **Automatic protocol-version and WASM-version resolution.** `Fhevm` clients now expose a
  `protocolVersion` field (`ProtocolVersionResolution`), resolved automatically at
  initialization time by reading on-chain host-contract data (RFC-005). Based on this, the SDK
  auto-selects the matching `tfhe.wasm`/`kms_lib.wasm` build for you — no configuration needed
  for the common case.
- **`moduleVersions` option to override WASM version selection.** `FhevmOptions` (and the new,
  more specific `FhevmEncryptOptions` / `FhevmDecryptOptions`) accept a `moduleVersions` field
  — `'auto'` (default) or an explicit `{ tfhe?, kms?, checkCompatibility? }` — for advanced
  users who need to pin a specific `tfhe`/`kms` WASM build instead of relying on
  auto-resolution. `checkCompatibility` (`'throw' | 'warn' | 'off'`) controls what happens when
  an explicit version doesn't match what the on-chain protocol expects.
- **`epochId` tracking in KMS extra-data (RFC-005).** The KMS context now carries an `epochId`
  alongside the existing signer/threshold data, needed to support protocol versions that
  key-rotate within a context.

#### Changed

- **Breaking: `KmsDecryptEip712Like` renamed to `Eip712Like`** in the `@fhevm/sdk/types` entry
  point.

#### Fixed

- **Relayer/edge error messages are now surfaced instead of a generic status message.** When
  the relayer returns a 401, or an edge proxy (Cloudflare/Kong) blocks a request with a 403,
  the SDK now includes the actual message the server sent (via a new `Details:` line on
  `RelayerResponseStatusError`) instead of only a hardcoded/generic "unexpected status"
  message. Applies to both the async request engine (input-proof/decrypt POST+GET polling) and
  the keyurl fetch path.
- **More robust `wasmBaseUrl` resolution.** The internal WASM base-URL resolver no longer
  misdetects its runtime in bundler/browser environments that inject Node-shaped globals (e.g.
  a shimmed `__filename`) — it now explicitly distinguishes real Node CJS from browser-like and
  Bun runtimes before deciding how to resolve the WASM asset URL, fixing WASM loading failures
  in certain bundled/browser setups.

### Internal

- Added `ProtocolVersionResolver` and `resolveFhevmVersions-p.ts` (protocol-version resolution,
  semver range matching) and expanded `HyperWasmSolver` to dispatch across multiple
  compatibility rules instead of a single static mapping; both are covered by new unit tests
  (`ProtocolVersionResolver-p.test.ts`, `HyperWasmSolver-p.test.ts`).
- Added a semver-parsing utility module (`core/base/semver.ts`) with unit tests.
- Added `ProtocolConfig` contract address to chain definitions (localTestnet, mainnet, sepolia,
  devnet, localstack variants) and to the `FhevmChain`/`HostContract` types, wired into
  `resolveFhevmConfig`.
- Renamed `getVersion` to `getHostContractVersion` across host-contract readers, KMS context
  helpers, and `ProtocolVersionResolver` for clarity.
- Renamed generic `address` fields/variables to contract-specific names (`aclAddress`,
  `fhevmExecutorAddress`, etc.) throughout host-contract helpers, ACL checks, delegation, and
  KMS share-fetching code — no behavior change.
- Fixed a type-guard bug in `assertRecordUintBigIntProperty` (`core/base/uint.ts`) that was
  incorrectly narrowing the checked value.
- Removed the dead, never-exported `FhevmUser` and `FhevmDecryptionKey` internal types.
- Added `readErrorMessage()`, a best-effort JSON error-body reader shared by the relayer
  async-request engine and keyurl fetch path, covered by new unit tests
  (`readErrorMessage.test.ts`, `RelayerAsyncRequest.errors.test.ts`).
- Added a `DRAFT_RFC_016.md` design document and updated `docs/compatibility.md`,
  `docs/GLOSSARY.md` (restructured top-down by altitude) with the new protocol/WASM
  compatibility model.
- Fixed a lint-breaking typo (`Signedcrypted` → `Signcrypted`) in
  `decryptKmsSigncryptedShares.ts` identifiers.
- Switched the js-sdk testnet/e2e test expectations to protocol v0.13; gated the hosted-relayer
  auth-error E2E test to mainnet only, since testnet no longer reliably enforces auth on keyurl
  requests.
- CI: parallelized the `dod:full` workflow and fixed several localstack test-setup issues
  (relayer config template, matrix-expansion hardening flagged by zizmor).
- Various test-suite fixes: delegation-propagation timing in delegated-decrypt tests,
  off-by-one timestamp flakiness in `signDecryptionPermit` request-validity tests, and general
  fheTest/viem test refactoring.

---

## v1.1.0-alpha.4 (2026-05-29)

### For SDK users

#### Added

- **Custom HTTP headers for relayer requests.** `RelayerCommonOptions` (which
  `RelayerUserDecryptOptions`, `RelayerInputProofOptions`, and the other relayer option types
  extend) now accepts a `headers: Record<string, string>` field. Anything you pass here is
  merged into every relayer fetch (`encryptValue()`/`encryptValues()`,
  `decryptValue()`/`decryptValues()`, and public decrypt), letting you attach things like a
  custom API key or tracing header without going through the `auth` option.

#### Changed

- **Stricter relayer URL validation.** The relayer base URL you configure is now parsed and
  validated up front (must be `http:`/`https:`, no embedded credentials, no query string or
  fragment, HTTPS required when auth credentials are set unless the host is localhost).
  Previously-tolerated malformed URLs now throw `InvalidUrlError` early instead of failing
  later with a less clear fetch error.
- Header names/values passed anywhere in the SDK are now normalized and validated against RFC
  7230 (lower-cased names, rejects CR/LF/NUL to prevent header injection, throws on
  case-colliding duplicate header keys).

#### Fixed

- **API key is now forwarded on relayer GET requests.** Async decrypt/encrypt jobs poll their
  status via GET; the `auth` credentials (e.g. Zama API key) were previously omitted from
  those polling requests by design and are now included, fixing polling against relayer
  deployments that require auth on every request.

### Internal

- Reworked `relayerUrl.ts`: replaced the old `parseZamaRelayerUrl` allowlist-style check with
  `validateRelayerBaseUrl()` (returns a normalized `URL`) and `buildRelayerUrlString()` for
  path joining.
- Added `InvalidUrlError` and header validation helpers (`normalizeHeaderName`,
  `validateHeaderValue`, `normalizeHeaders`) in `core/base/fetch.ts` and `core/base/string.ts`.
- Expanded `core/base/fetch.test.ts` with coverage for the new URL/header validation.
- Improved the compatibility matrix in `docs/compatibility.md`.

---

## v1.1.0-alpha.3 (2026-05-29)

### For SDK users

#### Fixed

- **Corrected the `threshold` argument passed to KMS response reconstruction.** The wasm-side
  decrypt/reconstruct call (`process_user_decryption_resp_from_js`) was being passed an
  explicit `threshold` value derived from `kmsSignersContext.threshold`, which didn't match
  what the KMS library expects there (it's a distinct value from
  `KMSVerifier.getThreshold()` — see the inline comment added in
  `core/modules/decrypt/module/api-p.ts`). It's now left `undefined` so it's computed
  automatically from the number of server addresses, fixing decryption reconstruction in
  configurations where the two thresholds diverged.

### Internal

- Refactored the fheTest cleartext/ciphertext test code.
- Updated `docs/compatibility.md`.
- Test-only: reorganized `test/fheTest/chains/*` into a shared `test/chains/` folder and added
  a `polygon_devnet` test chain config (despite the `feat(sdk): add support for another
evm-compatible sidechain` commit title, this change is scoped entirely to the test suite —
  no new chain was added to the public `@fhevm/sdk/chains` entry point).
- Renamed a handful of `*.readPublicValue.test.ts` fixture files to
  `*.decryptPublicValue.test.ts` to match the `alpha.2` rename.

---

## v1.1.0-alpha.2 (2026-05-27)

### For SDK users

#### Changed

- **Breaking: `readPublicValue*` renamed to `decryptPublicValue*`.** To match the naming of the
  rest of the decryption API, the following exports from `./actions/base` (and the matching
  client methods on `.`/`./ethers`/`./viem` clients) were renamed:

  | Old name                                                                         | New name                                                                                  |
  | -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
  | `readPublicValue` (+ `ReadPublicValueParameters`/`ReadPublicValueReturnType`)    | `decryptPublicValue` (+ `DecryptPublicValueParameters`/`DecryptPublicValueReturnType`)    |
  | `readPublicValues` (+ `ReadPublicValuesParameters`/`ReadPublicValuesReturnType`) | `decryptPublicValues` (+ `DecryptPublicValuesParameters`/`DecryptPublicValuesReturnType`) |
  | `readPublicValuesWithSignatures` (+ types)                                       | `decryptPublicValuesWithSignatures` (+ types)                                             |
  | `canReadPublicValue` (+ types)                                                   | `canDecryptPublicValue` (+ types)                                                         |
  | `canReadPublicValues` (+ types)                                                  | `canDecryptPublicValues` (+ types)                                                        |

  Update calls like `client.readPublicValue({ encryptedValue })` to
  `client.decryptPublicValue({ encryptedValue })`.

- **Default `tfhe.wasm` version bumped from `1.5.3` to `1.6.1`**, and the default `tkms` wasm
  version bumped from `0.13.10` to `0.13.20-0`. This affects which key/CRS format the SDK
  expects by default when no explicit version is configured — relevant mainly to users running
  against local/dev chains rather than public testnets, where the relayer's key material may
  not match the new default.

#### Fixed

- User-decryption responses from a v11-protocol relayer (which omit the `extraData` field) are
  no longer rejected: the response guard now backfills `extraData: '0x'` before validating,
  instead of failing the shape assertion.

### Internal

- Added `docs/compatibility.md`, documenting the TFHE/KMS/protocol version compatibility
  matrix (which `tfhe.wasm` version can read which PubKey/CRS, KMS-to-`tfhe-rs` crate pinning,
  per-chain protocol/PubKey versions, and on-chain contract versions per protocol release).
- Added multi-localstack test support (`localstack_v11`/`v12`/`v13`/`v14` chain configs,
  `fhevm-info.sh`, `lib-common.sh`, expanded `localstack-restart.sh`/`localstack-run-tests.sh`,
  new `test:localstack:v11`/`v12`/`v13` npm scripts) and a new `test/browser-ui` manual test
  harness, replacing the old single `abi-v1.ts`/`abi-v2.ts`/`fhe-test-addresses-*.json`
  fixtures with a `chain-defaults.json` manifest.
- Loosened the `tfhe.wasm` type-check contract (`type-check.test.ts`) to ignore the `eq` method
  on `CompactCiphertextList`/`ProvenCompactCiphertextList` instances, since it's not exposed
  consistently across wasm-bindgen versions and unused by the SDK.
- Worker SHA-256 mismatch errors (`startWorkers.js`) now include the offending `url` in the
  error message for easier debugging.

---

## v1.1.0-alpha.1 (2026-05-22)

### For SDK users

#### Added

- **Cleartext mode**: new `createFhevmCleartextClient`, `createFhevmCleartextBaseClient`,
  `createFhevmCleartextEncryptClient`, and `createFhevmCleartextDecryptClient` factories
  (exported from `@fhevm/sdk/ethers/cleartext` and `@fhevm/sdk/viem/cleartext`), letting apps
  run against cleartext host contracts (e.g. for local/dev testing) instead of the full FHE
  stack.
- `asEncryptedValue` and `isEncryptedValue` are now properly exported from `@fhevm/sdk/types`.
- Multi-version WASM support: the SDK now bundles and can select between multiple `tfhe`
  (v1.5.3, v1.6.0-dev, v1.6.1) and `tkms` (v0.13.10, v0.13.20-0) WASM module versions at
  runtime instead of shipping a single pinned version. (This is rolled back for the `v1.0.0`
  stable cut and reintroduced in a more mature, auto-resolving form at `v1.1.0-alpha.5`.)

#### Changed

- The `ethers` cleartext client factories now accept any `ethers.ContractRunner` (matching the
  non-cleartext `ethers` clients) instead of requiring a `Provider`, so signers can be passed
  directly.

#### Fixed

- Relayer response validation for `publicDecrypt`/`userDecrypt` no longer requires the
  `extraData` field on results, since KMS/coprocessor protocol v0.11 relayer responses omit
  it; the SDK now tolerates its absence while still validating it when present.
- `fromKmsExtraData()` / `assertIsKmsExtraData()` now also accept the bare `0x` empty value (in
  addition to `0x00`) as "no extra data," fixing decryption/signing against protocol v0.11.
- Fixed the secp256k1 signature parsing in the internal `sign()` helper for `@noble/curves`
  v2.x, which changed its recovered-signature byte layout (recovery byte moved from the last
  byte to the first). Signing decryption permits and delegation requests was broken against
  the newer curve library before this fix.
- Reduced the published npm package size by excluding `vitest.config.ts` and `tsconfig.json`
  from the `files` shipped in `@fhevm/sdk`.

### Internal

- Massive branch consolidation: this range merges the long-running `devex/js-sdk` cleartext
  and multi-version host-contracts work into the release line (`feat(sdk): js sdk cleartext
mode, multi-version host-contracts, and e2e test-suite integration`, #2479), including new
  Foundry deployment scripts, cleartext Solidity contract mirrors (`CleartextACL`,
  `CleartextFHEVMExecutor`, `CleartextKMSVerifier`, etc.), and a new
  `sdk/js-sdk-hardhat-v2-test` package for Hardhat-based e2e coverage.
- Added `RelayerAsyncRequest`, an internal retry/backoff/polling engine used by
  `fetchPublicDecrypt`, `fetchUserDecryptV1`/`V2`, `fetchDelegatedUserDecryptV1`, and
  `fetchCoprocessorSignatures` to make relayer HTTP calls more resilient (configurable
  retries, timeouts, and structured `RelayerAsyncRequestState`/`RelayerStateError` /
  `RelayerResponseStatusError` reporting).
- Added `RelayerErrorBody` type guard for parsing structured relayer error JSON bodies.
- Added internal `checkDelegation`, `isHandleDelegatedForUserDecryption`, and a
  `host-contracts/getUserDecryptionDelegationExpirationDate` helper for delegation checks
  against host contracts (not exposed as public actions).
- Renamed `WILDCARD_CONTRACT` to `WILDCARD_DELEGATION_ADDRESS` and `Keypair` identifiers/files
  to `KeyPair` throughout the codebase for naming consistency.
- Reworked the internal `createTypedValue`/`createTypedValueArray` helpers (now accepting
  `uint160` and canonicalizing it to `address`, with simplified generics) — these remain
  unexported from `@fhevm/sdk/base` (the barrel was and stays empty), so this is not a public
  API change.
- Extensive unit test additions (Vitest): `ZkProofBuilder`, `FheType`, ported relayer-sdk unit
  tests, and many standalone ethers/viem smoke tests factored out of ad hoc scripts.
- Large e2e test-suite rework under `test-suite/e2e` and `test-suite/fhevm` (new
  `test/sdk/fhevm-sdk` and `test/sdk/relayer-sdk` suites, v0.11/v12/v13 cleartext host-contract
  profiles, Docker/compose fixes for host-listener-consumer version gating, anvil/cleartext
  chain script factoring).
- Numerous build/tooling fixes: ESLint errors and `@noble/curves` v2.x compatibility across the
  package, CJS/ESM `tsconfig` split for the `wasm` directory, `codegen-loaders.mjs`
  prod/dev profile options, and package script adjustments.

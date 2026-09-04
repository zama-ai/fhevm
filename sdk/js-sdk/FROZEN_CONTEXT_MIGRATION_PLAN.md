# Frozen Context Migration Plan

Make `FhevmClientFrozenContext` the **single source of truth** for every resolved
version (protocol, PubKey/CRS, TFHE, TKMS, host-contract versions), and delete the
parallel per-version memo machinery that currently causes drift.

## End-state model

- `resolveFhevmClientFrozenContext(fhevm)` resolves the **complete** version basis
  in one pass (already implemented, `src/core/frozenContext/`).
- `ensureFrozenContext(fhevm)` resolves it **once**, deduping concurrent callers on
  a client-held promise, then collapses to stored data on the client
  (`#frozenContext`). Already implemented. It is the frozen-context analogue of the
  lazy `initTfheModule` — idempotent, point-of-use, retry-on-error.
- The `_init*` functions become **eager prefetches** (nothing depends on them
  running; actions resolve lazily too).
- All version reads go through the frozen context. No `getResolved*` / `setResolved*`,
  no `#protocolVersion` / `#tfheVersion` / `#tkmsVersion` fields.

### What stays (do NOT touch)

- `globalFheEncryptionKeyCache` (pub key / CRS bytes) — heavy, id/relayer-addressed,
  orthogonal to version coherence.
- `cachedTfhe/TkmsModulePromiseByVersion` (module-init promise caches) — keep, now
  fed the version from the frozen context.
- Pure derivations `protocolContextFromAclVersion`, `pubKeyCrsVersionFromProtocolVersion`
  (used by the frozen-context resolver).

## Consumer graph (the migration surface)

Version state is read by **four** clusters, not just the init fns:

1. `_initBase` / `_initEncrypt` / `_initDecrypt` — via
   `ensureResolvedProtocolVersion` / `resolveFhevmTfheVersion` / `resolveFhevmTkmsVersion`
   + `setResolved*`.
2. `asFhevmWithTfheVersion` / `asFhevmWithTkmsVersion` (`CoreFhevm-p.ts`) — read
   `getResolved*` internally; used by 7 action files:
   - encrypt: `encryptValue`, `encryptValues`, `generateZkProof`
   - decrypt: `decryptValue`, `decryptValues`, `decryptValuesFromPairs`, `generateTransportKeyPair`
3. `fetchKmsSigncryptedSharesV1-p.ts` / `V2-p.ts` — call `resolveFhevmTkmsVersion(context)`
   directly (not init fns).
4. Public client getters `client.protocolVersion` / `tfheVersion` / `tkmsVersion` —
   read the `#…Version` fields (used by tests + `asFhevmWith*`).

## Steps

> Do NOT compile between steps 1–5. Compile only at step 6. The intermediate is
> deliberately broken (once step 1 stops calling `setResolvedTfheVersion`, the public
> `.tfheVersion` reads a never-set field until step 5 re-points it).

### Step 1 — migrate all writers/readers to the frozen context

**1a. Init fns** — `src/core/clients/decorators/base.ts`, `encrypt-p.ts`, `decrypt-p.ts`

```ts
// _initBase
async function _initBase(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  await ensureFrozenContext(fhevm);
}

// _initEncrypt
export async function _initEncrypt(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'encrypt');
  const frozen = await ensureFrozenContext(f);
  await Promise.all([
    fetchFheEncryptionKeyBytes(f, {}),
    f.runtime.encrypt.initTfheModule({ tfheVersion: frozen.tfheVersion }),
  ]);
}

// _initDecrypt
export async function _initDecrypt(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'decrypt');
  const frozen = await ensureFrozenContext(f);
  await f.runtime.decrypt.initTkmsModule({ tkmsVersion: frozen.tkmsVersion });
}
```
Imports: drop `ensureResolvedProtocolVersion` / `resolveFhevmTfheVersion` /
`resolveFhevmTkmsVersion` and `setResolvedTfheVersion` / `setResolvedTkmsVersion`;
add `ensureFrozenContext`. Keep `asFhevmClientWith`, `fetchFheEncryptionKeyBytes`.

**1b. `asFhevmWith*`** — `CoreFhevm-p.ts` (~lines 453–478)
Re-point the internal guards from `getResolved*Version(f) === undefined` to
`getFrozenContext(f)` / `getFrozenContext(f)?.hasTfheVersion` etc. The 7 action call
sites stay unchanged.

**1c. KMS shares** — `fetchKmsSigncryptedSharesV1-p.ts`, `V2-p.ts`
Replace `resolveFhevmTkmsVersion(context)` with `(await ensureFrozenContext(context)).tkmsVersion`.

### Step 2 — verify zero remaining callers

```
grep -rn "ensureResolvedProtocolVersion\|resolveFhevmTfheVersion\|resolveFhevmTkmsVersion\|resolveFhevmProtocolVersion" src --include='*.ts' | grep -v '\.test\.ts'
grep -rn "getResolvedProtocolVersion\|getResolvedTfheVersion\|getResolvedTkmsVersion\|setResolvedProtocolVersion\|setResolvedTfheVersion\|setResolvedTkmsVersion" src --include='*.ts' | grep -v '\.test\.ts'
```
Both must return only the definitions themselves (about to be deleted).

### Step 3 — remove the old machinery

- `CoreFhevm-p.ts`: delete exported `getResolvedProtocolVersion`,
  `getResolvedTfheVersion`, `getResolvedTkmsVersion`, `setResolvedProtocolVersion`,
  `setResolvedTfheVersion`, `setResolvedTkmsVersion`.
- `resolveFhevmVersions-p.ts`: delete `ensureResolvedProtocolVersion`,
  `resolveFhevmProtocolVersion`, `_resolveFhevmProtocolContext`,
  `resolveFhevmTfheVersion`, `resolveFhevmTkmsVersion` (remove the file if nothing
  else remains). Keep the pure derivations they used (they live in
  `ProtocolVersionResolver-p.ts`).

### Step 4 — remove the symbol machinery

`CoreFhevm-p.ts`: delete symbols `GET_PROTOCOL_VERSION`, `SET_PROTOCOL_VERSION`,
`GET_TFHE_VERSION`, `SET_TFHE_VERSION`, `GET_TKMS_VERSION`, `SET_TKMS_VERSION`; their
in-class `Object.defineProperties` entries; and the `Get/SetProtocol/Tfhe/TkmsVersionFn`
type aliases. (Keep the `*_FROZEN_CONTEXT` and `*_FROZEN_CONTEXT_PROMISE` machinery.)

### Step 5 — re-point public getters + drop the fields

`CoreFhevm-p.ts`:
- `protocolVersion` / `tfheVersion` / `tkmsVersion` getters read from `#frozenContext`
  (throw a clear "await client.ready" message when the context or that version is
  absent). Keep the public properties — `client.protocolVersion` is asserted in tests.
- Delete fields `#protocolVersion`, `#tfheVersion`, `#tkmsVersion` and their
  constructor initializers.

### Step 6 — compile + verify

```
npx tsc -p src/tsconfig.json --noEmit
```
Then a targeted fhe test that reads `client.protocolVersion` / `getFrozenContext`
after `client.ready` (e.g. `viem-common/clientBase.tests.ts`).

## Notes

- `ensureFrozenContext` retry-on-error is only reachable via lazy point-of-use calls
  / `extend()` / direct calls — a failure *during init* still poisons `#readyPromise`
  like any init failure (init fns bundle retryable RPC with one-shot WASM startup).
  Out of scope here.
- A future enhancement: have actions `await ensureFrozenContext(fhevm)` at point of
  use (truly lazy + retryable) instead of the sync `asFhevmWith*`. Not required for
  this migration.
- `refresh()` (atomic whole-basis swap via `setFrozenContext`) is a separate,
  later feature.

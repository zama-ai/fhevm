# WASM Memory Leak Test Plan (TKMS decrypt path)

## Context

The TKMS decrypt path creates wasm-bindgen objects (ML-KEM keys, client, server addresses,
plaintexts). **WASM linear memory is not reclaimed by the JS garbage collector** — each object
must be `free()`d explicitly, and there is no `FinalizationRegistry` backstop in the SDK.

The `.free()` semantics were recently audited and fixed. The objects and their ownership:

Producers / owners (see [src/core/modules/decrypt/module/api-p.ts](../src/core/modules/decrypt/module/api-p.ts)):

- `generateTkmsPrivateKey` → owned `TkmsPrivateKey` (wasm private key).
- `deserializeTkmsPrivateKey` → owned `TkmsPrivateKey`.
- `GET_PUBLIC_KEY_FUNC` → **fresh, caller-owned** `TkmsPublicKey` (non-caching, since the fix).
- `decryptAndReconstruct` internally creates: 1 public key, N `ServerIdAddr`, 1 `Client`,
  M `TypedPlaintext`.

Free helper: [`WasmScope`](../src/core/base/wasmScope.ts) — LIFO, best-effort, tracks only OWNED objects.

### Ownership rules (from the Rust signatures in `zama-ai/kms`)

- `new_client(server_addrs: Vec<ServerIdAddr>, …)` takes the addrs **by value → moves/consumes them**.
  The `ServerIdAddr` wrappers must **NOT** be freed by us (double-free).
- `process_user_decryption_resp_from_js(client: &mut Client, …, enc_pk: &…, enc_sk: &…)` takes
  everything **by reference** (borrows) and returns an owned `Vec<TypedPlaintext>`.
  → `Client`, the derived public key, and each `TypedPlaintext` are ours to free; the caller's
  private key (`enc_sk`) is only borrowed and must survive the call.

### Sites fixed (the behaviors under test)

| Site | Expected behavior |
|---|---|
| `getTkmsPublicKeyHex` | derives a public key, returns its hex, frees the public key in `finally` |
| `GET_PUBLIC_KEY_FUNC` | returns a **fresh** public key each call (no caching); caller frees it |
| `decryptAndReconstruct` | frees public key + `Client` + all `TypedPlaintext`; never frees the moved `ServerIdAddr`s or the borrowed caller private key |
| `generateTransportKeyPair` | frees the transient generated private key in `finally` |
| `GetTkmsPrivateKeyFn` (realize) | frees the freshly deserialized private key **only if** `verifyTkmsPublicKey` throws; otherwise returns it |
| `decryptKmsSigncryptedShares` | frees the private key it obtained, in `finally` |

## Goal

Prove that:

1. Every wasm object minted on the decrypt/keygen paths is freed exactly once.
2. No object is freed that we do not own (moved-into-`new_client` addrs; borrowed caller key).
3. Repeated operations do not grow `WebAssembly.Memory` unbounded.
4. The specific bugs that were fixed do not regress.

## Strategy

Two complementary levels.

### Level 1 — Unit tests with a free-counting mock (fast, deterministic, CI)

Replace the TKMS wasm module (`kmsLib`) with a fake whose factory functions return objects with a
counting `free()`. This gives an exact allocation/free ledger without a live stack.

- Wrap/mimic `kmsLib.ml_kem_pke_keygen`, `u8vec_to_ml_kem_pke_sk`, `ml_kem_pke_get_pk`,
  `new_server_id_addr`, `new_client`, `process_user_decryption_resp_from_js` to return objects that
  increment a global `allocated` counter on creation and a `freed` counter on `.free()`
  (and record double-frees / frees-after-move).
- Model the **move semantics** faithfully: `new_client` marks each passed `ServerIdAddr` as
  *consumed*; a later `.free()` on a consumed addr records a `doubleFree`/`freeAfterMove` error.
- Model **borrow**: `process_…` does not consume `client` / `enc_pk` / `enc_sk`.

Assertions after each operation:

- `allocated === freed` for objects the SDK owns.
- `doubleFree === 0` and `freeAfterMove === 0`.
- The caller-owned private key is still alive (not freed) after `decryptAndReconstruct` returns.

### Level 2 — Integration (live stack, `fheTest`) memory-growth guard

Run against `localstack_v12/v13` (see the existing `fheTest` setups). Drive many iterations of the
real decrypt path and assert bounded WASM memory.

- Capture `WebAssembly.Memory.buffer.byteLength` (via the loaded TKMS module) after a warm-up
  iteration (module load + first-use allocations are expected and excluded).
- Run K iterations (e.g. 200) of: `signDecryptionPermit` → `decrypt` (full `decryptValuesFromPairs`).
- Assert steady-state growth is ~0 (allow a small slack for allocator fragmentation, e.g. ≤ 1 page
  / ≤ 64 KiB total over K iterations). A per-iteration leak of a ~1.6 KB key + client + plaintexts
  would blow well past that.
- Log the delta even on pass (`no silent caps`): record `bytesBefore`, `bytesAfter`, `perIter`.

## Test matrix (Level 1, per site)

1. **`getTkmsPublicKeyHex`**
   - one call → 1 public key allocated, 1 freed; returns correct hex.
   - **called twice on the same private key** → succeeds both times (regression: the old caching bug
     freed a cached public key, leaving a dangling `#publicKey` that threw "already disposed" on the
     2nd call). Assert no throw, 2 alloc / 2 free.

2. **`decryptAndReconstruct`** (happy path)
   - public key: 1 alloc / 1 free.
   - `ServerIdAddr`: N alloc, N consumed by `new_client`, **0 freed by us**, 0 freeAfterMove.
   - `Client`: 1 alloc / 1 free.
   - `TypedPlaintext`: M alloc / M free.
   - caller private key: alive after return.
   - error path: force `process_…` to throw → still frees public key + client (finally); server addrs
     already moved (Rust drops them); no double-free.

3. **`generateTransportKeyPair`**
   - private key: 1 alloc / 1 free (freed in `finally` after serialize + pubkey-hex).
   - public key derived inside `getTkmsPublicKeyHex`: 1 alloc / 1 free.
   - error path: `serializeTkmsPrivateKey` throws → private key still freed.

4. **`GetTkmsPrivateKeyFn` (realize)**
   - success: 1 private key alloc, returned (not freed here); freed later by
     `decryptKmsSigncryptedShares`.
   - **verify-failure**: `verifyTkmsPublicKey` throws (mismatched public key) → the freshly
     deserialized private key is freed, error propagates. Assert 1 alloc / 1 free and the original
     error is rethrown.

5. **`decryptKmsSigncryptedShares`**
   - private key obtained via `transportKeyPairToTkmsPrivateKey` is freed in `finally` on both the
     success and `decryptAndReconstruct`-throws paths.

## Regression tests (the specific bugs fixed)

- **Public-key caching double-free / use-after-free**: `getTkmsPublicKeyHex` twice on one key (see 1).
- **`ServerIdAddr` double-free**: assert the moved addrs are never `free()`d by the SDK (see 2).
- **Borrowed caller key freed too early**: after `decryptAndReconstruct`, the passed private key is
  still usable / not freed (see 2).
- **Transient generate key leak**: `generateTransportKeyPair` frees its private key (see 3).
- **Verify-failure leak**: realize frees on `verifyTkmsPublicKey` throw (see 4).

## Tooling notes

- The wasm classes (`Client`, `ServerIdAddr`, `TypedPlaintext`, `PrivateEncKeyMlKem512`,
  `PublicEncKeyMlKem512`) all expose `free(): void` (see `src/wasm/tkms/*/kms_lib.d.ts`).
- For Level 1, inject the fake `kmsLib` through the decrypt module init seam
  ([init-p.ts](../src/core/modules/decrypt/module/init-p.ts)) or spy on the real one and wrap the
  returned objects' `free`.
- For Level 2, only `WebAssembly.Memory` growth is observable (no per-object introspection); it is a
  coarse but decisive net-leak signal over many iterations.

## Out of scope / caveats

- The one-time cost of loading a WASM module and its first-use allocations is **not** a leak; exclude
  it via a warm-up iteration.
- `WebAssembly.Memory` grows and does not shrink — Level 2 asserts *bounded steady-state growth*, not
  a return to a baseline.
- TFHE (encrypt) worker/WASM lifetime is covered by [MULTI_WASM_TEST_PLAN.md](MULTI_WASM_TEST_PLAN.md);
  this plan is scoped to the TKMS decrypt object lifecycle.
- The type system cannot catch a wrong `.free()` (double-free / free-what-you-don't-own); only these
  runtime tests can — which is the whole point of Level 1's move/borrow modeling.

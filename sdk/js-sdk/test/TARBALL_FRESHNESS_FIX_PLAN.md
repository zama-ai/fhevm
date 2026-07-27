# Fix: durable tarball freshness for test platforms (Option A — content-hashed tarball)

## Symptom

`test/browser-next` ran the TFHE `mt+coop` cell against a **stale** `@fhevm/sdk`.
The worker fix was present in source and in the freshly packed tarball, but the
copy Turbopack bundled was the pre-fix build, so MT init still failed with:

```
[FAIL] All worker creation methods failed
  [cause] TypeError: __turbopack_context__.x is not a function
    at __newNodeWorkerFromJsCode      <-- old __isBrowserLike() → Node worker path in the browser
```

`npm run build` / `npm run test` were green throughout, because neither reinstalls
the tarball into `browser-next` (its `test:browser` step does not cover the Next
platform), and single-thread mode never spawns a worker.

## Root cause

One tarball, **constant name + version** (`fhevm-sdk-1.1.0-alpha.5.tgz`), is
consumed by multiple independent `node_modules` trees:

| Consumer | dep | refreshed by repack? |
|---|---|---|
| `test/manual-pack` | `file:fhevm-sdk-1.1.0-alpha.5.tgz` | yes (`rebuild_sdk_and_pack.sh` nukes its `node_modules` + lock) |
| `test/browser-next` | `file:../manual-pack/fhevm-sdk-1.1.0-alpha.5.tgz` | **no** |

npm keys `file:` tarball installs by **resolved version**. Because the version
never changes between repacks, `npm install` in `browser-next` sees the dep
already satisfied (existing extraction + a lockfile pinning the old integrity) and
**skips re-extraction** — even on `run.mjs --rebuild` (it calls `npm install`, but
npm no-ops). `shouldInstall()` only checks *existence*, never *freshness*, so it
can't catch this.

## Goal

After a single `rebuild_sdk_and_pack.sh` run, **every** test platform that consumes
the tarball is guaranteed to run the freshly built SDK, a stale install **fails
loudly** instead of silently, and repacking is **skipped when the SDK is unchanged**.

## Design: Option A — content-addressed tarball name

Give each freshly built tarball a **new identity** by embedding a short content
hash in its filename: `fhevm-sdk-<ver>-<sha8>.tgz`. Because each consumer's `file:`
dependency *string* then changes, npm re-resolves and re-extracts everywhere — no
reliance on cache-busting tricks. The filename also makes "which build is
installed" inspectable at a glance.

Rejected alternatives (recap): `file:../../src` / `npm link` bypass the tarball and
stop validating the *published* artifact (exports map, `files`, bundled wasm);
bumping the real published version pollutes release versioning for a test concern.

---

## Work items

### 1. Consumer registry (single source of truth)

In `test/scripts/rebuild_sdk_and_pack.sh`, declare the consumers once:

```bash
# Dirs whose package.json has "@fhevm/sdk": "file:.../fhevm-sdk-*.tgz".
CONSUMERS=("$ROOT_DIR/test/manual-pack" "$ROOT_DIR/test/browser-next")
```

Optionally auto-discover instead, so new platforms are picked up automatically:

```bash
mapfile -t CONSUMERS < <(grep -rl 'fhevm-sdk-.*\.tgz' "$ROOT_DIR/test" \
  --include=package.json | grep -v node_modules | xargs -n1 dirname)
```

`manual-pack` is special-cased only in that its tarball lives **inside** it; other
consumers reference it by relative path.

### 2. Content-hash the packed tarball and rename it

After `npm pack` produces `fhevm-sdk-<ver>.tgz`:

```bash
RAW_TARBALL=$(echo "$PACK_DIR"/fhevm-sdk-*.tgz)
VER=$(node -p "require('$ROOT_DIR/src/package.json').version")
SHA8=$(shasum -a 256 "$RAW_TARBALL" | cut -c1-8)
TARBALL_NAME="fhevm-sdk-${VER}-${SHA8}.tgz"
mv "$RAW_TARBALL" "$PACK_DIR/$TARBALL_NAME"
```

`rm -f "$PACK_DIR"/fhevm-sdk-*.tgz` at the top still clears the previous hashed
tarball, so only one ever exists.

> Note: hashing the **tarball** (not `dist/`) is intentional — it's exactly the
> bytes that get installed, so the hash is a faithful identity of what runs.
> `npm pack` is deterministic enough here; if pack-time nondeterminism (timestamps)
> ever bites, hash the `dist/` inputs instead and stamp that into the name.

### 3. Rewrite every consumer's `file:` dep to the new tarball name

For each consumer, point `@fhevm/sdk` at the freshly hashed tarball (correct
relative path per consumer) using a small node helper (robust JSON edit):

```bash
for c in "${CONSUMERS[@]}"; do
  node "$SCRIPT_DIR/set-sdk-dep.mjs" "$c/package.json" "$(rel_to "$c" "$PACK_DIR/$TARBALL_NAME")"
done
```

`manual-pack/package.json` keeps the bare filename (`file:<name>.tgz`); the script
already rewrites it — fold it into the same loop. `set-sdk-dep.mjs` reads JSON,
sets `dependencies["@fhevm/sdk"] = "file:" + relPath`, writes it back.

### 4. Force re-extract per consumer (without nuking unrelated deps)

```bash
for c in "${CONSUMERS[@]}"; do
  rm -rf "$c/node_modules/@fhevm/sdk"   # drop the old extraction
  (cd "$c" && npm install)              # re-resolves the changed file: spec
done
```

`manual-pack` may keep its existing `rm -rf node_modules package-lock.json`
(it's tiny). `browser-next` must NOT lose `next`/`react` — removing only
`node_modules/@fhevm/sdk` + `npm install` is enough, because the dep string
changed in step 3 (so the lockfile entry is rewritten, not reused).

### 5. Freshness guard (the durability)

New `test/scripts/assert-fresh-sdk.mjs`:

- Resolve the consumer's `@fhevm/sdk` `file:` target from its `package.json`.
- sha256 a sentinel file inside the **tarball** (`package/wasm/tfhe/v1.6.1/startWorkers.js`)
  and the **installed** copy (`node_modules/@fhevm/sdk/wasm/tfhe/v1.6.1/startWorkers.js`).
- Mismatch (or missing install) → `throw` with a clear message:
  `"@fhevm/sdk in <consumer> is stale (installed != packed). Run: npm run test -- --rebuild"`.

Invoke it:
- from `test/browser-next/globalSetup.ts` (before any spec runs), and
- from `test/browser-next/scripts/run.mjs` right after the install step.

This converts the entire "silently runs old SDK" failure class into a hard,
actionable error — even if a future npm version changes caching behavior.

### 6. Conditional repack (fast dev loop — performance refinement)

Replace `run.mjs`'s existence-only `shouldInstall()` with a fingerprint check, and
skip the 24 MB repack when the SDK is unchanged:

- Compute an SDK fingerprint: `git rev-parse HEAD:src` + dirty-hash of `src/` and
  `scripts/wasm/` (or `git status --porcelain` over those paths). Store it in
  `test/manual-pack/.sdk-fingerprint` next to the tarball when packing.
- `rebuild_sdk_and_pack.sh`: if the current fingerprint matches the stored one and
  a tarball exists, **skip build+pack** (unless `--rebuild` / `--build-profile`
  forces it).
- `run.mjs`: repack only when the fingerprint differs (or `--rebuild`); always run
  the freshness guard (step 5) regardless — cheap.

---

## Files touched

- `test/scripts/rebuild_sdk_and_pack.sh` — consumer registry; hash + rename tarball;
  rewrite each consumer's `file:` dep; per-consumer force re-extract; write
  `.sdk-fingerprint`; skip when unchanged.
- `test/scripts/set-sdk-dep.mjs` *(new)* — rewrite `@fhevm/sdk` `file:` dep in a
  consumer's `package.json`.
- `test/scripts/assert-fresh-sdk.mjs` *(new)* — installed-vs-tarball freshness guard.
- `test/browser-next/scripts/run.mjs` — fingerprint-based repack/install; call guard.
- `test/browser-next/globalSetup.ts` — call guard before specs.
- `test/manual-pack/package.json`, `test/browser-next/package.json` — `file:` dep
  rewritten by the script to the hashed tarball name (committed once; auto-updated
  thereafter).

## Minimal core vs full

- **Durable core (kills the bug class): steps 1–5.**
- **Performance refinement: step 6.**
- Smallest change that still fixes it: **2 + 3 + 4 + 5** (hashed name + rewrite +
  re-extract all consumers + guard).

## Verification

1. `bash test/scripts/rebuild_sdk_and_pack.sh --build-profile=dev` → one
   `fhevm-sdk-<ver>-<sha8>.tgz`; both consumers' deps point at it; both
   `node_modules/@fhevm/sdk` re-extracted.
2. `cd test/browser-next && FHEVM_TEST_THREADS=mt FHEVM_TEST_COOP=1 \
   npx playwright test specs/encrypt.spec.ts` → **pass** (`numberOfThreads=2`,
   proof emitted), proving the fix is actually exercised.
3. Negative test: hand-edit the installed `startWorkers.js`, run the guard →
   it must **fail loudly**.
4. Re-run with no SDK change → step 6 skips repack; guard still passes.

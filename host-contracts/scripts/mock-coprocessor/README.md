# mock-coprocessor

A long-running TypeScript daemon that imitates the production FHEVM
coprocessor for the **purposes of local testnet experimentation**. It polls
the deployed `FHEVMExecutor` (and `ConfidentialBridge`) on every configured
chain, replays each FHE operation in plaintext, and stores the result in a
shared SQLite DB.

Demo contracts that emit FHE ops (`FHECounter` is shipped here; `ConfidentialOFT`
also works) light up against this daemon without a real coprocessor.

> **Scope.** No real cryptography, no DVN. Operates on event logs +
> arithmetic + a small EIP-712-signed input-bundle builder that mirrors what
> the production relayer would emit. Suitable for end-to-end integration
> smoke tests; **not** suitable as a stand-in for a real coprocessor in any
> security context.

---

## Architecture

```
                          ┌───────────────────────────┐
  RPC: Sepolia ──poll──▶  │  chain-worker (sepolia)   │  ─┐
                          └───────────────────────────┘   │
                                                          ├─▶  MockDb (SQLite)
                          ┌───────────────────────────┐   │      handle → clearText
  RPC: Amoy    ──poll──▶  │  chain-worker (amoy)      │  ─┘
                          └───────────────────────────┘
```

- One **chain worker** per supported chain, polling `eth_getLogs` for the
  FHEVMExecutor + ConfidentialBridge addresses snapshotted in that chain's
  `<chain>/addresses/.env.host`.
- Logs are parsed against the same event ABI used by `test/coprocessorUtils.ts`
  (`abi/events.ts`) and dispatched to a handler:
  - `handlers/fhe-executor.ts` — full operator coverage (TrivialEncrypt,
    FheAdd/Sub/Mul/Div/Rem/BitAnd/BitOr/…/IfThenElse/Sum/IsIn/MulDiv/Rand/…).
    Ported from `test/coprocessorUtils.ts::insertHandleFromEvent`.
  - `handlers/bridge.ts` — on `HandleBridged` copies the source-chain
    clearText into the destination handle. Cross-chain races (chain B's
    `HandleBridged` seen before chain A's `TrivialEncrypt`) are resolved by a
    pending-mappings retry queue. Also handles `FallbackGrantedPlaintext`.
- A shared **SQLite** DB stores `(handle → clearText)` so the separate
  `pnpm mock:query` process can read them. Handles are globally unique by
  construction (RFC 008 §Handle uniqueness), so one table suffices. No block
  cursor is persisted — see "Live events only" below.

---

## Files

| File | Purpose |
|---|---|
| `index.ts` | CLI: `daemon`, `query <handle>`, `encrypt` |
| `service.ts` | Orchestrator: spawns one worker per chain, graceful shutdown |
| `chain-worker.ts` | Polls one chain, batches `eth_getLogs`, dispatches |
| `config.ts` | Per-chain config — RPC URL, addresses from `<chain>/addresses/.env.host`, polling knobs |
| `db.ts` | Thin sqlite3 wrapper, `ciphertexts` table |
| `input.ts` | Off-chain builder for `FHE.fromExternal` inputs (handles + EIP-712-signed `inputProof`); CLI shim for `pnpm mock:encrypt` |
| `abi/events.ts` | Event signatures for FHEVMExecutor + ConfidentialBridge |
| `handlers/fhe-executor.ts` | Plaintext derivation for each FHE operator |
| `handlers/bridge.ts` | Cross-chain `HandleBridged` propagation + retry queue |
| `mock-coprocessor.db` | (Runtime) SQLite file — gitignored |

---

## Quickstart

### 1. Pre-reqs

- ConfidentialBridge already deployed on every chain you want to watch
  (`task:deployBridge` + snapshot of `addresses/.env.host` into
  `<chain>/addresses/` — see `addresses/BRIDGE_DEPLOYMENT.md` §2.4).
- A funded `DEPLOYER_PRIVATE_KEY` (only needed if you'll also run the demo
  task that submits FHE txs).
- Optional: per-chain RPC overrides in `.env`:
  ```bash
  SEPOLIA_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/…
  POLYGON_AMOY_RPC_URL=https://polygon-amoy.infura.io/v3/…
  ```
  Defaults to public endpoints.

### 2. Start the daemon

```bash
pnpm mock:daemon
```

You should see output like:

```
[mock-coprocessor] starting service for 2 chain(s): sepolia(eid=40161), polygonAmoy(eid=40267)
[mock-coprocessor] db=/.../scripts/mock-coprocessor/mock-coprocessor.db
[mock-coprocessor:sepolia] starting: rpc=… executor=0x… bridge=0x…
[mock-coprocessor:sepolia] initialised at chain head 9301245 — only events from block 9301246 onwards will be processed
[mock-coprocessor:polygonAmoy] starting: rpc=… executor=0x… bridge=0x…
[mock-coprocessor:polygonAmoy] initialised at chain head 15403200 — only events from block 15403201 onwards will be processed
[mock-coprocessor:sepolia] processed blocks 9301246-9301246 (head=9301246): inserted=0 pending=0 skipped=0
…
```

**Live events only.** Each chain worker seeds its cursor at the chain's
current head on startup and processes only events submitted from then on.
The daemon does **not** catch up missed events across restarts — this is a
one-time-testing tool, not a historical indexer.

> **Submit your test transactions AFTER you've seen the
> `initialised at chain head N` line**, or they'll be skipped.

The poll loop processes 1k-block batches (`MOCK_COPROCESSOR_MAX_BLOCK_RANGE`)
and idles for `MOCK_COPROCESSOR_POLL_INTERVAL_MS` (default 5s) when at head.

`Ctrl-C` stops cleanly. The next start initialises at whatever block is at
head then — no resume.

> Need to wipe the accumulated ciphertexts from a previous session?
> `pnpm mock:reset` removes the SQLite file; the next `mock:daemon`
> creates a fresh one.

### 3. Smoke test with `FHECounter`

In another terminal:

```bash
# Deploy FHECounter on Sepolia
cp sepolia/addresses/{FHEVMHostAddresses.sol,.env.host} addresses/
RPC_URL=https://eth-sepolia.g.alchemy.com/v2/… \
  npx hardhat --network sepolia task:deployFHECounter
cp addresses/.env.host sepolia/addresses/.env.host

# Run 3 × add(7) and print the resulting handle
RPC_URL=https://eth-sepolia.g.alchemy.com/v2/… \
  npx hardhat --network sepolia task:runFHECounterDemo --amount 7 --times 3
```

The task prints something like:

```
Current encrypted value handle: 0x…
Expected plaintext (= 3 × 7): 21
Verify via the mock coprocessor once it catches up:
  pnpm mock:query 0x…
(should print: 21)
```

Wait ~one poll cycle, then run the suggested `pnpm mock:query 0x…` — it
should print `21`.

Repeat on Polygon Amoy:

```bash
cp polygonAmoy/addresses/{FHEVMHostAddresses.sol,.env.host} addresses/
RPC_URL=https://rpc-amoy.polygon.technology \
  npx hardhat --network polygonAmoy task:deployFHECounter
cp addresses/.env.host polygonAmoy/addresses/.env.host

RPC_URL=https://rpc-amoy.polygon.technology \
  npx hardhat --network polygonAmoy task:runFHECounterDemo --amount 4 --times 2
# → expects plaintext = 8
```

### 4. End-to-end OFT bridge flow (advanced)

After §5 of `addresses/BRIDGE_DEPLOYMENT.md` (both OFTs deployed and wired),
the daemon's bridge handler propagates clear-texts across chains:

1. On Sepolia call `ConfidentialOFT.send(40267, …, amount, recipient, …)`.
2. Bridge emits `BridgeHandle` on Sepolia → daemon notes the source-side
   ciphertext (already populated by the source-side `FheSub` event in `_burn`).
3. Bridge emits `HandleBridged` on Amoy with `(srcHandle, dstHandle)` → daemon
   copies `srcHandle`'s clearText to `dstHandle` in the shared DB.
4. On Amoy the OFT's `_mint` runs a new `FheAdd(oldBalance, dstHandle)` →
   daemon resolves the new balance from already-known operands.
5. `pnpm mock:query <newBalanceHandle>` returns the expected balance.

### 5. Encrypted user inputs (`FHE.fromExternal`)

Dapp functions that take `externalEuintXX` + `bytes inputProof` parameters
(e.g. `ConfidentialOFT.mint`) need a relayer to produce the bundle off-chain.
`pnpm mock:encrypt` does the equivalent locally:

```bash
pnpm mock:encrypt \
  --contract     0xYourContract \
  --user         0xYourEOA \
  --type         euint64 \
  --value        1000 \
  --host-chain-id 11155111
# handle=0x…
# inputProof=0x…
```

Internally it:

1. Builds the per-input ciphertext blob (`[FheType byte] || BE(value) || random32`),
   keccak-hashes the concatenation, and derives the handle byte-by-byte per
   `test/fhevmjsMocked.ts` — including the host chain id at bytes 22–29 and
   the `FheType` tag at byte 30.
2. Packs the on-chain `inputProof` layout: `numHandles | numSigners |
   handles | signatures | extraData`.
3. EIP-712-signs the `CiphertextVerification` typed-data (over `(ctHandles,
   userAddress, contractAddress, contractChainId, extraData)`) with the
   coprocessor signer keys from `PRIVATE_KEY_COPROCESSOR_ACCOUNT_*`. Domain
   is `(name="InputVerification", version="1", chainId=$CHAIN_ID_GATEWAY,
   verifyingContract=$INPUT_VERIFICATION_ADDRESS)`.
4. **Inserts the cleartext into the mock DB** so the daemon's `VerifyInput`
   handler resolves the same handle once the contract tx is mined.

If you set `COPROCESSOR_SIGNER_ADDRESS_*` alongside the private keys, the
CLI cross-checks each derived address matches and refuses to build a bundle
the on-chain `InputVerifier` would reject.

**Required env vars** (in addition to the per-chain RPC URLs):
`CHAIN_ID_GATEWAY`, `INPUT_VERIFICATION_ADDRESS`, `NUM_COPROCESSORS`, and
`PRIVATE_KEY_COPROCESSOR_ACCOUNT_0..N`. These are the same values used at
`task:deployInputVerifier` time.

---

## Configuration

All knobs are env vars (no config file). Defaults are sensible for testnet.

| Env var | Default | Purpose |
|---|---|---|
| `SEPOLIA_RPC_URL` | `https://sepolia.drpc.org` | Sepolia JSON-RPC |
| `POLYGON_AMOY_RPC_URL` | `https://rpc-amoy.polygon.technology` | Amoy JSON-RPC |
| `MOCK_COPROCESSOR_POLL_INTERVAL_MS` | `5000` | Sleep between poll cycles when at head |
| `MOCK_COPROCESSOR_MAX_BLOCK_RANGE` | `1000` | Per-batch block range (most RPCs cap at 1k–10k) |
| `MOCK_COPROCESSOR_ERROR_BACKOFF_MS` | `10000` | Backoff after a failed poll cycle |
| `MOCK_COPROCESSOR_BRIDGE_RETRY_LIMIT` | `20` | Retries per pending HandleBridged before dropping |
| `MOCK_COPROCESSOR_DB_PATH` | `scripts/mock-coprocessor/mock-coprocessor.db` | SQLite file path |

To add a new chain: extend `CHAINS` in `config.ts` with `(name, chainId, lzEid, rpcEnvVar, rpcDefault, addressesEnv)` and add a matching `<chain>/addresses/.env.host` snapshot.

---

## Limitations

- **No input-verifier flow.** `VerifyInput` events expect the handle to be
  pre-populated by the relayer; we throw if it isn't. Use `TrivialEncrypt`
  flows only (the `FHECounter` smoke test is built on this).
- **No public-decryption oracle.** The mock just stores the plaintext; nothing
  posts it back on-chain. Use `pnpm mock:query` (off-chain) to read it.
- **Reorgs are not handled.** If a reorged-out tx had FHE events, we'll have
  stale entries in the DB. Wipe with `pnpm mock:reset` if you suspect this.
- **`FheRand` non-determinism.** Each restart re-rolls `FheRand` results to
  fresh random values (we `INSERT OR REPLACE`). Don't rely on rand handles
  surviving across daemon restarts.

---

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| `addresses snapshot not found at …` | A `<chain>/addresses/.env.host` is missing | Run `task:deployAllHostContracts` on that chain and snapshot — see `addresses/BRIDGE_DEPLOYMENT.md` §2.4. |
| `Operand handle not found in mock DB` | The daemon started AFTER the operand-producing tx was mined (live-only mode skips it) | `Ctrl-C` the daemon, `pnpm mock:reset` to wipe ciphertexts, restart the daemon, then re-submit the producing tx. |
| Lots of `pending` log lines | Cross-chain bridge events outpacing the source side | Normal during catch-up; resolves automatically. If it persists, raise `MOCK_COPROCESSOR_BRIDGE_RETRY_LIMIT`. |
| `eth_getLogs` rate-limit / 429 | RPC provider cap | Lower `MOCK_COPROCESSOR_MAX_BLOCK_RANGE`, increase `MOCK_COPROCESSOR_POLL_INTERVAL_MS`, or use a paid RPC. |
| `pnpm mock:query <handle>` exits 1 | Daemon hasn't seen the producing event yet, or handle is wrong case | Wait one poll cycle (~5s default), or check the handle prefix/suffix matches what the tx emitted. |

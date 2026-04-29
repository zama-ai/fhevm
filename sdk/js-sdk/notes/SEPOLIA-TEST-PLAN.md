# FHE Test Suite — Testing Plan

## Goal

End-to-end testing of the SDK against live chains (sepolia / mainnet) using a pre-deployed `FHETest.sol` contract.

This plan is not meant to be fully exhaustive — it covers the foundational setup and key test scenarios. Additional tests can be added incrementally as needed.

These tests also serve as a specification for the SDK's public API — they exercise and validate the exported surface, helping define what the public API should look like from a consumer's perspective.

Some tests also double as **CLI utilities** — runnable individually to inspect on-chain state. For example, running the connectivity test in isolation can display the current encrypted values stored in `FHETest.sol`, coprocessor config, etc. This is useful for debugging and general chain exploration outside of a full test run.

The idea is that tests like `connectivity.test.ts` can be run standalone (`vitest run connectivity`) as diagnostic tools, not just assertions.

## Test folder

`./test/fheTest`

Tests should be spread across separate files, one per phase / concern (e.g. `connectivity.test.ts`, `client.test.ts`, `encrypt.test.ts`, `publicDecrypt.test.ts`, `userDecrypt.test.ts`, `errors.test.ts`). This keeps files focused, makes it easy to run a single phase in isolation, and avoids monolithic test files.

Each test file should include a header comment with the CLI command to run it standalone, e.g.:

```ts
// npx vitest run --config test/fheTest/vitest.config.ts connectivity
```

## Environment & Secrets

- `test/.env` — shared secrets (gitignored):
  - `MNEMONIC` — HD wallet mnemonic for signing transactions / EIP-712
  - `ZAMA_FHEVM_API_KEY` — API key passed as `auth` in SDK config
- `test/.env.<chain>` (e.g. `.env.sepolia`, `.env.mainnet`) — chain-specific, public:
  - `RPC_URL` — JSON-RPC endpoint
- The setup file must **fail fast** with a clear message if `MNEMONIC` or `ZAMA_FHEVM_API_KEY` are missing.

## FHE Encryption Key Cache

The FHE public encryption key is ~50MB. To avoid re-downloading on every run:

- Cache location: `test/fheTest/.keys/<chain>/key.json` (JSON — key bytes + metadata such as key source, version, etc.)
- On setup: if cached file exists, load and deserialize from disk. Otherwise let the SDK fetch it, then serialize key + metadata to JSON and write to disk.
- Cache read/write happens in **globalSetup** (once per vitest run) to avoid parallel race conditions.
- Clear cache: a `clearKeyCache(chain?: "sepolia" | "mainnet")` TS function (deletes one chain or all). Also exposed as npm script `"test:clear-keys": "rm -rf test/fheTest/.keys"`.
- Write/Read to/from cache: a `readKeyFromCache(chain: "sepolia" | "mainnet"): FheEncryptionKeyBytes | undefined` TS function and a `writeKeyToCache(fheEncryptionKeyBytes:FheEncryptionKeyBytes, chain: "sepolia" | "mainnet")`.
- Check cache: `hasKeyInCache(chain: "sepolia" | "mainnet"): boolean` — checks if the cache file exists without deserializing.
- `test/fheTest/.keys/` must be in `.gitignore`.

## Chain Selection

Single vitest config with `CHAIN` env var (defaults to `sepolia`):

```
test/fheTest/vitest.config.ts
```

npm scripts:

```json
"test:sepolia": "CHAIN=sepolia vitest -c test/fheTest/vitest.config.ts",
"test:mainnet": "CHAIN=mainnet vitest -c test/fheTest/vitest.config.ts",
"test:clear-keys": "rm -rf test/fheTest/.keys"
```

The config reads `CHAIN`, loads `test/.env` + `test/.env.<chain>`, and selects the correct address from `FHETestAddresses`.

## Test Config Shape

```ts
type FheTestConfig = {
  readonly chain: "sepolia" | "mainnet";
  readonly wallet: ethers.HDNodeWallet;
  readonly signer: ethers.Signer; // wallet connected to provider
  readonly provider: ethers.JsonRpcProvider;
  readonly zamaApiKey: string;
  readonly fheTestAddress: string; // from abi.ts
  readonly fheTestContract: ethers.Contract; // bound to signer + ABI + address
  readonly fheTestAbi: typeof FHETestABI;
};
```

The config provides **ingredients only** — no pre-built SDK client or pre-fetched encryption key.
Each test creates its own client via `createFhevmClient`, `createFhevmBaseClient`, etc., so that
client creation and key fetching are themselves exercised by the tests (Phase 1+).

The FHE encryption key cache (`test/fheTest/.keys/`) is a **separate utility** that tests can opt into
(e.g. by passing cached key bytes to the SDK config) but is not injected into the config by default.

Built once in a shared `beforeAll` helper, exposed to all tests.

## .gitignore

Verify the following are gitignored:

- `test/.env` (secrets)
- `test/fheTest/.keys/` (cached key blobs)
- `.env.sepolia` and `.env.mainnet` are safe to commit (public RPC URLs only)

## Timeouts

On-chain tests are slow (block confirmation, relayer). Config should set:

- `testTimeout: 120_000`
- `hookTimeout: 120_000`
- `retry: 0` (retrying wastes gas)

## Contract ABI & Addresses

Stored in `test/fheTest/abi.ts`. Addresses:

- mainnet: `0x7553CB9124f974Ee475E5cE45482F90d5B6076BC`
- testnet (sepolia): `0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241`

## Test Cases

### Phase 0 — Connectivity (`connectivity.test.ts`)

Sanity checks that the chain, RPC, and contract are reachable before any SDK logic runs.
Also serves as a CLI diagnostic — displays on-chain state when run standalone.

- Provider connects to the RPC endpoint
- Wallet has a non-zero balance (fail fast if empty — all subsequent txs would fail)
- `fheTestContract.CONTRACT_NAME()` returns a string (proves ABI + address + provider work)
- `fheTestContract.confidentialProtocolId()` returns a value
- `fheTestContract.getCoprocessorConfig()` returns valid addresses
- Read and display stored encrypted values: `getEbool()`, `getEuint8()`, `getEuint16()`, `getEuint32()`, `getEuint64()`, `getEuint128()`, `getEuint256()`, `getEaddress()`

### Phase 1 — Client creation & public API surface (`client.test.ts`)

- Import and call `createFhevmBaseClient` (minimalist, no modules) — proves exports resolve and basic client works
- Import and call `createFhevmEncryptClient`, `createFhevmDecryptClient`, `createFhevmClient` — proves all client factories work
- `createFhevmBaseClient` + `.extend(encryptActions)` + `.extend(decryptActions)` — proves composability via `extend()`
- Create `fhevmBaseClient` + fetch `FheEncryptionKey` (verifies key fetch + cache works)
- Doubles as living documentation of the expected public API surface

### Phase 2 — Encrypt (`encrypt.test.ts`)

- Encrypt a value for each FHE type (euint8/16/32/64/128/256, ebool, eaddress)
- Send encrypted value on-chain: `encrypt(euint8, value)` -> call `addEuint8(handle, proof)` -> assert tx success
- Error case: invalid proof rejected

### Phase 3 — Public Decrypt (`publicDecrypt.test.ts`)

- `randEuint8()` -> `makePubliclyDecryptableEuint8()` -> `getEuint8()` -> SDK `publicDecrypt` -> verify cleartext
- Cover all FHE types

### Phase 4 — User Decrypt (`userDecrypt.test.ts`)

- Encrypt -> store on-chain -> sign decryption permit -> decrypt -> verify cleartext matches original
- Cover all FHE types

### Phase 5 — Error Cases (`errors.test.ts`)

- Unauthorized decrypt (no permit)
- Invalid proof submission
- Missing / wrong API key

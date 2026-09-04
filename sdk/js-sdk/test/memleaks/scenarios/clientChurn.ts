import { createFhevmEncryptClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '../../../src/ethers/index.js';
import { globalFheEncryptionKeyCache } from '../../../src/core/key/FheEncryptionKeyCache-p.js';
import { createLogger, encryptTestCases } from '../../fheTest/setupCommon.js';
import { createTfheMemoryReader } from '../support/wasmMemory.js';
import type { Scenario } from './scenario.js';

// ---------------------------------------------------------------------------
// Mode B: force a fresh public-key + CRS deserialize every iteration
// ---------------------------------------------------------------------------
//
// Targets `deserializeFheEncryptionPublicKey`/`deserializeFheEncryptionCrs`
// (src/core/modules/encrypt/module/api-p.ts:438-485), which wrap native
// wasm-bindgen objects (`CompactPkeCrs`, `TfheCompactPublicKey`) in
// `TfheCompactPkeCrsImpl`/`TfheCompactPublicKeyImpl` — neither wrapper class
// calls `.free()` on its native handle, unlike `buildWithProofPacked`, which
// frees its ciphertext-list/builder objects in a `finally`.
//
// This is NOT a guaranteed leak, and this scenario exists specifically to
// measure it rather than assume it: the vendored tfhe glue is a `--weak-refs`
// wasm-bindgen build, and both `CompactPkeCrs` and `TfheCompactPublicKey`
// register themselves with a `FinalizationRegistry` at construction (see
// `CompactPkeCrsFinalization`/`TfheCompactPublicKeyFinalization` in
// src/wasm/tfhe/v1.6.2/tfhe.js). So even without an explicit `.free()`, the
// underlying WASM memory *can* still be reclaimed once the JS wrapper becomes
// unreachable and a GC pass runs its pending finalizers. Whether that
// actually happens in practice — promptly, at all, or only once memory
// pressure forces it — is exactly the open question this scenario is built
// to answer empirically.
//
// A more basic pitfall this scenario deliberately avoids: naively creating
// "N fresh clients" against the same chain would NOT exercise this path more
// than once. `globalFheEncryptionKeyCache`
// (src/core/key/FheEncryptionKeyCache-p.ts) is a process-wide singleton keyed
// by relayerUrl with "first write wins" semantics — the key is deserialized
// once per relayerUrl for the life of the process, and every later client
// just awaits the same cached entry. Its own doc comment says as much: "To
// force a re-fetch, call `remove(relayerUrl)` before `ensureBytes`." — so
// this scenario does exactly that before every iteration, which is also what
// makes the old entry's wasm objects unreachable (and thus GC/finalizer
// eligible) in the first place.

export const clientChurnScenario: Scenario = {
  name: 'clientChurn',
  description:
    'Evicts the global FHE key cache and creates a fresh client every iteration, forcing one public-key+CRS deserialize per iteration.',
  // A real relayer round-trip plus a full key deserialize per iteration is far
  // heavier than clientReuse's steady-state loop — default to fewer iterations.
  defaultIterations: 200,
  defaultIterationsDuration: '~15 min',
  setup: async ({ config }) => {
    // Process-wide singleton: when running multiple scenarios in one `main.ts`
    // invocation (e.g. `--scenario all`), only the first scenario's setup()
    // may call this — a later call with a fresh `createLogger()` reference
    // would throw even though the effective config is identical.
    if (!hasFhevmRuntimeConfig()) {
      setFhevmRuntimeConfig({
        auth: { type: 'ApiKeyHeader', value: config.zamaApiKey },
        logger: createLogger(console.log, config.chainName),
      });
    }

    const tfheVersion =
      config.moduleVersions !== undefined && config.moduleVersions !== 'auto' ? config.moduleVersions.tfhe : undefined;
    const relayerUrl = config.fhevmChain.fhevm.relayerUrl;
    const options = config.moduleVersions !== undefined ? { moduleVersions: config.moduleVersions } : undefined;

    const readTfheMemory = await createTfheMemoryReader(tfheVersion);

    const uint8Case = encryptTestCases.find((tc) => tc.type === 'uint8');
    if (uint8Case === undefined) {
      throw new Error('encryptTestCases has no uint8 entry.');
    }

    const iterate = async (): Promise<void> => {
      globalFheEncryptionKeyCache.remove(relayerUrl);

      const client = createFhevmEncryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options,
      });
      await client.ready;

      await client.encryptValue({
        contractAddress: config.fheTestAddress,
        userAddress: config.wallet.address,
        value: uint8Case,
      });
    };

    return { iterate, readTfheMemory };
  },
};

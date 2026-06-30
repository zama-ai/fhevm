import type { TfheVersion } from '../../../src/wasm/tfhe/loadTfheLib.js';
import type { FhevmChain } from '../../../src/core/types/fhevmChain.js';
import {
  DUMMY_RELAYER_BASE_URL,
  TFHE_VERSIONS,
  TKMS_VERSIONS,
  assertTfheModuleReady,
  assertTkmsModuleReady,
  assertWellFormedProof,
  attempt,
  buildUint64Proof,
  errorMessage,
  initAllModules,
  loadAndCacheKey,
  log,
  makeDummyChain,
  runSmokePage,
  setupMultiWasmRuntime,
} from './multiWasmHarness.js';

/**
 * A chain is defined by a unique relayer URL, the TFHE module version it requires,
 * and the serialized PubKey/CRS format it serves (a key.<keyTfheVersion>.json file).
 *
 * The TFHE-rs serialization format is forward-compatible but not backward-compatible,
 * so a chain runs only when its module version can deserialize the key it serves:
 *  - a key serialized by an older version loads on any newer module,
 *  - a key serialized by a newer version fails on an older module.
 */
type ChainDefinition = {
  /** The TFHE module version that deserializes the key and builds the proof. */
  readonly tfheVersion: TfheVersion;
  /** The serialized key format served (test/keys/key.<keyTfheVersion>.json). */
  readonly keyTfheVersion: string;
  /** Whether a mini encryption on this chain is expected to succeed. */
  readonly shouldRun: boolean;
  /** Self-contained dummy chain with a relayer URL unique to (tfheVersion, keyTfheVersion). */
  readonly chain: FhevmChain;
};

function defineChainDefinition(tfheVersion: TfheVersion, keyTfheVersion: string, shouldRun: boolean): ChainDefinition {
  // The relayer URL is unique per (module, key) so each chain gets its own cache
  // slot (the global key cache is keyed by relayer URL, first-write-wins per slot).
  const chain = makeDummyChain(`${DUMMY_RELAYER_BASE_URL}/${tfheVersion}/${keyTfheVersion}`);
  return { tfheVersion, keyTfheVersion, shouldRun, chain };
}

const CHAIN_DEFINITIONS: readonly ChainDefinition[] = [
  defineChainDefinition('1.5.3', '1.4.0-alpha.3', true),
  defineChainDefinition('1.6.1', '1.4.0-alpha.3', true),
  defineChainDefinition('1.5.3', '1.5.4', true),
  defineChainDefinition('1.6.1', '1.5.4', true),
  defineChainDefinition('1.5.3', '1.6.1', false),
  defineChainDefinition('1.6.1', '1.6.1', true),
];

void runSmokePage(async () => {
  const runtime = setupMultiWasmRuntime();

  log('Initializing TFHE and TKMS versions concurrently...');
  await initAllModules(runtime);
  log('[PASS] All modules initialized');

  for (const tfheVersion of TFHE_VERSIONS) {
    await assertTfheModuleReady(runtime, tfheVersion);
  }

  // Battle-test concurrency: run every chain's prime-then-encrypt in parallel,
  // exercising concurrent cache population (each chain has a unique slot),
  // interleaved deserialize, and shared per-version TFHE worker pools at once.
  log(`Running ${CHAIN_DEFINITIONS.length} per-chain mini encryptions concurrently...`);
  const results = await Promise.all(
    CHAIN_DEFINITIONS.map(async (definition) => {
      await loadAndCacheKey(runtime, definition.chain, definition.keyTfheVersion);
      const outcome = await attempt(async () => {
        const zkProof = await buildUint64Proof(runtime, definition.chain, definition.tfheVersion);
        assertWellFormedProof(zkProof, 1);
      });
      return { definition, outcome };
    }),
  );

  // Assert and log in definition order so the output stays deterministic even
  // though the encryptions ran concurrently.
  for (const { definition, outcome } of results) {
    const label = `module ${definition.tfheVersion} + key.${definition.keyTfheVersion}`;
    if (outcome.ok !== definition.shouldRun) {
      if (definition.shouldRun) {
        throw new Error(`Expected ${label} to run, but it failed: ${errorMessage(outcome.error)}`);
      }
      throw new Error(`Expected ${label} to fail, but the encryption ran successfully.`);
    }
    log(`[PASS] ${label} ${outcome.ok ? 'ran' : 'failed'} as expected`);
  }
  log('[PASS] Concurrent per-chain mini encryptions verified');

  for (const tkmsVersion of TKMS_VERSIONS) {
    await assertTkmsModuleReady(runtime, tkmsVersion);
  }
});

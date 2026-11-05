import type { WasmCounts } from './wasmInstanceCounter.js';
import { getWasmCounts } from './wasmInstanceCounter.js';
import {
  createFhevmBaseClient,
  createFhevmClient,
  createFhevmDecryptClient,
  createFhevmEncryptClient,
  createFhevmEncryptWithPadClient,
} from '../../../src/ethers/index.js';
import {
  DUMMY_RELAYER_BASE_URL,
  assertTfheModuleReady,
  assertTkmsModuleReady,
  log,
  runSmokePage,
  setSmokeRuntimeConfig,
  setupDummyChain,
} from './smokeHarness.js';

/** The newest serialized key format in test/keys that the canonical TFHE module deserializes. */
const KEY_TFHE_VERSION = '1.6.1';

const ZERO = { tfhe: 0, tkms: 0, other: 0 };
const ONE_EACH = { tfhe: 1, tkms: 1, other: 0 };

function assertSingleInstance(counts: WasmCounts, step: string): void {
  if (JSON.stringify(counts) !== JSON.stringify({ compiled: ONE_EACH, instantiated: ONE_EACH })) {
    throw new Error(`${step}: expected one TFHE and one TKMS module, got ${JSON.stringify(counts)}`);
  }
  log(`[PASS] ${step}: ${JSON.stringify(counts)}`);
}

void runSmokePage(async () => {
  setSmokeRuntimeConfig();

  // The encrypt clients prefetch the FHE key at init: it is provided, not fetched from the (dummy) relayer, and
  // checked for real against the dummy chain's KMSGeneration.
  const { chain, provider, fheEncryptionKey } = await setupDummyChain(
    `${DUMMY_RELAYER_BASE_URL}/clients`,
    KEY_TFHE_VERSION,
  );

  const clients = {
    base: createFhevmBaseClient({ chain, provider }),
    decrypt: createFhevmDecryptClient({ chain, provider }),
    encrypt: createFhevmEncryptClient({ chain, provider, options: { fheEncryptionKey } }),
    encryptWithPad: createFhevmEncryptWithPadClient({ chain, provider }),
    full: createFhevmClient({ chain, provider, options: { fheEncryptionKey } }),
  };
  log(`Created ${Object.keys(clients).length} clients: ${Object.keys(clients).join(', ')}`);
  const before = getWasmCounts();
  if (JSON.stringify(before) !== JSON.stringify({ compiled: ZERO, instantiated: ZERO })) {
    throw new Error(`Expected no WASM module before init, got ${JSON.stringify(before)}`);
  }
  log('[PASS] No WASM module compiled before init');

  // Every client initializes at once: the concurrent first calls must share one TFHE and one TKMS module.
  await Promise.all(Object.values(clients).map((c) => c.ready));
  log('[PASS] All clients ready');
  assertSingleInstance(getWasmCounts(), 'after concurrent init');

  // Awaiting ready again is a no-op.
  await Promise.all(Object.values(clients).map((c) => c.ready));
  assertSingleInstance(getWasmCounts(), 'after a second init');

  await assertTfheModuleReady(clients.full.runtime, clients.full.tfheVersion);
  await assertTkmsModuleReady(clients.full.runtime, clients.full.tkmsVersion);
  assertSingleInstance(getWasmCounts(), 'after using both modules');
});

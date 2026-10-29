import type { ethers as EthersT } from 'ethers';
import type { WasmCounts } from './wasmInstanceCounter.js';
import { AbiCoder } from 'ethers';
import { getWasmCounts } from './wasmInstanceCounter.js';
import {
  createFhevmBaseClient,
  createFhevmClient,
  createFhevmDecryptClient,
  createFhevmEncryptClient,
  createFhevmEncryptWithPadClient,
} from '../../../src/ethers/index.js';
import { getEthersRuntime } from '../../../src/ethers/internal/runtime.js';
import {
  DUMMY_RELAYER_BASE_URL,
  TFHE_VERSION,
  TKMS_VERSION,
  assertTfheModuleReady,
  assertTkmsModuleReady,
  loadAndCacheKey,
  log,
  makeDummyChain,
  runSmokePage,
  setupSmokeRuntime,
} from './smokeHarness.js';

/** The newest serialized key format in test/keys that the canonical TFHE module deserializes. */
const KEY_TFHE_VERSION = '1.6.1';

/** Host contract versions of the v1.0.0 stack, as each `getVersion()` returns them. */
const HOST_CONTRACT_VERSIONS = ['ACL v0.6.0', 'InputVerifier v0.3.0', 'KMSVerifier v0.5.0'] as const;

/**
 * An offline ethers runner: answers `getVersion()` for the chain's host contracts, so `client.ready` resolves the
 * frozen context without an RPC node. Any other call fails the test.
 */
function createVersionOnlyRunner(versionsByAddress: ReadonlyMap<string, string>): EthersT.ContractRunner {
  return {
    provider: null,
    call: (tx: { readonly to?: unknown }): Promise<string> => {
      const version = versionsByAddress.get(String(tx.to).toLowerCase());
      if (version === undefined) {
        return Promise.reject(new Error(`Unexpected call to ${String(tx.to)}`));
      }
      return Promise.resolve(AbiCoder.defaultAbiCoder().encode(['string'], [version]));
    },
  } as unknown as EthersT.ContractRunner;
}

const ZERO = { tfhe: 0, tkms: 0, other: 0 };
const ONE_EACH = { tfhe: 1, tkms: 1, other: 0 };

function assertSingleInstance(counts: WasmCounts, step: string): void {
  if (JSON.stringify(counts) !== JSON.stringify({ compiled: ONE_EACH, instantiated: ONE_EACH })) {
    throw new Error(`${step}: expected one TFHE and one TKMS module, got ${JSON.stringify(counts)}`);
  }
  log(`[PASS] ${step}: ${JSON.stringify(counts)}`);
}

void runSmokePage(async () => {
  const runtime = setupSmokeRuntime();

  const chain = makeDummyChain(`${DUMMY_RELAYER_BASE_URL}/clients`);
  const { acl, inputVerifier, kmsVerifier } = chain.fhevm.contracts;
  const provider = createVersionOnlyRunner(
    new Map(
      [acl, inputVerifier, kmsVerifier].map((c, i) => [c.address.toLowerCase(), HOST_CONTRACT_VERSIONS[i] as string]),
    ),
  );
  // The encrypt clients prefetch the FHE key at init: serve it from the cache, not the (dummy) relayer.
  await loadAndCacheKey(getEthersRuntime(), chain, KEY_TFHE_VERSION);

  const clients = {
    base: createFhevmBaseClient({ chain, provider }),
    decrypt: createFhevmDecryptClient({ chain, provider }),
    encrypt: createFhevmEncryptClient({ chain, provider }),
    encryptWithPad: createFhevmEncryptWithPadClient({ chain, provider }),
    full: createFhevmClient({ chain, provider }),
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

  await assertTfheModuleReady(runtime, TFHE_VERSION);
  await assertTkmsModuleReady(runtime, TKMS_VERSION);
  assertSingleInstance(getWasmCounts(), 'after using both modules');
});

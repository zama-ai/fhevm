import { createFhevmDecryptClient, createFhevmEncryptClient } from '../../../src/ethers/index.js';
import {
  DUMMY_RELAYER_BASE_URL,
  assertTfheModuleReady,
  assertTkmsModuleReady,
  assertWellFormedProof,
  attempt,
  errorMessage,
  generateUint64Proof,
  log,
  runSmokePage,
  setSmokeRuntimeConfig,
  setupDummyChain,
} from './smokeHarness.js';

/**
 * A chain is defined by a unique relayer URL and the serialized PubKey/CRS format
 * it serves as its active key (a key.<keyTfheVersion>.json file).
 *
 * The TFHE-rs serialization format is forward-compatible but not backward-compatible,
 * so a chain runs only when the TFHE module can deserialize the key it serves:
 *  - a key serialized by an older version loads on any newer module,
 *  - a key serialized by a newer version fails on an older module.
 */
type ChainDefinition = {
  /** The serialized key format served (test/keys/key.<keyTfheVersion>.json). */
  readonly keyTfheVersion: string;
  /** Whether a mini encryption on this chain is expected to succeed. */
  readonly shouldRun: boolean;
  /** Relayer URL unique to the key, so each chain gets its own key cache slot. */
  readonly relayerUrl: string;
};

function defineChainDefinition(keyTfheVersion: string, shouldRun: boolean): ChainDefinition {
  return { keyTfheVersion, shouldRun, relayerUrl: `${DUMMY_RELAYER_BASE_URL}/${keyTfheVersion}` };
}

const CHAIN_DEFINITIONS: readonly ChainDefinition[] = [
  defineChainDefinition('1.4.0-alpha.3', true),
  defineChainDefinition('1.5.4', true),
  defineChainDefinition('1.6.1', true),
];

void runSmokePage(async () => {
  setSmokeRuntimeConfig();

  // One encrypt client per chain, each checking its chain's key for real against the chain's KMSGeneration,
  // and a decrypt client for the TKMS module. Every client shares the same TFHE and TKMS modules.
  const chains = await Promise.all(
    CHAIN_DEFINITIONS.map((definition) => setupDummyChain(definition.relayerUrl, definition.keyTfheVersion)),
  );
  const encryptClients = chains.map(({ chain, provider, fheEncryptionKey }) =>
    createFhevmEncryptClient({ chain, provider, options: { fheEncryptionKey } }),
  );
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  const decryptClient = createFhevmDecryptClient({ chain: chains[0]!.chain, provider: chains[0]!.provider });
  log(`Created ${encryptClients.length} encrypt clients and 1 decrypt client`);

  // Battle-test concurrency: every chain's init (key check + cache population, each chain has a unique slot)
  // and prime-then-encrypt run in parallel, interleaving deserialize against the shared TFHE worker pool.
  log(`Running ${CHAIN_DEFINITIONS.length} per-chain mini encryptions concurrently...`);
  const [results] = await Promise.all([
    Promise.all(
      CHAIN_DEFINITIONS.map(async (definition, i) => {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        const client = encryptClients[i]!;
        const outcome = await attempt(async () => {
          await client.init();
          assertWellFormedProof(await generateUint64Proof(client), 1);
        });
        return { definition, client, outcome };
      }),
    ),
    decryptClient.init(),
  ]);

  // Assert and log in definition order so the output stays deterministic even
  // though the encryptions ran concurrently.
  for (const { definition, client, outcome } of results) {
    const label = `key.${definition.keyTfheVersion}`;
    if (outcome.ok !== definition.shouldRun) {
      if (definition.shouldRun) {
        throw new Error(`Expected ${label} to run, but it failed: ${errorMessage(outcome.error)}`);
      }
      throw new Error(`Expected ${label} to fail, but the encryption ran successfully.`);
    }
    log(`[PASS] ${label} ${outcome.ok ? `ran on module ${client.tfheVersion}` : 'failed'} as expected`);
  }
  log('[PASS] Concurrent per-chain mini encryptions verified');

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  const encryptClient = encryptClients[0]!;
  await assertTfheModuleReady(encryptClient.runtime, encryptClient.tfheVersion);
  await assertTkmsModuleReady(decryptClient.runtime, decryptClient.tkmsVersion);
});

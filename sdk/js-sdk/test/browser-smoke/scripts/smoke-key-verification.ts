import type { DummyChain, DummyChainOptions } from './smokeHarness.js';
import { createFhevmEncryptClient } from '../../../src/ethers/index.js';
import {
  DUMMY_RELAYER_BASE_URL,
  attempt,
  errorMessage,
  log,
  runSmokePage,
  setSmokeRuntimeConfig,
  setupDummyChain,
} from './smokeHarness.js';

/**
 * Negative checks of the FHE key verification: a real encrypt client, given the
 * real test key, must refuse it whenever the chain's KMSGeneration does not vouch
 * for it, and keep refusing it without reading KMSGeneration again.
 */

/** The newest serialized key format in test/keys that the canonical TFHE module deserializes. */
const KEY_TFHE_VERSION = '1.6.1';

/** A digest no key has: 32 zero bytes. */
const WRONG_DIGEST = `0x${'00'.repeat(32)}`;

/** The KMSGeneration reads that check a key, as a set. */
const KEY_CHECK_READS = ['getActiveCrsId', 'getActiveKeyId', 'getCrsMaterials', 'getKeyMaterials', 'getVersion'];

/** A dummy chain of its own for `name`: a relayer URL of its own, so a key cache slot of its own. */
function setupCase(name: string, options?: DummyChainOptions): Promise<DummyChain> {
  return setupDummyChain(`${DUMMY_RELAYER_BASE_URL}/key-verification/${name}`, KEY_TFHE_VERSION, options);
}

function createClient({ chain, provider, fheEncryptionKey }: DummyChain) {
  return createFhevmEncryptClient({ chain, provider, options: { fheEncryptionKey } });
}

function assertError(error: unknown, name: string, message: RegExp, step: string): void {
  if (!(error instanceof Error) || error.name !== name || !message.test(error.message)) {
    throw new Error(`${step}: expected ${name} matching ${String(message)}, got ${errorMessage(error)}`);
  }
}

/** The client accepts the key: the control case, so the failures below come from what KMSGeneration serves. */
async function expectKeyAccepted(name: string): Promise<void> {
  const dummy = await setupCase(name);
  await createClient(dummy).ready;
  const reads = [...new Set(dummy.kmsGenerationCalls)].sort();
  if (JSON.stringify(reads) !== JSON.stringify(KEY_CHECK_READS)) {
    throw new Error(`${name}: expected the key check reads ${KEY_CHECK_READS.join(', ')}, got ${reads.join(', ')}`);
  }
  log(`[PASS] ${name}: key accepted after ${dummy.kmsGenerationCalls.length} KMSGeneration reads`);
}

/**
 * The client refuses the key with a FheEncryptionKeyVerificationError, and a
 * retry fails with the same error, without reading KMSGeneration again.
 */
async function expectKeyRejected(name: string, options: DummyChainOptions, message: RegExp): Promise<void> {
  const dummy = await setupCase(name, options);
  const client = createClient(dummy);

  const first = await attempt(() => client.ready);
  if (first.ok) {
    throw new Error(`${name}: expected the key to be rejected, but the client became ready.`);
  }
  assertError(first.error, 'FheEncryptionKeyVerificationError', message, name);
  const readsAfterFirst = dummy.kmsGenerationCalls.length;
  if (readsAfterFirst === 0) {
    throw new Error(`${name}: expected the key to be checked against KMSGeneration, but no read was made.`);
  }
  log(`[PASS] ${name}: rejected (${errorMessage(first.error)})`);

  // A failed init is retried by the next call; a key that failed the check keeps failing, with no RPC call.
  const retry = await attempt(() => client.init());
  if (retry.ok || retry.error !== first.error) {
    throw new Error(`${name}: expected the retry to fail with the same error, got ${String(retry.error)}`);
  }
  if (dummy.kmsGenerationCalls.length !== readsAfterFirst) {
    throw new Error(
      `${name}: expected no KMSGeneration read on retry, got ${dummy.kmsGenerationCalls.slice(readsAfterFirst).join(', ')}`,
    );
  }
  log(`[PASS] ${name}: the retry fails with the same error, without reading KMSGeneration again`);
}

/** Creating the client fails: with no KMSGeneration and no `keyVerification`, nothing can check the key. */
async function expectNoKmsGenerationRejected(name: string): Promise<void> {
  const dummy = await setupCase(name, { withoutKmsGeneration: true });
  const outcome = await attempt(() => Promise.resolve().then(() => createClient(dummy)));
  if (outcome.ok) {
    throw new Error(`${name}: expected client creation to fail without a KMSGeneration.`);
  }
  assertError(outcome.error, 'FhevmConfigError', /has no KMSGeneration contract/, name);
  log(`[PASS] ${name}: client creation rejected (${errorMessage(outcome.error)})`);
}

void runSmokePage(async () => {
  setSmokeRuntimeConfig();

  await expectKeyAccepted('control: digests of the key');
  await expectKeyRejected('wrong CRS digest', { digests: { crsDigest: WRONG_DIGEST } }, /CRS digest mismatch/);
  await expectKeyRejected(
    'wrong public key digest',
    { digests: { publicKeyDigest: WRONG_DIGEST } },
    /FHE public key digest mismatch/,
  );
  await expectKeyRejected('no active key', { activeKeyId: 0n }, /No active FHE public key/);
  await expectNoKmsGenerationRejected('no KMSGeneration');
});

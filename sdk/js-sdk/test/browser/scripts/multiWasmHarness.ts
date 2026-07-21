// Shared harness for the multi-WASM browser smoke / robustness tests.
//
// Centralizes the runtime config, module init + readiness assertions, dummy
// chains, key loading/caching, encryption, and the DOM logging harness, so each
// test page stays small and focused on the behavior it exercises.

import type { TfheVersion } from '../../../src/wasm/tfhe/loadTfheLib.js';
import type { TkmsVersion } from '../../../src/wasm/tkms/loadKmsLib.js';
import type { FheEncryptionCrsBytes, FheEncryptionKeyBytes } from '../../../src/core/types/fheEncryptionKey.js';
import type { FhevmChain } from '../../../src/core/types/fhevmChain.js';
import type { FhevmRuntime, WithDecrypt, WithEncrypt } from '../../../src/core/types/coreFhevmRuntime.js';
import type { ZkProof } from '../../../src/core/types/zkProof-p.js';
import { setFhevmRuntimeConfig } from '../../../src/ethers/index.js';
import { createLogger } from './common.js';
import { getEthersRuntime } from '../../../src/ethers/internal/runtime.js';
import { encryptModule } from '../../../src/core/modules/encrypt/module/index.js';
import { decryptModule } from '../../../src/core/modules/decrypt/module/index.js';
import { defineFhevmChain } from '../../../src/core/chains/utils.js';
import { createZkProofBuilder } from '../../../src/core/coprocessor/ZkProofBuilder-p.js';
import { globalFheEncryptionKeyCache } from '../../../src/core/key/FheEncryptionKeyCache-p.js';
import { createFhevmClientFrozenContext } from '../../../src/core/frozenContext/FhevmClientFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////
// Versions + resource budget
////////////////////////////////////////////////////////////////////////////////

export const EXPECTED_THREADS = 1;
export const TFHE_VERSIONS: readonly TfheVersion[] = ['1.5.3', '1.6.2'];
export const TKMS_VERSIONS: readonly TkmsVersion[] = ['0.13.10', '0.13.20-0'];

/** A runtime equipped with both the encrypt and decrypt modules. */
export type MultiWasmRuntime = WithEncrypt & WithDecrypt;

////////////////////////////////////////////////////////////////////////////////
// DOM logging harness
////////////////////////////////////////////////////////////////////////////////

const logEl = document.getElementById('log')!;
const t0 = performance.now();

function elapsedMs(): string {
  return (performance.now() - t0).toFixed(0);
}

export function log(msg: string): void {
  logEl.textContent += `[${elapsedMs()}ms] ${msg}\n`;
}

function done(status: 'pass' | 'fail'): void {
  const el = document.createElement('div');
  el.id = 'result';
  el.dataset.status = status;
  el.className = status;
  el.textContent = status.toUpperCase();
  document.body.appendChild(el);
}

/**
 * Runs a smoke page's `main`, reporting success/failure to the DOM via a
 * `#result` element the Playwright spec waits on. Requires cross-origin
 * isolation so the multithreaded TFHE workers can use SharedArrayBuffer.
 */
export async function runSmokePage(main: () => Promise<void>): Promise<void> {
  try {
    if (!globalThis.crossOriginIsolated) {
      throw new Error('Expected crossOriginIsolated=true for multithreaded TFHE workers.');
    }
    await main();
    log(`\nAll checks passed in ${elapsedMs()}ms`);
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`);
    done('fail');
  }
}

////////////////////////////////////////////////////////////////////////////////
// Result helpers
////////////////////////////////////////////////////////////////////////////////

export type AttemptOutcome = { readonly ok: boolean; readonly error?: unknown };

/** Runs `fn`, capturing success/failure as a value instead of throwing. */
export async function attempt(fn: () => Promise<unknown>): Promise<AttemptOutcome> {
  try {
    await fn();
    return { ok: true };
  } catch (err) {
    return { ok: false, error: err };
  }
}

export function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

////////////////////////////////////////////////////////////////////////////////
// Bytes
////////////////////////////////////////////////////////////////////////////////

export function base64ToBytes(base64: string): Uint8Array {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  return bytes;
}

////////////////////////////////////////////////////////////////////////////////
// Runtime setup
////////////////////////////////////////////////////////////////////////////////

const WASM_URLS: Record<string, URL> = {
  'tfhe_bg.v1.5.3.wasm': new URL('/__raw_wasm/src/wasm/tfhe/v1.5.3/tfhe_bg.wasm', location.origin),
  'tfhe-worker.v1.5.3.mjs': new URL('/__raw_wasm/src/wasm/tfhe/v1.5.3/tfhe-worker.mjs', location.origin),
  'tfhe_bg.v1.6.2.wasm': new URL('/__raw_wasm/src/wasm/tfhe/v1.6.2/tfhe_bg.wasm', location.origin),
  'tfhe-worker.v1.6.2.mjs': new URL('/__raw_wasm/src/wasm/tfhe/v1.6.2/tfhe-worker.mjs', location.origin),
  'kms_lib_bg.v0.13.10.wasm': new URL('/__raw_wasm/src/wasm/tkms/v0.13.10/kms_lib_bg.wasm', location.origin),
  'kms_lib_bg.v0.13.20-0.wasm': new URL('/__raw_wasm/src/wasm/tkms/v0.13.20-0/kms_lib_bg.wasm', location.origin),
};

function locateWasmFile(file: string): URL {
  const url = WASM_URLS[file];
  if (!url) {
    throw new Error(`Unknown WASM file: ${file}`);
  }
  return url;
}

/**
 * Configures the runtime for direct, URL-backed, multithreaded module init, then
 * returns a runtime extended with the encrypt + decrypt modules.
 */
export function setupMultiWasmRuntime(): MultiWasmRuntime {
  log('Setting runtime config for direct module initialization...');
  setFhevmRuntimeConfig({
    wasmAssetLoadMode: 'verified-blob',
    singleThread: false,
    numberOfThreads: EXPECTED_THREADS,
    locateFile: locateWasmFile,
    logger: createLogger(log),
  });
  log('[PASS] Runtime config set');

  return getEthersRuntime().extend(encryptModule).extend(decryptModule);
}

////////////////////////////////////////////////////////////////////////////////
// Module init + readiness assertions
////////////////////////////////////////////////////////////////////////////////

/** Initializes every TFHE + TKMS version concurrently. */
export async function initAllModules(runtime: MultiWasmRuntime): Promise<void> {
  await Promise.all([
    ...TFHE_VERSIONS.map((tfheVersion) => runtime.encrypt.initTfheModule({ tfheVersion })),
    ...TKMS_VERSIONS.map((tkmsVersion) => runtime.decrypt.initTkmsModule({ tkmsVersion })),
  ]);
}

/** Asserts a TFHE module is initialized, multithreaded with the expected thread count, and URL-backed. */
export async function assertTfheModuleReady(runtime: WithEncrypt, tfheVersion: TfheVersion): Promise<void> {
  const info = await runtime.encrypt.getTfheModuleInfo({ tfheVersion });
  if (info === undefined) {
    throw new Error(`Missing TFHE module info for ${tfheVersion}`);
  }
  if (info.threadsAvailable !== true) {
    throw new Error(`Expected TFHE ${tfheVersion} threadsAvailable=true, got ${String(info.threadsAvailable)}`);
  }
  if (info.numberOfThreads !== EXPECTED_THREADS) {
    throw new Error(`Expected TFHE ${tfheVersion} numberOfThreads=${EXPECTED_THREADS}, got ${info.numberOfThreads}`);
  }
  if (info.wasmUrl === undefined || info.workerUrl === undefined) {
    throw new Error(`Expected TFHE ${tfheVersion} to use URL-backed WASM and worker assets.`);
  }
  log(
    `[PASS] TFHE ${tfheVersion}: threads=${info.numberOfThreads}, wasm=${info.wasmUrl.pathname}, worker=${info.workerUrl.pathname}`,
  );
}

/** Generates a TKMS private key and asserts it reports the requested version. */
async function generateCheckedTkmsPrivateKey(runtime: WithDecrypt, tkmsVersion: TkmsVersion) {
  const tkmsPrivateKey = await runtime.decrypt.generateTkmsPrivateKey({ tkmsVersion });
  if (tkmsPrivateKey.tkmsVersion !== tkmsVersion) {
    throw new Error(`Expected TKMS private key version ${tkmsVersion}, got ${tkmsPrivateKey.tkmsVersion}`);
  }
  return tkmsPrivateKey;
}

/** Derives the TKMS public key and asserts it is a non-empty, even-length byte string; returns its byte length. */
async function tkmsPublicKeyByteLength(
  runtime: WithDecrypt,
  tkmsVersion: TkmsVersion,
  tkmsPrivateKey: Awaited<ReturnType<typeof generateCheckedTkmsPrivateKey>>,
): Promise<number> {
  const publicKeyHex = await runtime.decrypt.getTkmsPublicKeyHex({ tkmsVersion, tkmsPrivateKey });
  const hexNoPrefix = publicKeyHex.startsWith('0x') ? publicKeyHex.slice(2) : publicKeyHex;
  if (hexNoPrefix.length === 0 || hexNoPrefix.length % 2 !== 0) {
    throw new Error(`Expected TKMS ${tkmsVersion} public key to be non-empty.`);
  }
  return hexNoPrefix.length / 2;
}

/** Asserts a TKMS module is initialized and URL-backed; returns its wasm path and memory pages. */
async function assertTkmsModuleInfo(
  runtime: WithDecrypt,
  tkmsVersion: TkmsVersion,
): Promise<{ readonly wasmPath: string; readonly memoryPages: number }> {
  const info = await runtime.decrypt.getTkmsModuleInfo({ tkmsVersion });
  if (info === undefined) {
    throw new Error(`Missing TKMS module info for ${tkmsVersion}`);
  }
  if (info.wasmUrl === undefined) {
    throw new Error(`Expected TKMS ${tkmsVersion} to use URL-backed WASM assets.`);
  }
  return { wasmPath: info.wasmUrl.pathname, memoryPages: info.memory.pages };
}

/** Asserts a TKMS module generates a valid key pair and is URL-backed. */
export async function assertTkmsModuleReady(runtime: WithDecrypt, tkmsVersion: TkmsVersion): Promise<void> {
  const tkmsPrivateKey = await generateCheckedTkmsPrivateKey(runtime, tkmsVersion);
  const publicKeyByteLength = await tkmsPublicKeyByteLength(runtime, tkmsVersion, tkmsPrivateKey);
  const { wasmPath, memoryPages } = await assertTkmsModuleInfo(runtime, tkmsVersion);
  log(
    `[PASS] TKMS ${tkmsVersion}: publicKeyBytes=${publicKeyByteLength}, wasm=${wasmPath}, memoryPages=${memoryPages}`,
  );
}

////////////////////////////////////////////////////////////////////////////////
// Dummy chains
////////////////////////////////////////////////////////////////////////////////

export const DUMMY_RELAYER_BASE_URL = 'http://localhost:9000';
const DUMMY_CONTRACT_ADDRESS = '0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c';
const DUMMY_USER_ADDRESS = '0x857Ca72A957920Fa0FB138602995839866Bd4005';

/**
 * A self-contained dummy chain (no live provider/relayer needed). The Zk proof
 * builder only reads `id`, the ACL address, and `relayerUrl`; the key itself is
 * injected into the global cache before building.
 */
export function makeDummyChain(relayerUrl: string): FhevmChain {
  return defineFhevmChain({
    id: 12_345,
    fhevm: {
      contracts: {
        acl: { address: DUMMY_CONTRACT_ADDRESS },
        inputVerifier: { address: DUMMY_USER_ADDRESS },
        kmsVerifier: { address: '0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11' },
        protocolConfig: undefined, // To be filled
      },
      relayerUrl,
      gateway: {
        id: 54_321,
        contracts: {
          decryption: { address: '0xF0bFB159C7381F7CB332586004d8247252C5b816' },
          inputVerification: { address: '0x3b12Fc766Eb598b285998877e8E90F3e43a1F8d2' },
        },
      },
    },
  });
}

////////////////////////////////////////////////////////////////////////////////
// Encryption keys (test/keys/*.json)
////////////////////////////////////////////////////////////////////////////////

/** Shape of the JSON key files stored under test/keys (bytes are base64-encoded). */
export type SerializedKeyFile = {
  readonly publicKeyBytes: { readonly id: string; readonly bytes: string };
  readonly crsBytes: { readonly id: string; readonly capacity: number; readonly bytes: string };
};

/** Fetches and parses test/keys/key.<keyTfheVersion>.json. */
export async function fetchSerializedKeyFile(keyTfheVersion: string): Promise<SerializedKeyFile> {
  const res = await fetch(new URL(`/test/keys/key.${keyTfheVersion}.json`, location.origin));
  if (!res.ok) {
    throw new Error(`Failed to fetch key.${keyTfheVersion}: ${res.status} ${res.statusText}`);
  }
  return (await res.json()) as SerializedKeyFile;
}

/** Decodes a serialized key file into wasm-ready (Uint8Array) key bytes tagged for `chain`. */
export function toFheEncryptionKeyBytes(keyFile: SerializedKeyFile, chain: FhevmChain): FheEncryptionKeyBytes {
  return {
    publicKeyBytes: {
      id: keyFile.publicKeyBytes.id,
      bytes: base64ToBytes(keyFile.publicKeyBytes.bytes),
    },
    crsBytes: {
      id: keyFile.crsBytes.id,
      capacity: keyFile.crsBytes.capacity,
      bytes: base64ToBytes(keyFile.crsBytes.bytes),
    } as FheEncryptionCrsBytes,
    metadata: { relayerUrl: chain.fhevm.relayerUrl, chainId: chain.id },
  };
}

/**
 * Loads test/keys/key.<keyTfheVersion>.json and injects it into `chain`'s cache
 * slot (its relayer URL), so an encryption can resolve the key offline.
 */
export async function loadAndCacheKey(runtime: FhevmRuntime, chain: FhevmChain, keyTfheVersion: string): Promise<void> {
  const keyFile = await fetchSerializedKeyFile(keyTfheVersion);
  const keyBytes = toFheEncryptionKeyBytes(keyFile, chain);
  globalFheEncryptionKeyCache.setBytes(runtime, chain.fhevm.relayerUrl, keyBytes);
}

////////////////////////////////////////////////////////////////////////////////
// Encryption
////////////////////////////////////////////////////////////////////////////////

/**
 * Builds a real ZK proof for a single uint64 on `chain` using the `tfheVersion`
 * module. The key must already be cached for `chain` (see loadAndCacheKey).
 */
export async function buildUint64Proof(
  runtime: WithEncrypt,
  chain: FhevmChain,
  tfheVersion: TfheVersion,
): Promise<ZkProof> {
  const builder = createZkProofBuilder();
  builder.addUint64(42n);
  return builder.build(
    { chain, runtime },
    {
      contractAddress: DUMMY_CONTRACT_ADDRESS,
      userAddress: DUMMY_USER_ADDRESS,
      extraData: '0x00',
      // No real client here — synthesize the frozen version basis carrying the
      // TFHE version under test; build() reads its tfheVersion from this.
      fhevmContext: createFhevmClientFrozenContext({ tfheVersion }),
    },
  );
}

/** Asserts a built proof is non-empty and exposes exactly `expectedHandleCount` handles. */
export function assertWellFormedProof(zkProof: ZkProof, expectedHandleCount: number): void {
  if (zkProof.ciphertextWithZkProof.length === 0) {
    throw new Error('Expected a non-empty ciphertext-with-proof.');
  }
  const inputHandles = zkProof.getInputHandles();
  if (inputHandles.length !== expectedHandleCount) {
    throw new Error(`Expected ${expectedHandleCount} input handle(s), got ${inputHandles.length}.`);
  }
}

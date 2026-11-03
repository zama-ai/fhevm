/**
 * Shared harness for the browser smoke / robustness tests.
 *
 * Centralizes the runtime config, module readiness assertions, dummy chains,
 * test keys, encryption, and the DOM logging harness, so each test page stays
 * small and focused on the behavior it exercises.
 *
 * Pages drive the SDK through its public entry points only (clients, actions,
 * runtime config). A dummy chain mocks the host chain's RPC alone: the SDK
 * checks the test keys for real against its mocked KMSGeneration.
 */

import type { ethers as EthersT } from 'ethers';
import type { TfheVersion } from '../../../src/wasm/tfhe/loadTfheLib.js';
import type { TkmsVersion } from '../../../src/wasm/tkms/loadKmsLib.js';
import type { FheEncryptionCrsBytes, FheEncryptionKeyBytes } from '../../../src/core/types/fheEncryptionKey.js';
import type { FhevmChain } from '../../../src/core/types/fhevmChain.js';
import type { WithDecrypt, WithEncrypt } from '../../../src/core/types/coreFhevmRuntime.js';
import type { FhevmEncryptClient } from '../../../src/core/types/fhevmClient.js';
import type { GenerateZkProofReturnType as ZkProof } from '../../../src/core/actions/encrypt/index.js';
import { Interface } from 'ethers';
import { setFhevmRuntimeConfig } from '../../../src/ethers/index.js';
import { defineFhevmChain } from '../../../src/core/chains/index.js';
import { generateZkProof } from '../../../src/core/actions/encrypt/index.js';
import { createLogger } from './common.js';

////////////////////////////////////////////////////////////////////////////////
// Resource budget
////////////////////////////////////////////////////////////////////////////////

export const EXPECTED_THREADS = 1;

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

/** `<name>.v<version>.<ext>`, as the SDK asks for it, and the folder of its build. */
const WASM_FILE_PATTERN = /^(tfhe_bg|tfhe-worker|kms_lib_bg)\.v(.+)\.(wasm|mjs)$/;
const WASM_FOLDERS: Record<string, string> = { tfhe_bg: 'tfhe', 'tfhe-worker': 'tfhe', kms_lib_bg: 'tkms' };

/** Serves the version the SDK asks for from its raw build folder. */
function locateWasmFile(file: string): URL {
  const match = WASM_FILE_PATTERN.exec(file);
  if (match === null) {
    throw new Error(`Unknown WASM file: ${file}`);
  }
  const [, name, version, ext] = match as unknown as [string, string, string, string];
  return new URL(`/__raw_wasm/src/wasm/${WASM_FOLDERS[name]}/v${version}/${name}.${ext}`, location.origin);
}

/** Configures the runtime for URL-backed, multithreaded module init. */
export function setSmokeRuntimeConfig(): void {
  log('Setting runtime config...');
  setFhevmRuntimeConfig({
    wasmAssetLoadMode: 'verified-blob',
    singleThread: false,
    numberOfThreads: EXPECTED_THREADS,
    locateFile: locateWasmFile,
    logger: createLogger(log),
  });
  log('[PASS] Runtime config set');
}

////////////////////////////////////////////////////////////////////////////////
// Module readiness assertions
////////////////////////////////////////////////////////////////////////////////

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
  const tkmsPrivateKey = await runtime.decrypt.generateTkmsPrivateKey();
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
  const publicKeyHex = await runtime.decrypt.getTkmsPublicKeyHex({ tkmsPrivateKey });
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
 * A self-contained dummy chain: no relayer is ever reached (the key is
 * provided), and its RPC is mocked (see createDummyChainRunner).
 */
function makeDummyChain(relayerUrl: string, withKmsGeneration: boolean): FhevmChain {
  return defineFhevmChain({
    id: 12_345,
    fhevm: {
      contracts: {
        acl: { address: DUMMY_CONTRACT_ADDRESS },
        inputVerifier: { address: DUMMY_USER_ADDRESS },
        kmsVerifier: { address: '0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11' },
        kmsGeneration: withKmsGeneration ? { address: '0x3E0fBCcE61af7C01113027449eEFFF5DCd501419' } : undefined,
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

/** Host contract versions of the v1.0.0 stack, as each `getVersion()` returns them. */
const HOST_CONTRACT_VERSIONS = {
  acl: 'ACL v0.6.0',
  inputVerifier: 'InputVerifier v0.3.0',
  kmsVerifier: 'KMSVerifier v0.5.0',
  kmsGeneration: 'KMSGeneration v0.1.0',
} as const;

/** The active key and CRS ids of the mocked KMSGeneration: any non-zero id (the test keys carry label ids). */
const ACTIVE_KEY_ID = 1n;
const ACTIVE_CRS_ID = 1n;

/** What a dummy chain's mocked KMSGeneration serves as the active key. */
type MockedKmsGeneration = {
  readonly activeKeyId: bigint;
  readonly digests: FheEncryptionKeyDigests;
  /** Every KMSGeneration function called, in order. */
  readonly calls: string[];
};
/** `KeyType.Public` in KMSGeneration's key digests. */
const KEY_TYPE_PUBLIC = 1;

/** The host contract views the SDK reads, as deployed. */
const HOST_CONTRACT_ABI = new Interface([
  'function getVersion() view returns (string)',
  'function getActiveKeyId() view returns (uint256)',
  'function getActiveCrsId() view returns (uint256)',
  'function getKeyMaterials(uint256 keyId) view returns (string[], (uint8 keyType, bytes digest)[])',
  'function getCrsMaterials(uint256 crsId) view returns (string[], bytes)',
]);

/**
 * The mocked RPC of a dummy chain, as an ethers runner: answers `getVersion()`
 * for its ACL, InputVerifier and KMSVerifier, and, for its KMSGeneration (if
 * any), the reads that check a key: version, active ids, and the digests as
 * the active key and CRS materials. Any other call fails the test.
 */
function createDummyChainRunner(
  chain: FhevmChain,
  kmsGenerationMock: MockedKmsGeneration | undefined,
): EthersT.ContractRunner {
  const { acl, inputVerifier, kmsVerifier, kmsGeneration } = chain.fhevm.contracts;
  const versionsByAddress = new Map<string, string>([
    [acl.address.toLowerCase(), HOST_CONTRACT_VERSIONS.acl],
    [inputVerifier.address.toLowerCase(), HOST_CONTRACT_VERSIONS.inputVerifier],
    [kmsVerifier.address.toLowerCase(), HOST_CONTRACT_VERSIONS.kmsVerifier],
  ]);
  const kmsGenerationAddress = kmsGeneration?.address.toLowerCase();

  const answer = (to: string, data: string): string => {
    const fn = HOST_CONTRACT_ABI.getFunction(data.slice(0, 10));
    if (fn === null) {
      throw new Error(`Unexpected call to ${to}: selector ${data.slice(0, 10)}`);
    }
    const version = versionsByAddress.get(to.toLowerCase());
    if (fn.name === 'getVersion' && version !== undefined) {
      return HOST_CONTRACT_ABI.encodeFunctionResult(fn, [version]);
    }
    if (kmsGenerationMock === undefined || to.toLowerCase() !== kmsGenerationAddress) {
      throw new Error(`Unexpected call to ${to}: ${fn.name}`);
    }
    kmsGenerationMock.calls.push(fn.name);
    const { activeKeyId, digests } = kmsGenerationMock;
    const args = HOST_CONTRACT_ABI.decodeFunctionData(fn, data);
    switch (fn.name) {
      case 'getVersion':
        return HOST_CONTRACT_ABI.encodeFunctionResult(fn, [HOST_CONTRACT_VERSIONS.kmsGeneration]);
      case 'getActiveKeyId':
        return HOST_CONTRACT_ABI.encodeFunctionResult(fn, [activeKeyId]);
      case 'getActiveCrsId':
        return HOST_CONTRACT_ABI.encodeFunctionResult(fn, [ACTIVE_CRS_ID]);
      case 'getKeyMaterials':
        if (args[0] !== activeKeyId) {
          throw new Error(`Unexpected getKeyMaterials(${String(args[0])})`);
        }
        return HOST_CONTRACT_ABI.encodeFunctionResult(fn, [[], [[KEY_TYPE_PUBLIC, digests.publicKeyDigest]]]);
      case 'getCrsMaterials':
        if (args[0] !== ACTIVE_CRS_ID) {
          throw new Error(`Unexpected getCrsMaterials(${String(args[0])})`);
        }
        return HOST_CONTRACT_ABI.encodeFunctionResult(fn, [[], digests.crsDigest]);
      default:
        throw new Error(`Unexpected call to ${to}: ${fn.name}`);
    }
  };

  return {
    provider: null,
    call: (tx: { readonly to?: unknown; readonly data?: unknown }): Promise<string> => {
      try {
        return Promise.resolve(answer(String(tx.to), String(tx.data)));
      } catch (e) {
        return Promise.reject(e);
      }
    },
  } as unknown as EthersT.ContractRunner;
}

/** A dummy chain, its mocked RPC, and the test key it serves as active. */
export type DummyChain = {
  readonly chain: FhevmChain;
  readonly provider: EthersT.ContractRunner;
  /** For `options.fheEncryptionKey`: no relayer is reached. */
  readonly fheEncryptionKey: FheEncryptionKeyBytes;
  /** Every KMSGeneration function the SDK called, in order (empty without a KMSGeneration). */
  readonly kmsGenerationCalls: readonly string[];
};

/** Deviations of a dummy chain from a KMSGeneration serving the key as active (negative key checks). */
export type DummyChainOptions = {
  /** Serves these digests in place of the key's own. */
  readonly digests?: { readonly publicKeyDigest?: string; readonly crsDigest?: string } | undefined;
  /** Serves this active key id (0: no key generation completed). */
  readonly activeKeyId?: bigint | undefined;
  /** Deploys no KMSGeneration on the chain. */
  readonly withoutKmsGeneration?: boolean | undefined;
};

/**
 * A dummy chain whose active key is test/keys/key.<keyTfheVersion>.json: its
 * mocked KMSGeneration serves that key's digests (key.<keyTfheVersion>.digests.json),
 * so the SDK checks the key for real before using it. `options` make it serve
 * something else, for the check to fail.
 */
export async function setupDummyChain(
  relayerUrl: string,
  keyTfheVersion: string,
  options: DummyChainOptions = {},
): Promise<DummyChain> {
  const chain = makeDummyChain(relayerUrl, options.withoutKmsGeneration !== true);
  const [keyFile, digests] = await Promise.all([
    fetchTestKeyFile<SerializedKeyFile>(`key.${keyTfheVersion}.json`),
    fetchTestKeyFile<FheEncryptionKeyDigests>(`key.${keyTfheVersion}.digests.json`),
  ]);
  const kmsGenerationCalls: string[] = [];
  const kmsGenerationMock: MockedKmsGeneration | undefined =
    options.withoutKmsGeneration === true
      ? undefined
      : {
          activeKeyId: options.activeKeyId ?? ACTIVE_KEY_ID,
          digests: { ...digests, ...options.digests },
          calls: kmsGenerationCalls,
        };
  return {
    chain,
    provider: createDummyChainRunner(chain, kmsGenerationMock),
    fheEncryptionKey: toFheEncryptionKeyBytes(keyFile, chain),
    kmsGenerationCalls,
  };
}

////////////////////////////////////////////////////////////////////////////////
// Encryption keys (test/keys/*.json)
////////////////////////////////////////////////////////////////////////////////

/** Shape of the JSON key files stored under test/keys (bytes are base64-encoded). */
type SerializedKeyFile = {
  readonly publicKeyBytes: { readonly id: string; readonly bytes: string };
  readonly crsBytes: { readonly id: string; readonly capacity: number; readonly bytes: string };
};

/** Shape of test/keys/key.<version>.digests.json: the key's KMSGeneration digests. */
type FheEncryptionKeyDigests = {
  readonly publicKeyDigest: string;
  readonly crsDigest: string;
};

/** Fetches and parses test/keys/<name>. */
async function fetchTestKeyFile<T>(name: string): Promise<T> {
  const res = await fetch(new URL(`/test/keys/${name}`, location.origin));
  if (!res.ok) {
    throw new Error(`Failed to fetch ${name}: ${res.status} ${res.statusText}`);
  }
  return (await res.json()) as T;
}

/** Decodes a serialized key file into wasm-ready (Uint8Array) key bytes tagged for `chain`. */
function toFheEncryptionKeyBytes(keyFile: SerializedKeyFile, chain: FhevmChain): FheEncryptionKeyBytes {
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

////////////////////////////////////////////////////////////////////////////////
// Encryption
////////////////////////////////////////////////////////////////////////////////

/** Builds a real ZK proof for a single uint64 with `client`'s (checked) key. */
export function generateUint64Proof(client: FhevmEncryptClient): Promise<ZkProof> {
  return generateZkProof(client, {
    contractAddress: DUMMY_CONTRACT_ADDRESS,
    userAddress: DUMMY_USER_ADDRESS,
    values: [{ type: 'uint64', value: 42n }],
  });
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

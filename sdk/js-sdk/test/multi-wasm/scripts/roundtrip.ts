import type { FhevmChain } from '../../../src/core/chains/index.js';
import type { FhevmModuleVersions } from '../../../src/core/types/moduleVersions.js';
import type { WasmAssetLoadMode } from '../../../src/core/types/wasmAssets.js';
import type { TypedValue } from '../../../src/core/types/index.js';
import type { TkmsVersion } from '../../../src/wasm/tkms/loadKmsLib.js';
import type { TfheVersion } from '../../../src/wasm/tfhe/loadTfheLib.js';
import { defineFhevmChain } from '../../../src/core/chains/index.js';
import { createFhevmClient, setFhevmRuntimeConfig } from '../../../src/ethers/index.js';
import { ethers } from 'ethers';

type Matrix = {
  readonly defaults: { readonly roundTrip: RoundTrip };
  readonly supportedVersionPairs: readonly VersionPair[];
  readonly assetUrlSets: Record<string, AssetUrlSet>;
};

type VersionPair = {
  readonly tfhe: TfheVersion;
  readonly kms: TkmsVersion;
};

type AssetUrlSet = {
  readonly tfheWasm: string;
  readonly tfheWorker: string;
  readonly kmsWasm: string;
};

type RoundTrip = {
  readonly clearType: TypedValue['type'];
  readonly contractMethod:
    | 'setEbool'
    | 'setEuint8'
    | 'setEuint16'
    | 'setEuint32'
    | 'setEuint64'
    | 'setEuint128'
    | 'setEuint256'
    | 'setEaddress';
  readonly makePublic: boolean;
  readonly value: boolean | number | string;
};

const LOCAL_CDN = 'local';
const WASM_ASSET_LOAD_MODES: readonly WasmAssetLoadMode[] = [
  'auto',
  'embedded-base64',
  'verified-blob',
  'precheck-direct-url',
  'trusted-direct-url',
];

// solve minio problem
const LOCALSTACK_RELAYER_URL = new URL('/__localstack_relayer', location.origin).toString();

// Eager-import all localstack* chain configs from the test/chains
// folder. `import.meta.glob` requires a static string literal, but the
// pattern matches every current and future variant (localstack.ts,
// localstack_v11.ts, ...), so adding a new chain just means dropping its
// .ts file in that directory — no edit here.
const localstackChainLoaders = import.meta.glob<Record<string, FhevmChain>>('../../chains/localstack*.ts');

async function resolveChain(parameters: {
  readonly chainName: string;
  readonly relayerUrl: string;
}): Promise<FhevmChain> {
  const { chainName, relayerUrl } = parameters;

  const modulePath = `../../chains/${chainName}.ts`;
  const loader = localstackChainLoaders[modulePath];
  if (!loader) {
    const available = Object.keys(localstackChainLoaders)
      .map((p) => p.replace(/^.*\/(.+)\.ts$/, '$1'))
      .join(', ');
    throw new Error(`Unsupported chainName "${chainName}". Available: ${available}.`);
  }

  const mod = await loader();
  const sourceChain = mod[chainName];
  if (!sourceChain) {
    throw new Error(`Chain "${chainName}" not exported by ${modulePath}.`);
  }

  // Override relayerUrl with the COEP-safe Vite proxy URL.
  return defineFhevmChain({
    id: sourceChain.id,
    fhevm: { ...sourceChain.fhevm, relayerUrl },
  }) satisfies FhevmChain;
}

const FHETEST_ABI = [
  {
    type: 'function',
    name: 'setEuint8',
    inputs: [
      { name: 'inputEuint8', type: 'bytes32', internalType: 'externalEuint8' },
      { name: 'inputProof', type: 'bytes', internalType: 'bytes' },
      { name: 'clearValue', type: 'uint8', internalType: 'uint8' },
      { name: 'makePublic', type: 'bool', internalType: 'bool' },
    ],
    outputs: [{ name: '', type: 'bytes32', internalType: 'euint8' }],
    stateMutability: 'nonpayable',
  },
] as const;

const logEl = document.getElementById('log')!;
const t0 = performance.now();

function log(msg: string) {
  const elapsed = (performance.now() - t0).toFixed(0);
  const line = `[${elapsed}ms] ${msg}`;
  logEl.textContent += `${line}\n`;
  console.log(`[multi-wasm] ${line}`);
}

function done(status: 'pass' | 'fail') {
  const el = document.createElement('div');
  el.id = 'result';
  el.dataset.status = status;
  el.className = status;
  el.textContent = status.toUpperCase();
  document.body.appendChild(el);
}

async function run() {
  try {
    const query = new URLSearchParams(location.search);
    const tfhe = requiredQuery(query, 'tfhe') as TfheVersion;
    const kms = requiredQuery(query, 'kms') as TkmsVersion;
    const mode = requiredQuery(query, 'mode');
    const cdn = requiredQuery(query, 'cdn');
    const chainName = requiredQuery(query, 'chainName');
    const rpcUrl = requiredQuery(query, 'rpcUrl');
    const mnemonic = requiredQuery(query, 'mnemonic');
    const fheTestAddress = requiredQuery(query, 'fheTestAddress');

    if (!WASM_ASSET_LOAD_MODES.includes(mode as WasmAssetLoadMode)) {
      throw new Error(`Unknown wasmAssetLoadMode: ${mode}`);
    }
    if (mode === 'embedded-base64' && cdn !== LOCAL_CDN) {
      throw new Error(`embedded-base64 is incompatible with cdn "${cdn}"`);
    }

    const matrix = await loadMatrix();
    const versionPair = matrix.supportedVersionPairs.find(
      (candidate) => candidate.tfhe === tfhe && candidate.kms === kms,
    );
    if (versionPair === undefined) {
      throw new Error(`Unsupported version pair: tfhe=${tfhe} kms=${kms}`);
    }

    const wasmAssetLoadMode = mode as WasmAssetLoadMode;
    const roundTrip = matrix.defaults.roundTrip;
    const value = normalizeRoundTripValue(roundTrip);
    const moduleVersions: FhevmModuleVersions = { tfhe, kms };
    const typedValue = { type: roundTrip.clearType, value } as TypedValue;
    const assetUrls = wasmAssetLoadMode === 'embedded-base64' ? undefined : resolveAssetUrls(matrix, versionPair, cdn);

    log(`Matrix entry: TFHE ${tfhe} + TKMS ${kms} / ${wasmAssetLoadMode} / ${cdn}`);
    log(`TFHE ${tfhe}, TKMS ${kms}`);
    log(`wasmAssetLoadMode: ${wasmAssetLoadMode}`);
    log(`cdn: ${cdn}`);
    log(`assetUrls: ${assetUrls === undefined ? 'embedded/default' : JSON.stringify(assetUrls)}`);

    setFhevmRuntimeConfig({
      wasmAssetLoadMode,
      locateFile:
        assetUrls === undefined
          ? undefined
          : (file: string): URL => {
              return resolveWasmAssetUrl(versionPair, assetUrls, file);
            },
      logger: {
        debug: (message: string) => log(`[debug] ${message}`),
        warn: (message: string) => log(`[warn] ${message}`),
        error: (message: string, cause: unknown) => {
          log(`[error] ${message}`);
          if (cause !== undefined) log(`[error] ${cause}`);
        },
      },
    });

    const fhevmChain = await resolveChain({
      chainName,
      relayerUrl: LOCALSTACK_RELAYER_URL,
    });

    const provider = new ethers.JsonRpcProvider(rpcUrl);
    const wallet = ethers.HDNodeWallet.fromMnemonic(ethers.Mnemonic.fromPhrase(mnemonic)).connect(provider);
    const signer = new ethers.NonceManager(wallet);
    const fheTest = new ethers.Contract(fheTestAddress, FHETEST_ABI, signer);

    log('Creating FHEVM client...');
    const client = createFhevmClient({
      chain: fhevmChain,
      provider,
      options: { moduleVersions },
    });

    log('Initializing FHEVM client...');
    await client.ready;

    log(`Encrypting ${roundTrip.clearType} value...`);
    const encrypted = await client.encryptValues({
      contractAddress: fheTestAddress,
      userAddress: wallet.address,
      values: [typedValue],
    });

    const encryptedValue = encrypted.encryptedValues[0];
    if (encryptedValue === undefined) {
      throw new Error('encryptValues returned no encrypted value');
    }

    log(`Submitting encrypted value to FHETest.${roundTrip.contractMethod}...`);
    const setValue = fheTest[roundTrip.contractMethod] as (
      inputHandle: string,
      inputProof: string,
      clearValue: boolean | bigint | number | string,
      makePublic: boolean,
    ) => Promise<ethers.TransactionResponse>;
    const tx = await setValue(encryptedValue, encrypted.inputProof, value, roundTrip.makePublic);
    const receipt = await tx.wait();
    if (receipt?.status !== 1) {
      throw new Error(`FHETest transaction failed: ${tx.hash}`);
    }

    log('Private decrypting value...');
    const transportKeyPair = await client.generateTransportKeyPair();
    const signedPermit = await client.signDecryptionPermit({
      transportKeyPair,
      contractAddresses: [fheTestAddress],
      durationSeconds: 24 * 3600,
      startTimestamp: Math.floor(Date.now() / 1000) - 5,
      signerAddress: wallet.address,
      signer,
    });
    const privateValues = await client.decryptValues({
      encryptedValues: [encryptedValue],
      contractAddress: fheTestAddress,
      signedPermit,
      transportKeyPair,
    });

    assertEqualValue('private decrypt', privateValues[0]?.value, value);
    log(`  private decrypt -> ${String(privateValues[0]?.value)}`);

    if (roundTrip.makePublic) {
      log('Public decrypting value...');
      const publicValues = await client.decryptPublicValues({ encryptedValues: [encryptedValue] });
      assertEqualValue('public decrypt', publicValues[0]?.value, value);
      log(`  public decrypt -> ${String(publicValues[0]?.value)}`);
    }

    log(`All checks passed in ${(performance.now() - t0).toFixed(0)}ms`);
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`);
    done('fail');
  }
}

async function loadMatrix(): Promise<Matrix> {
  const response = await fetch('/test/multi-wasm/matrix.json');
  if (!response.ok) {
    throw new Error(`Failed to load matrix.json: ${response.status}`);
  }
  return (await response.json()) as Matrix;
}

function requiredQuery(query: URLSearchParams, name: string): string {
  const value = query.get(name);
  if (value === null || value === '') {
    throw new Error(`Missing query parameter: ${name}`);
  }
  return value;
}

function resolveAssetUrls(matrix: Matrix, versionPair: VersionPair, cdn: string): AssetUrlSet {
  const template = matrix.assetUrlSets[cdn];
  if (template === undefined) {
    throw new Error(`Unknown assetUrlSet "${cdn}"`);
  }
  return {
    tfheWasm: renderAssetUrlTemplate(template.tfheWasm, versionPair),
    tfheWorker: renderAssetUrlTemplate(template.tfheWorker, versionPair),
    kmsWasm: renderAssetUrlTemplate(template.kmsWasm, versionPair),
  };
}

function renderAssetUrlTemplate(template: string, versions: VersionPair): string {
  return template.replace(/\{tfhe\}/g, versions.tfhe).replace(/\{kms\}/g, versions.kms);
}

function resolveWasmAssetUrl(versionPair: VersionPair, assetUrls: AssetUrlSet, file: string): URL {
  const urlsByFilename: Record<string, string> = {
    [`tfhe_bg.v${versionPair.tfhe}.wasm`]: assetUrls.tfheWasm,
    [`tfhe-worker.v${versionPair.tfhe}.mjs`]: assetUrls.tfheWorker,
    [`kms_lib_bg.v${versionPair.kms}.wasm`]: assetUrls.kmsWasm,
  };
  const path = urlsByFilename[file];

  if (path === undefined) {
    throw new Error(`Unexpected WASM asset request: ${file}`);
  }

  return new URL(path, location.origin);
}

function normalizeRoundTripValue(roundTrip: RoundTrip): boolean | bigint | number | string {
  switch (roundTrip.clearType) {
    case 'uint64':
    case 'uint128':
    case 'uint256':
      return BigInt(roundTrip.value.toString());
    default:
      return roundTrip.value;
  }
}

function assertEqualValue(label: string, actual: unknown, expected: unknown): void {
  if (String(actual) !== String(expected)) {
    throw new Error(`${label} mismatch: expected ${String(expected)}, got ${String(actual)}`);
  }
}

run();

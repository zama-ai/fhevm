import type { FhevmChain } from '../../../src/core/chains/index.js';
import type { FhevmModuleVersions } from '../../../src/core/types/moduleVersions.js';
import type { WasmAssetLoadMode } from '../../../src/core/types/wasmAssets.js';
import type { TkmsVersion } from '../../../src/wasm/tkms/loadKmsLib.js';
import type { TfheVersion } from '../../../src/wasm/tfhe/loadTfheLib.js';
import { defineFhevmChain, sepolia } from '../../../src/core/chains/index.js';
import { createFhevmClient, setFhevmRuntimeConfig } from '../../../src/ethers/index.js';
import { createFhevmCleartextClient } from '../../../src/ethers/cleartext/index.js';
import { ethers } from 'ethers';
import './styles.css';

type ChainTarget = 'testnet' | 'localstack' | 'localcleartext';
type AssetSource = 'local' | 'jsdelivr' | 'unpkg';

type BrowserUiConfig = {
  readonly targets: Record<
    ChainTarget,
    {
      readonly rpcUrl: string;
      readonly mnemonic: string;
      readonly fheTestAddress: string;
    }
  >;
};

type Matrix = {
  readonly supportedVersionPairs: readonly VersionPair[];
  readonly assetUrlSets: Record<AssetSource, AssetUrlSet>;
};

type VersionPair = {
  readonly tfhe: TfheVersion;
  readonly kms: TkmsVersion;
  readonly cdns?: readonly AssetSource[];
};

type AssetUrlSet = {
  readonly tfheWasm: string;
  readonly tfheWorker: string;
  readonly kmsWasm: string;
};

type DemoState = {
  readonly client: ReturnType<typeof createFhevmClient> | ReturnType<typeof createFhevmCleartextClient>;
  readonly fheTest: FheTestContract;
  readonly fheTestAddress: string;
  readonly provider: ethers.JsonRpcProvider;
  readonly signer: ethers.NonceManager;
  readonly walletAddress: string;
  readonly value: bigint;
  readonly encryptedValue: string;
  readonly storedHandle: string;
};

type FheTestContract = ethers.Contract & {
  readonly setEuint64: (
    inputEuint64: string,
    inputProof: string,
    clearValue: bigint,
    makePublic: boolean,
  ) => Promise<ethers.ContractTransactionResponse>;
  readonly getHandleOf: (account: string, fheType: number) => Promise<string>;
};

const LOCALSTACK_RELAYER_URL = new URL('/__localstack_relayer', location.origin).toString();
const FORM_SETTINGS_STORAGE_KEY = 'fhevm-browser-ui-form-settings-v1';
const WASM_RUNTIME_CONTROL_IDS = new Set([
  'wasmAssetLoadMode',
  'tfheVersion',
  'kmsVersion',
  'assetSource',
  'threadingMode',
  'initMode',
]);
const UINT64_MAX = (1n << 64n) - 1n;

const FHETEST_ABI = [
  {
    type: 'function',
    name: 'CONTRACT_NAME',
    inputs: [],
    outputs: [{ name: '', type: 'string', internalType: 'string' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'setEuint64',
    inputs: [
      { name: 'inputEuint64', type: 'bytes32', internalType: 'externalEuint64' },
      { name: 'inputProof', type: 'bytes', internalType: 'bytes' },
      { name: 'clearValue', type: 'uint64', internalType: 'uint64' },
      { name: 'makePublic', type: 'bool', internalType: 'bool' },
    ],
    outputs: [{ name: '', type: 'bytes32', internalType: 'euint64' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    name: 'getHandleOf',
    inputs: [
      { name: 'account', type: 'address', internalType: 'address' },
      { name: 'fheType', type: 'uint8', internalType: 'enum FheType' },
    ],
    outputs: [{ name: '', type: 'bytes32', internalType: 'bytes32' }],
    stateMutability: 'view',
  },
] as const;

const elements = {
  form: byId<HTMLFormElement>('demoForm'),
  environmentLine: byId<HTMLParagraphElement>('environmentLine'),
  valueInputLabel: byId<HTMLSpanElement>('valueInputLabel'),
  valueInput: byId<HTMLInputElement>('valueInput'),
  chainTarget: byId<HTMLSelectElement>('chainTarget'),
  wasmAssetLoadMode: byId<HTMLSelectElement>('wasmAssetLoadMode'),
  tfheVersion: byId<HTMLSelectElement>('tfheVersion'),
  kmsVersion: byId<HTMLSelectElement>('kmsVersion'),
  assetSource: byId<HTMLSelectElement>('assetSource'),
  threadingMode: byId<HTMLSelectElement>('threadingMode'),
  initMode: byId<HTMLSelectElement>('initMode'),
  decryptMode: byId<HTMLSelectElement>('decryptMode'),
  encryptButton: byId<HTMLButtonElement>('encryptButton'),
  restartButton: byId<HTMLButtonElement>('restartButton'),
  inputHandle: byId<HTMLElement>('inputHandle'),
  storedHandle: byId<HTMLElement>('storedHandle'),
  transactionHash: byId<HTMLElement>('transactionHash'),
  logOutput: byId<HTMLPreElement>('logOutput'),
};

let matrix: Matrix | undefined;
let browserConfig: BrowserUiConfig | undefined;
let runtimeConfigKey: string | undefined;
let demoState: DemoState | undefined;
const t0 = performance.now();
const defaultInputValue = elements.valueInput.value;

void boot();

async function boot(): Promise<void> {
  try {
    setBusy(true, 'Loading test configuration...');
    const [loadedMatrix, loadedConfig] = await Promise.all([loadMatrix(), loadBrowserUiConfig()]);
    matrix = loadedMatrix;
    browserConfig = loadedConfig;

    renderSelectOptions(elements.tfheVersion, unique(loadedMatrix.supportedVersionPairs.map((p) => p.tfhe)));
    renderSelectOptions(elements.kmsVersion, unique(loadedMatrix.supportedVersionPairs.map((p) => p.kms)));
    renderSelectOptions(elements.assetSource, Object.keys(loadedMatrix.assetUrlSets));
    restorePersistedFormSettings();
    enhanceSelects(elements.form.querySelectorAll('select'));

    elements.environmentLine.textContent = `crossOriginIsolated=${String(crossOriginIsolated)} / ${navigator.userAgent}`;
    elements.encryptButton.addEventListener('click', () => {
      if (demoState === undefined) {
        void encrypt();
      } else {
        void decrypt();
      }
    });
    elements.restartButton.addEventListener('click', () => restartWorkflow({ clearLog: true, resetInput: true }));
    elements.form.addEventListener('change', (event) => {
      if (isWasmRuntimeControl(event.target)) {
        persistFormSettings();
        location.reload();
        return;
      }
      if (event.target === elements.decryptMode && demoState !== undefined) {
        return;
      }
      const wasEncrypted = demoState !== undefined;
      restartWorkflow({ resetInput: wasEncrypted });
    });

    setBusy(false, '');
    log('Browser UI ready.');
  } catch (err) {
    setBusy(false, '');
    logError(err);
  }
}

async function encrypt(): Promise<void> {
  try {
    const options = readFormOptions();
    validateDecryptionSelection(options);
    resetResults();
    setEncryptMode();
    setBusy(true, 'Preparing FHEVM client...');
    elements.encryptButton.disabled = true;
    elements.restartButton.disabled = true;
    setPrimaryButtonLoading('Encrypting');

    const clientContext = await createClientContext(options);

    if (options.manualInit) {
      setBusy(true, 'Initializing WASM runtime...');
      await clientContext.client.ready;
      log('Runtime initialized manually.');
    }

    setBusy(true, `Encrypting uint64 ${options.value.toString()}...`);
    const encrypted = await clientContext.client.encryptValues({
      contractAddress: clientContext.fheTestAddress,
      userAddress: clientContext.walletAddress,
      values: [{ type: 'uint64', value: options.value }],
    });

    const encryptedValue = encrypted.encryptedValues[0];
    if (encryptedValue === undefined) {
      throw new Error('encryptValues returned no encrypted value.');
    }
    setText(elements.inputHandle, encryptedValue);
    log(`Input handle: ${encryptedValue}`);

    setBusy(true, 'Submitting encrypted value to FHETest...');
    const tx = await clientContext.fheTest.setEuint64(
      encryptedValue,
      encrypted.inputProof,
      options.value,
      options.publicDecrypt,
    );
    setText(elements.transactionHash, tx.hash);
    log(`Transaction submitted: ${tx.hash}`);

    setBusy(true, 'Waiting for transaction receipt...');
    const receipt = await tx.wait();
    if (receipt?.status !== 1) {
      throw new Error(`FHETest transaction failed: ${tx.hash}`);
    }

    const storedHandle = (await clientContext.fheTest.getHandleOf(clientContext.walletAddress, 5)) as string;
    setText(elements.storedHandle, storedHandle);
    elements.valueInput.value = storedHandle;
    log(`Stored handle: ${storedHandle}`);

    demoState = {
      ...clientContext,
      value: options.value,
      encryptedValue,
      storedHandle,
    };
    setDecryptMode();
    setBusy(false, 'Encrypted value stored. Ready to decrypt.');
  } catch (err) {
    setBusy(false, '');
    logError(err);
    setEncryptMode();
  } finally {
    elements.encryptButton.disabled = false;
  }
}

async function decrypt(): Promise<void> {
  try {
    const state = demoState;
    if (state === undefined) {
      throw new Error('Encrypt a value first.');
    }

    const options = readDecryptOptions();
    validateDecryptionSelection(options);
    elements.encryptButton.disabled = true;
    setPrimaryButtonLoading('Decrypting');

    const valuesToDecrypt = [state.storedHandle];
    let clearValue: unknown;

    if (options.publicDecrypt) {
      setBusy(true, 'Running public decrypt...');
      const publicValues = await state.client.decryptPublicValues({ encryptedValues: valuesToDecrypt });
      const value = publicValues[0]?.value;
      assertRoundTrip('public decrypt', value, state.value);
      clearValue = value;
      log(`Public decrypt: ${String(value)}`);
    } else {
      log('Public decrypt: skipped');
    }

    if (options.userDecrypt) {
      setBusy(true, 'Signing permit and running user decrypt...');
      const transportKeyPair = await state.client.generateTransportKeyPair();
      const signedPermit = await state.client.signLegacyDecryptionPermit({
        transportKeyPair,
        contractAddresses: [state.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: state.walletAddress,
        signer: state.signer,
      });
      const privateValues = await state.client.decryptValues({
        encryptedValues: valuesToDecrypt,
        contractAddress: state.fheTestAddress,
        signedPermit,
        transportKeyPair,
      });
      const value = privateValues[0]?.value;
      assertRoundTrip('user decrypt', value, state.value);
      clearValue = value;
      log(`User decrypt: ${String(value)}`);
    } else {
      log('User decrypt: skipped');
    }

    elements.valueInput.value = String(clearValue ?? state.value);
    demoState = undefined;
    setEncryptMode({ restartEnabled: true });
    setBusy(false, '');
  } catch (err) {
    setBusy(false, '');
    logError(err);
    setDecryptMode();
  } finally {
    elements.encryptButton.disabled = false;
  }
}

async function createClientContext(
  options: FormOptions,
): Promise<Omit<DemoState, 'value' | 'encryptedValue' | 'storedHandle'>> {
  const loadedMatrix = requireLoaded(matrix, 'matrix');
  const loadedConfig = requireLoaded(browserConfig, 'browser config');
  const targetConfig = loadedConfig.targets[options.chainTarget];
  if (targetConfig === undefined) {
    throw new Error(`Missing browser UI config for ${options.chainTarget}.`);
  }

  const versionPair = resolveVersionPair(loadedMatrix, options.tfhe, options.kms, options.assetSource);
  applyRuntimeConfig(loadedMatrix, versionPair, options);
  const moduleVersions: FhevmModuleVersions = {
    tfhe: versionPair.tfhe,
    kms: versionPair.kms,
  };

  const provider = new ethers.JsonRpcProvider(targetConfig.rpcUrl);
  await assertContractDeployed(provider, targetConfig.fheTestAddress, 'FHETest', options.chainTarget);
  const wallet = ethers.HDNodeWallet.fromMnemonic(ethers.Mnemonic.fromPhrase(targetConfig.mnemonic)).connect(provider);
  await ensureSignerFunds(options.chainTarget, wallet.address);
  const signer = new ethers.NonceManager(wallet);
  const fheTest = new ethers.Contract(targetConfig.fheTestAddress, FHETEST_ABI, signer) as unknown as FheTestContract;
  const chain = resolveChain(options.chainTarget);
  const client =
    options.chainTarget === 'localcleartext'
      ? createFhevmCleartextClient({ chain, provider, options: { moduleVersions } })
      : createFhevmClient({ chain, provider, options: { moduleVersions } });

  log(`Target: ${options.chainTarget}`);
  log(`RPC URL: ${targetConfig.rpcUrl}`);
  log(`FHETest: ${targetConfig.fheTestAddress}`);
  log(`Signer: ${wallet.address}`);
  log(`WASM: TFHE ${versionPair.tfhe}, TKMS ${versionPair.kms}, ${options.wasmAssetLoadMode}, ${options.assetSource}`);
  log(`Threads: ${options.threaded ? 'threaded' : 'single-thread'}`);

  return {
    client,
    fheTest,
    fheTestAddress: targetConfig.fheTestAddress,
    provider,
    signer,
    walletAddress: wallet.address,
  };
}

async function assertContractDeployed(
  provider: ethers.JsonRpcProvider,
  address: string,
  contractName: string,
  chainTarget: ChainTarget,
): Promise<void> {
  const code = await provider.getCode(address);
  if (code === '0x') {
    throw new Error(
      `${contractName} is not deployed at ${address} on ${chainTarget}. ` +
        'Restart/redeploy the selected local test stack so test/chains/chain-defaults.json matches the RPC state.',
    );
  }
}

async function ensureSignerFunds(chainTarget: ChainTarget, address: string): Promise<void> {
  if (chainTarget === 'testnet') {
    return;
  }

  log(`Funding local signer with anvil_setBalance: ${address}`);
  const response = await fetch('/__browser_ui/ensure-funded', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ chainTarget, address }),
  });

  if (!response.ok) {
    throw new Error(`Failed to fund signer: ${await response.text()}`);
  }

  const result = (await response.json()) as { readonly funded: boolean; readonly balanceWei?: string };
  if (result.funded) {
    log(`Signer balance set to ${result.balanceWei ?? 'local funding balance'}.`);
  }
}

function applyRuntimeConfig(matrix_: Matrix, versionPair: VersionPair, options: FormOptions): void {
  const assetUrls =
    options.wasmAssetLoadMode === 'embedded-base64'
      ? undefined
      : resolveAssetUrls(matrix_, versionPair, options.assetSource);
  const key = JSON.stringify({
    mode: options.wasmAssetLoadMode,
    assetUrls,
    threaded: options.threaded,
  });

  if (runtimeConfigKey !== undefined) {
    if (runtimeConfigKey !== key) {
      throw new Error('WASM runtime options changed after initialization. Reload the page before using new options.');
    }
    return;
  }

  setFhevmRuntimeConfig({
    wasmAssetLoadMode: options.wasmAssetLoadMode,
    singleThread: !options.threaded,
    numberOfThreads: options.threaded ? Math.max(2, Math.min(navigator.hardwareConcurrency || 4, 8)) : undefined,
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
        if (cause !== undefined) log(`[error] ${String(cause)}`);
      },
    },
  });
  runtimeConfigKey = key;
}

function resolveChain(target: ChainTarget): FhevmChain {
  switch (target) {
    case 'testnet':
      return sepolia;
    case 'localstack':
      return defineFhevmChain({
        id: 12_345,
        fhevm: {
          contracts: {
            acl: { address: '0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c' },
            inputVerifier: { address: '0x857Ca72A957920Fa0FB138602995839866Bd4005' },
            kmsVerifier: { address: '0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11' },
            protocolConfig: undefined, // To be filled
          },
          relayerUrl: LOCALSTACK_RELAYER_URL,
          gateway: {
            id: 54_321,
            contracts: {
              decryption: { address: '0xF0bFB159C7381F7CB332586004d8247252C5b816' },
              inputVerification: { address: '0x35760912360E875DA50D40a74305575c23D55783' },
            },
          },
        },
      });
    case 'localcleartext':
      return defineFhevmChain({
        id: 31_337,
        fhevm: {
          contracts: {
            acl: { address: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D' },
            inputVerifier: { address: '0x36772142b74871f255CbD7A3e89B401d3e45825f' },
            kmsVerifier: { address: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030' },
            protocolConfig: undefined, // To be filled
          },
          relayerUrl: 'http://localhost:8545',
          gateway: {
            id: 654_321,
            contracts: {
              decryption: { address: '0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721' },
              inputVerification: { address: '0x6189F6c0c3E40B4a3c72ec86262295D78d845297' },
            },
          },
        },
      });
  }
}

function readFormOptions(): FormOptions {
  const value = parseUint64(elements.valueInput.value);
  return {
    value,
    chainTarget: elements.chainTarget.value as ChainTarget,
    wasmAssetLoadMode: elements.wasmAssetLoadMode.value as WasmAssetLoadMode,
    tfhe: elements.tfheVersion.value as TfheVersion,
    kms: elements.kmsVersion.value as TkmsVersion,
    assetSource: elements.assetSource.value as AssetSource,
    threaded: elements.threadingMode.value === 'threaded',
    manualInit: elements.initMode.value === 'manual',
    publicDecrypt: elements.decryptMode.value === 'both' || elements.decryptMode.value === 'public',
    userDecrypt: elements.decryptMode.value === 'both' || elements.decryptMode.value === 'user',
  };
}

function persistFormSettings(): void {
  const settings = {
    value: demoState === undefined ? elements.valueInput.value : demoState.value.toString(),
    chainTarget: elements.chainTarget.value,
    wasmAssetLoadMode: elements.wasmAssetLoadMode.value,
    tfheVersion: elements.tfheVersion.value,
    kmsVersion: elements.kmsVersion.value,
    assetSource: elements.assetSource.value,
    threadingMode: elements.threadingMode.value,
    initMode: elements.initMode.value,
    decryptMode: elements.decryptMode.value,
  };

  sessionStorage.setItem(FORM_SETTINGS_STORAGE_KEY, JSON.stringify(settings));
}

function restorePersistedFormSettings(): void {
  const rawSettings = sessionStorage.getItem(FORM_SETTINGS_STORAGE_KEY);
  if (rawSettings === null) {
    return;
  }
  sessionStorage.removeItem(FORM_SETTINGS_STORAGE_KEY);

  const settings = JSON.parse(rawSettings) as Partial<Record<string, string>>;
  if (settings.value !== undefined) {
    elements.valueInput.value = settings.value;
  }
  setSelectValueIfPresent(elements.chainTarget, settings.chainTarget);
  setSelectValueIfPresent(elements.wasmAssetLoadMode, settings.wasmAssetLoadMode);
  setSelectValueIfPresent(elements.tfheVersion, settings.tfheVersion);
  setSelectValueIfPresent(elements.kmsVersion, settings.kmsVersion);
  setSelectValueIfPresent(elements.assetSource, settings.assetSource);
  setSelectValueIfPresent(elements.threadingMode, settings.threadingMode);
  setSelectValueIfPresent(elements.initMode, settings.initMode);
  setSelectValueIfPresent(elements.decryptMode, settings.decryptMode);
}

function setSelectValueIfPresent(select: HTMLSelectElement, value: string | undefined): void {
  if (value !== undefined && Array.from(select.options).some((option) => option.value === value)) {
    select.value = value;
  }
}

function isWasmRuntimeControl(target: EventTarget | null): target is HTMLSelectElement {
  return target instanceof HTMLSelectElement && WASM_RUNTIME_CONTROL_IDS.has(target.id);
}

type FormOptions = {
  readonly value: bigint;
  readonly chainTarget: ChainTarget;
  readonly wasmAssetLoadMode: WasmAssetLoadMode;
  readonly tfhe: TfheVersion;
  readonly kms: TkmsVersion;
  readonly assetSource: AssetSource;
  readonly threaded: boolean;
  readonly manualInit: boolean;
  readonly publicDecrypt: boolean;
  readonly userDecrypt: boolean;
};

type DecryptOptions = Pick<FormOptions, 'publicDecrypt' | 'userDecrypt'>;

function readDecryptOptions(): DecryptOptions {
  return {
    publicDecrypt: elements.decryptMode.value === 'both' || elements.decryptMode.value === 'public',
    userDecrypt: elements.decryptMode.value === 'both' || elements.decryptMode.value === 'user',
  };
}

function validateDecryptionSelection(options: DecryptOptions): void {
  if (!options.publicDecrypt && !options.userDecrypt) {
    throw new Error('Select public decrypt, user decrypt, or both.');
  }
}

async function loadMatrix(): Promise<Matrix> {
  const response = await fetch('/test/multi-wasm/matrix.json');
  if (!response.ok) {
    throw new Error(`Failed to load multi-wasm matrix: HTTP ${response.status}`);
  }
  return (await response.json()) as Matrix;
}

async function loadBrowserUiConfig(): Promise<BrowserUiConfig> {
  const response = await fetch('/__browser_ui/config');
  if (!response.ok) {
    throw new Error(`Failed to load browser UI config: HTTP ${response.status}`);
  }
  return (await response.json()) as BrowserUiConfig;
}

function resolveVersionPair(
  matrix_: Matrix,
  tfhe: TfheVersion,
  kms: TkmsVersion,
  assetSource: AssetSource,
): VersionPair {
  const pair = matrix_.supportedVersionPairs.find((candidate) => candidate.tfhe === tfhe && candidate.kms === kms);
  if (pair === undefined) {
    throw new Error(`Unsupported TFHE/KMS pair: ${tfhe} / ${kms}.`);
  }
  if (pair.cdns !== undefined && !pair.cdns.includes(assetSource)) {
    throw new Error(`Asset source ${assetSource} is not enabled for TFHE ${tfhe} / KMS ${kms}.`);
  }
  return pair;
}

function resolveAssetUrls(matrix_: Matrix, versionPair: VersionPair, assetSource: AssetSource): AssetUrlSet {
  const template = matrix_.assetUrlSets[assetSource];
  if (template === undefined) {
    throw new Error(`Unknown asset source: ${assetSource}`);
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

function parseUint64(value: string): bigint {
  const normalized = value.trim();
  if (!/^\d+$/.test(normalized)) {
    throw new Error('Value must be a base-10 uint64.');
  }
  const parsed = BigInt(normalized);
  if (parsed > UINT64_MAX) {
    throw new Error(`Value exceeds uint64 max (${UINT64_MAX.toString()}).`);
  }
  return parsed;
}

function assertRoundTrip(label: string, actual: unknown, expected: bigint): void {
  if (String(actual) !== expected.toString()) {
    throw new Error(`${label} mismatch: expected ${expected.toString()}, got ${String(actual)}`);
  }
}

function renderSelectOptions(select: HTMLSelectElement, values: readonly string[]): void {
  select.textContent = '';
  for (const value of values) {
    const option = document.createElement('option');
    option.value = value;
    option.textContent = value;
    select.append(option);
  }
}

let customSelectDocumentHandlersAttached = false;

function enhanceSelects(selects: Iterable<HTMLSelectElement>): void {
  for (const select of selects) {
    if (select.dataset.customSelect === 'true') {
      continue;
    }

    select.dataset.customSelect = 'true';
    select.classList.add('native-select');

    const dropdown = document.createElement('div');
    dropdown.className = 'custom-select';

    const button = document.createElement('button');
    button.type = 'button';
    button.className = 'custom-select-button';
    button.setAttribute('aria-haspopup', 'listbox');
    button.setAttribute('aria-expanded', 'false');

    const menu = document.createElement('div');
    menu.className = 'custom-select-menu';
    menu.role = 'listbox';
    menu.hidden = true;

    dropdown.append(button, menu);
    select.insertAdjacentElement('afterend', dropdown);

    const sync = (): void => {
      button.textContent = selectedOptionText(select);
      renderCustomSelectOptions(select, menu);
    };

    button.addEventListener('click', () => {
      const willOpen = menu.hidden;
      closeAllCustomSelects();
      if (willOpen) {
        dropdown.dataset.open = 'true';
        menu.hidden = false;
        button.setAttribute('aria-expanded', 'true');
      }
    });

    select.addEventListener('change', sync);
    sync();
  }

  if (!customSelectDocumentHandlersAttached) {
    customSelectDocumentHandlersAttached = true;
    document.addEventListener('click', (event) => {
      if (!(event.target instanceof Element) || event.target.closest('.custom-select') === null) {
        closeAllCustomSelects();
      }
    });
    document.addEventListener('keydown', (event) => {
      if (event.key === 'Escape') {
        closeAllCustomSelects();
      }
    });
  }
}

function renderCustomSelectOptions(select: HTMLSelectElement, menu: HTMLElement): void {
  menu.textContent = '';

  for (const option of select.options) {
    const item = document.createElement('button');
    item.type = 'button';
    item.className = 'custom-select-option';
    item.role = 'option';
    item.textContent = option.textContent;
    item.dataset.value = option.value;
    item.setAttribute('aria-selected', String(option.selected));

    item.addEventListener('click', () => {
      select.value = option.value;
      select.dispatchEvent(new Event('change', { bubbles: true }));
      closeAllCustomSelects();
    });

    menu.append(item);
  }
}

function selectedOptionText(select: HTMLSelectElement): string {
  return select.selectedOptions[0]?.textContent ?? select.value;
}

function closeAllCustomSelects(): void {
  for (const dropdown of document.querySelectorAll<HTMLElement>('.custom-select[data-open="true"]')) {
    const button = dropdown.querySelector<HTMLButtonElement>('.custom-select-button');
    const menu = dropdown.querySelector<HTMLElement>('.custom-select-menu');
    dropdown.removeAttribute('data-open');
    if (button !== null) {
      button.setAttribute('aria-expanded', 'false');
    }
    if (menu !== null) {
      menu.hidden = true;
    }
  }
}

function unique(values: readonly string[]): readonly string[] {
  return [...new Set(values)];
}

function requireLoaded<T>(value: T | undefined, name: string): T {
  if (value === undefined) {
    throw new Error(`${name} is not loaded.`);
  }
  return value;
}

function setBusy(_isBusy: boolean, _message: string): void {
  // Intentionally non-visual: progress is shown in the log and primary button state.
}

function resetResults(): void {
  setText(elements.inputHandle, '-');
  setText(elements.storedHandle, '-');
  setText(elements.transactionHash, '-');
}

function restartWorkflow(options: { readonly clearLog?: boolean; readonly resetInput: boolean }): void {
  demoState = undefined;
  if (options.resetInput) {
    elements.valueInput.value = defaultInputValue;
  }
  if (options.clearLog === true) {
    elements.logOutput.textContent = '';
  }
  resetResults();
  setEncryptMode();
  setBusy(false, '');
}

function setEncryptMode(options: { readonly restartEnabled?: boolean } = {}): void {
  elements.valueInputLabel.textContent = 'uint64 value';
  elements.valueInput.readOnly = false;
  elements.valueInput.classList.remove('handle-input');
  setPrimaryButtonLabel('Encrypt');
  elements.restartButton.disabled = options.restartEnabled !== true;
}

function setDecryptMode(): void {
  elements.valueInputLabel.textContent = 'Encrypted handle';
  elements.valueInput.readOnly = true;
  elements.valueInput.classList.add('handle-input');
  setPrimaryButtonLabel('Decrypt');
  elements.restartButton.disabled = false;
}

function setPrimaryButtonLabel(label: string): void {
  elements.encryptButton.classList.remove('is-loading');
  elements.encryptButton.replaceChildren(document.createTextNode(label));
}

function setPrimaryButtonLoading(label: string): void {
  const spinner = document.createElement('span');
  spinner.className = 'button-spinner';
  spinner.setAttribute('aria-hidden', 'true');

  const text = document.createElement('span');
  text.textContent = label;

  elements.encryptButton.classList.add('is-loading');
  elements.encryptButton.replaceChildren(spinner, text);
}

function setText(element: Element, text: string): void {
  element.textContent = text;
}

function log(message: string, level: 'info' | 'error' = 'info'): void {
  const elapsed = (performance.now() - t0).toFixed(0).padStart(5, ' ');
  const line = `[${elapsed}ms] ${message}\n`;
  if (level === 'error') {
    const span = document.createElement('span');
    span.className = 'log-error';
    span.textContent = line;
    elements.logOutput.append(span);
  } else {
    elements.logOutput.append(document.createTextNode(line));
  }
  elements.logOutput.scrollTop = elements.logOutput.scrollHeight;
  console.log(`[browser-ui] ${message}`);
}

function logError(err: unknown): void {
  const message = err instanceof Error ? (err.stack ?? err.message) : String(err);
  log(message, 'error');
  console.error(err);
}

function byId<T extends HTMLElement>(id: string): T {
  const element = document.getElementById(id);
  if (element === null) {
    throw new Error(`Missing element #${id}.`);
  }
  return element as T;
}

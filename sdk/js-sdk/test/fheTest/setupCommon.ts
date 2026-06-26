import type { FhevmChain } from '@fhevm/sdk/chains';
import type { EncryptedValue, TypedValue } from '@fhevm/sdk/types';
import type { ProtocolVersion } from '../../src/core/types/coreFhevmClient.js';
import type { FhevmModuleVersions } from '../../src/core/types/moduleVersions.js';
import { execFileSync } from 'node:child_process';
import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { mnemonicToAccount } from 'viem/accounts';
import { localcleartext } from '../chains/localcleartext.js';
import { localstack } from '../chains/localstack.js';
import { localstack_v11 } from '../chains/localstack_v11.js';
import { localstack_v12 } from '../chains/localstack_v12.js';
import { localstack_v13 } from '../chains/localstack_v13.js';
import { localstack_v14 } from '../chains/localstack_v14.js';
import { devnet } from '../chains/devnet.js';
import { polygon_devnet } from '../chains/polygon_devnet.js';
import { mainnet, sepolia, sepolia as testnet } from '@fhevm/sdk/chains';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export const FHE_TEST_CHAIN_NAMES = [
  'sepolia',
  'testnet',
  'mainnet',
  'devnet',
  'localcleartext',
  'localcleartext_v12',
  'localcleartext_v13',
  'localstack',
  'localstack_v11',
  'localstack_v12',
  'localstack_v13',
  'localstack_v14',
  'polygon_devnet',
] as const;

export type FheTestChainName = (typeof FHE_TEST_CHAIN_NAMES)[number];

export type FheTestVersion = 'v1' | 'v2';

export type FheTestBaseEnv = {
  readonly chainName: FheTestChainName;
  readonly fhevmChain: FhevmChain;
  readonly rpcUrl: string;
  readonly mnemonic: string;
  readonly zamaApiKey: string;
  readonly fheTestAddress: string;
  readonly fheTestVersion: FheTestVersion;
  readonly protocolVersion: ProtocolVersion;
  readonly fheEncryptionKeyTfheVersion: string;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
};

// ---------------------------------------------------------------------------
// FHETest contract version
// ---------------------------------------------------------------------------

export function isCleartext(chainName: FheTestChainName) {
  return chainName === 'localcleartext' || chainName.startsWith('localcleartext_');
}

// ---------------------------------------------------------------------------
// Protocol version per chain
// ---------------------------------------------------------------------------

const PROTOCOL_VERSION_BY_CHAIN: Readonly<Record<FheTestChainName, ProtocolVersion>> = {
  sepolia: '0.13.0',
  testnet: '0.13.0',
  mainnet: '0.11.0',
  localcleartext: '0.13.0',
  localcleartext_v12: '0.12.0',
  localcleartext_v13: '0.13.0',
  localstack: '0.14.0',
  localstack_v11: '0.11.0',
  localstack_v12: '0.12.0',
  localstack_v13: '0.13.0',
  localstack_v14: '0.14.0',
  devnet: '0.13.0',
  polygon_devnet: '0.13.0',
};

export function getExpectedProtocolVersion(chainName: FheTestChainName): ProtocolVersion {
  return PROTOCOL_VERSION_BY_CHAIN[chainName];
}

// ---------------------------------------------------------------------------
// TFHE wasm version per chain
// ---------------------------------------------------------------------------

export type TfheVersion = '1.5.3' | '1.6.2';

const TFHE_VERSION_BY_CHAIN: Readonly<Record<FheTestChainName, TfheVersion | undefined>> = {
  sepolia: '1.5.3',
  testnet: '1.5.3', // alias for sepolia
  mainnet: '1.5.3',
  localcleartext: undefined,
  localcleartext_v12: undefined,
  localcleartext_v13: undefined,
  localstack_v11: '1.5.3',
  localstack_v12: '1.5.3',
  devnet: '1.6.2',
  polygon_devnet: '1.6.2',
  localstack: '1.6.2',
  localstack_v13: '1.6.2',
  localstack_v14: '1.6.2',
};

/** Returns the TFHE wasm version for a given test chain, or `undefined` for cleartext chains. */
export function getTfheVersion(chainName: FheTestChainName): TfheVersion | undefined {
  return TFHE_VERSION_BY_CHAIN[chainName];
}

const FHE_ENCRYPTION_KEY_TFHE_VERSION_BY_CHAIN: Readonly<Partial<Record<FheTestChainName, string>>> = {
  sepolia: '1.4.0-alpha.3',
  testnet: '1.4.0-alpha.3',
  localcleartext: 'cleartext',
  localcleartext_v12: 'cleartext',
  localcleartext_v13: 'cleartext',
  localstack_v11: '1.5.1',
  localstack_v12: '1.5.4',
  localstack_v13: '1.6.1',
};

export function getFheEncryptionKeyTfheVersion(chainName: FheTestChainName): string {
  return FHE_ENCRYPTION_KEY_TFHE_VERSION_BY_CHAIN[chainName] ?? getTfheVersion(chainName) ?? 'unknown';
}

// ---------------------------------------------------------------------------
// .env parser (no external dependency)
// ---------------------------------------------------------------------------

function parseEnvFile(filePath: string): Record<string, string> {
  if (!existsSync(filePath)) {
    return {};
  }
  const content = readFileSync(filePath, 'utf-8');
  const result: Record<string, string> = {};
  for (const line of content.split('\n')) {
    const trimmed = line.trim();
    if (trimmed === '' || trimmed.startsWith('#')) {
      continue;
    }
    const eqIndex = trimmed.indexOf('=');
    if (eqIndex === -1) {
      continue;
    }
    const key = trimmed.slice(0, eqIndex).trim();
    let value = trimmed.slice(eqIndex + 1).trim();
    // Strip surrounding quotes
    if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
      value = value.slice(1, -1);
    }
    result[key] = value;
  }
  return result;
}

// ---------------------------------------------------------------------------
// Resolve chain
// ---------------------------------------------------------------------------

function resolveChainNames(): FheTestChainName[] {
  const raw = process.env.CHAIN ?? 'sepolia';
  const known = FHE_TEST_CHAIN_NAMES as readonly string[];

  const entries = raw
    .split(',')
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0);

  if (entries.length === 0) {
    throw new Error(`Invalid CHAIN env var: "${raw}". Expected one or more comma-separated chain names.`);
  }

  const invalid = entries.filter((entry) => !known.includes(entry));
  if (invalid.length > 0) {
    throw new Error(
      `Invalid CHAIN env var: ${invalid.map((c) => `"${c}"`).join(', ')}. ` +
        `Expected one of: ${FHE_TEST_CHAIN_NAMES.map((c) => `"${c}"`).join(', ')}.`,
    );
  }

  return Array.from(new Set(entries)) as FheTestChainName[];
}

// ---------------------------------------------------------------------------
// Resolve FHETest address
// ---------------------------------------------------------------------------

function resolveFHETestAddress(chainName: FheTestChainName): string {
  const defaults = loadChainDefaults()[chainName];
  if (!defaults?.fheTestAddress) {
    throw new Error(`Missing fheTestAddress for "${chainName}" in test/chains/chain-defaults.json.`);
  }
  return defaults.fheTestAddress;
}

function resolveFHETestVersion(chainName: FheTestChainName): FheTestVersion {
  const defaults = loadChainDefaults()[chainName];
  if (!defaults?.fheTestVersion) {
    throw new Error(`Missing fheTestVersion for "${chainName}" in test/chains/chain-defaults.json.`);
  }
  return defaults.fheTestVersion;
}

// ---------------------------------------------------------------------------
// Build base env
// ---------------------------------------------------------------------------

let _baseEnv: FheTestBaseEnv | undefined;

type ChainDefaults = {
  readonly rpcUrl?: string;
  readonly mnemonic?: string;
  readonly fheTestAddress: string;
  readonly fheTestVersion: FheTestVersion;
};

let _chainDefaults: Partial<Record<FheTestChainName, ChainDefaults>> | undefined;

function loadChainDefaults(): Partial<Record<FheTestChainName, ChainDefaults>> {
  if (_chainDefaults === undefined) {
    const p = resolve(__dirname, '../chains/chain-defaults.json');
    _chainDefaults = JSON.parse(readFileSync(p, 'utf-8')) as Partial<Record<FheTestChainName, ChainDefaults>>;
  }
  return _chainDefaults;
}

const DEFAULT_ANVIL_FUNDING_BALANCE_WEI = 10_000n * 10n ** 18n;
const DEFAULT_MNEMONIC_DERIVATION_PATH = "m/44'/60'/0'/0/0";
const FHETEST_CONTRACT_NAME = 'FHETestv2';
const FHETEST_INIT_FHE_TYPE_IDS = [0, 2, 3, 4, 5, 6, 8, 7] as const;

function foundryCastEnv(): NodeJS.ProcessEnv {
  const { CHAIN: _chain, ...env } = process.env;
  return env;
}

function isLocalCleartextChain(chainName: FheTestChainName): boolean {
  return chainName === 'localcleartext' || chainName.startsWith('localcleartext_');
}

function isLocalAnvilChain(chainName: FheTestChainName): boolean {
  return isLocalCleartextChain(chainName) || chainName.startsWith('localstack');
}

function tryFoundryCast(args: readonly string[]): string | undefined {
  try {
    return execFileSync('cast', [...args], {
      encoding: 'utf-8',
      env: foundryCastEnv(),
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim();
  } catch {
    return undefined;
  }
}

function foundryCast(args: readonly string[], errorMessage: string): string {
  try {
    return execFileSync('cast', [...args], {
      encoding: 'utf-8',
      env: foundryCastEnv(),
      stdio: ['ignore', 'pipe', 'pipe'],
    }).trim();
  } catch (error) {
    const maybeExecError = error as { stderr?: unknown; message?: unknown };
    const stderr =
      typeof maybeExecError.stderr === 'string'
        ? maybeExecError.stderr.trim()
        : Buffer.isBuffer(maybeExecError.stderr)
          ? maybeExecError.stderr.toString('utf-8').trim()
          : '';
    const message = typeof maybeExecError.message === 'string' ? maybeExecError.message : '';
    const details = stderr || message;
    throw new Error(details ? `${errorMessage}\n${details}` : errorMessage);
  }
}

function isAnvilRpc(rpcUrl: string): boolean {
  return tryFoundryCast(['rpc', 'web3_clientVersion', '--rpc-url', rpcUrl])?.toLowerCase().includes('anvil') === true;
}

function getAccountBalanceWei(address: string, rpcUrl: string): bigint {
  const balance = foundryCast(
    ['balance', address, '--rpc-url', rpcUrl],
    `Failed to read balance for ${address} on ${rpcUrl}.`,
  );
  return BigInt(balance);
}

function getMnemonicPrivateKey(mnemonic: string): string {
  return foundryCast(
    ['wallet', 'private-key', '--mnemonic', mnemonic, '--mnemonic-derivation-path', DEFAULT_MNEMONIC_DERIVATION_PATH],
    'Failed to derive the mnemonic private key.',
  );
}

function assertFheTestIsDeployed(fheTestAddress: string, rpcUrl: string): void {
  const code = foundryCast(
    ['code', fheTestAddress, '--rpc-url', rpcUrl],
    `Failed to read FHETest bytecode at ${fheTestAddress} on ${rpcUrl}.`,
  );
  if (code === '' || code === '0x') {
    throw new Error(`FHETest is not deployed at ${fheTestAddress} on ${rpcUrl}.`);
  }

  const contractName = foundryCast(
    ['call', fheTestAddress, 'CONTRACT_NAME()(string)', '--rpc-url', rpcUrl],
    `Failed to call FHETest CONTRACT_NAME() at ${fheTestAddress} on ${rpcUrl}.`,
  );
  if (contractName !== FHETEST_CONTRACT_NAME && contractName !== `"${FHETEST_CONTRACT_NAME}"`) {
    throw new Error(`Unexpected FHETest CONTRACT_NAME() at ${fheTestAddress}: ${contractName}.`);
  }
}

function aliceHasAllFheTestHandles(fheTestAddress: string, aliceAddress: string, rpcUrl: string): boolean {
  for (const fheTypeId of FHETEST_INIT_FHE_TYPE_IDS) {
    const hasHandle = foundryCast(
      [
        'call',
        fheTestAddress,
        'hasHandleOf(address,uint8)(bool)',
        aliceAddress,
        String(fheTypeId),
        '--rpc-url',
        rpcUrl,
      ],
      `Failed to call FHETest hasHandleOf(${aliceAddress}, ${fheTypeId}) at ${fheTestAddress} on ${rpcUrl}.`,
    );
    if (hasHandle !== 'true') {
      return false;
    }
  }
  return true;
}

function initAliceFheTestHandlesIfNeeded(
  fheTestAddress: string,
  aliceAddress: string,
  mnemonic: string,
  rpcUrl: string,
): void {
  if (aliceHasAllFheTestHandles(fheTestAddress, aliceAddress, rpcUrl)) {
    return;
  }

  const privateKey = getMnemonicPrivateKey(mnemonic);
  foundryCast(
    ['send', fheTestAddress, 'initFheTest(bool)', 'false', '--rpc-url', rpcUrl, '--private-key', privateKey],
    `Failed to initialize FHETest handles for ${aliceAddress} at ${fheTestAddress} on ${rpcUrl}.`,
  );

  if (!aliceHasAllFheTestHandles(fheTestAddress, aliceAddress, rpcUrl)) {
    throw new Error(`FHETest initFheTest(false) completed but Alice handles are still missing for ${aliceAddress}.`);
  }

  // The coprocessor registers Alice's handles on the gateway CiphertextCommits contract
  // asynchronously. Tests that call the relayer for public/user decryption need
  // this registration to complete before the relayer's readiness check passes.
  // We block the setup thread briefly so the coprocessor can catch up.
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 30_000);
}

/**
 * Funds the mnemonic account when local tests are pointed at Anvil and the
 * account has no gas for write transactions.
 */
export function fundMnemonicIfEmptyOnAnvil(
  chainName: FheTestChainName,
  mnemonic: string,
  rpcUrl: string,
  balanceWei: bigint = DEFAULT_ANVIL_FUNDING_BALANCE_WEI,
): void {
  if (!isLocalAnvilChain(chainName) || !isAnvilRpc(rpcUrl)) {
    return;
  }

  const account = mnemonicToAccount(mnemonic);
  if (getAccountBalanceWei(account.address, rpcUrl) > 0n) {
    return;
  }

  const hexBalance = `0x${balanceWei.toString(16)}`;
  foundryCast(
    ['rpc', 'anvil_setBalance', account.address, hexBalance, '--rpc-url', rpcUrl],
    `Failed to fund mnemonic account ${account.address} on ${rpcUrl}.`,
  );
}

/**
 * Performs the local-chain preflight needed before FHETest suites run:
 * verifies FHETest is deployed, funds Alice on Anvil if needed, and initializes
 * Alice's FHETest handles when they are missing.
 */
export function runPreliminaryFheTestSetup(
  chainName: FheTestChainName,
  mnemonic: string,
  rpcUrl: string,
  fheTestAddress: string,
): void {
  if (!isLocalAnvilChain(chainName)) {
    return;
  }
  if (!isAnvilRpc(rpcUrl)) {
    throw new Error(`Expected "${chainName}" RPC ${rpcUrl} to be an Anvil node.`);
  }

  const alice = mnemonicToAccount(mnemonic);
  assertFheTestIsDeployed(fheTestAddress, rpcUrl);

  fundMnemonicIfEmptyOnAnvil(chainName, mnemonic, rpcUrl);

  if (getAccountBalanceWei(alice.address, rpcUrl) === 0n) {
    throw new Error(`Alice ${alice.address} has zero balance on ${rpcUrl}.`);
  }

  initAliceFheTestHandlesIfNeeded(fheTestAddress, alice.address, mnemonic, rpcUrl);
}

export function prepareSingleChain(): FheTestBaseEnv {
  return prepareChains()[0]!;
}

export function prepareChains(): FheTestBaseEnv[] {
  const chainNames: FheTestChainName[] = resolveChainNames();

  const chains: FheTestBaseEnv[] = [];
  for (let i = 0; i < chainNames.length; ++i) {
    const c = _prepareChain(chainNames[i]!);
    chains.push(c);
  }

  return chains;
}

function _prepareChain(chainName: FheTestChainName): FheTestBaseEnv {
  const testDir = resolve(__dirname, '..');
  const isLocalstack = chainName.startsWith('localstack');
  const envFilename = isLocalCleartextChain(chainName)
    ? '.env.localcleartext'
    : isLocalstack
      ? '.env.localstack'
      : `.env.${chainName}`;

  // Load shared secrets
  const sharedEnv = parseEnvFile(resolve(testDir, '.env'));
  // Load chain-specific env
  const chainEnv = parseEnvFile(resolve(testDir, envFilename));

  const defaults = loadChainDefaults()[chainName];

  const mnemonic = sharedEnv.MNEMONIC ?? process.env.MNEMONIC ?? defaults?.mnemonic;
  if (!mnemonic) {
    throw new Error(
      `MNEMONIC is missing for "${chainName}". Set it in test/.env, as the MNEMONIC env var, or add a mnemonic to test/chains/chain-defaults.json.`,
    );
  }

  const zamaApiKey = sharedEnv.ZAMA_FHEVM_API_KEY ?? process.env.ZAMA_FHEVM_API_KEY;
  if (!zamaApiKey) {
    throw new Error('ZAMA_FHEVM_API_KEY is missing. Set it in test/.env or as an environment variable.');
  }

  const rpcUrl = chainEnv.RPC_URL ?? process.env.RPC_URL ?? defaults?.rpcUrl;
  if (!rpcUrl) {
    throw new Error(
      `RPC_URL is missing for "${chainName}". Set it in test/${envFilename}, as the RPC_URL env var, or add an entry to test/chains/chain-defaults.json.`,
    );
  }

  const fheTestAddress = resolveFHETestAddress(chainName);
  const fheTestVersion = resolveFHETestVersion(chainName);

  const chainMap: Record<FheTestChainName, FhevmChain> = {
    localstack,
    localstack_v11,
    localstack_v12,
    localstack_v13,
    localstack_v14,
    localcleartext,
    localcleartext_v12: localcleartext,
    localcleartext_v13: localcleartext,
    polygon_devnet,
    sepolia,
    mainnet,
    devnet,
    testnet,
  };
  const fhevmChain = chainMap[chainName];
  if (!fhevmChain) {
    const valid = Object.keys(chainMap)
      .map((k) => `"${k}"`)
      .join(', ');
    throw new Error(`Unsupported chain: "${chainName}". Expected one of ${valid}.`);
  }

  runPreliminaryFheTestSetup(chainName, mnemonic, rpcUrl, fheTestAddress);

  const tfheVersion = getTfheVersion(chainName);
  _baseEnv = {
    chainName,
    fhevmChain,
    rpcUrl,
    mnemonic,
    zamaApiKey,
    fheTestAddress,
    fheTestVersion,
    protocolVersion: getExpectedProtocolVersion(chainName),
    fheEncryptionKeyTfheVersion: getFheEncryptionKeyTfheVersion(chainName),
    moduleVersions: tfheVersion === undefined ? undefined : { tfhe: tfheVersion },
  };

  return _baseEnv;
}

export function safeJSONstringify(o: unknown, space?: string | number): string {
  try {
    return JSON.stringify(o, (_, v: unknown) => (typeof v === 'bigint' ? v.toString() : v), space);
  } catch {
    return '';
  }
}

// ---------------------------------------------------------------------------
// Tools to check type of handles
// ---------------------------------------------------------------------------

const FheTypeNameToTypeId = {
  ebool: 0,
  //euint4: 1, has been deprecated
  euint8: 2,
  euint16: 3,
  euint32: 4,
  euint64: 5,
  euint128: 6,
  eaddress: 7,
  euint256: 8,
} as const;

export function fheTypeIdFromName(name: string): number {
  return FheTypeNameToTypeId[name as keyof typeof FheTypeNameToTypeId];
}

const FheTypeIdToClearType = {
  0: 'bool',
  //1 : 'uint4', has been deprecated
  2: 'uint8',
  3: 'uint16',
  4: 'uint32',
  5: 'uint64',
  6: 'uint128',
  7: 'address',
  8: 'uint256',
} as const;

export function clearTypeFromFheTypeId(typeId: number): string {
  return FheTypeIdToClearType[typeId as keyof typeof FheTypeIdToClearType];
}

export function fheTypeIdFromHandle(handle: EncryptedValue): number {
  const FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET = 30 * 2;
  const bytesHex = `0x${handle.slice(2 + FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET, 2 + FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET + 2)}`;
  return parseInt(bytesHex, 16) as number;
}

export function clearTypeFromHandle(handle: EncryptedValue): string {
  return clearTypeFromFheTypeId(fheTypeIdFromHandle(handle));
}

export function chainIdFromHandle(handle: EncryptedValue): bigint {
  const FHEVM_HANDLE_CHAINID_BYTE_OFFSET = 22 * 2;
  const bytesHex = `0x${handle.slice(2 + FHEVM_HANDLE_CHAINID_BYTE_OFFSET, 2 + FHEVM_HANDLE_CHAINID_BYTE_OFFSET + 8 * 2)}`;
  return BigInt(bytesHex);
}

export function is0x(s: string): s is `0x${string}` {
  return typeof s === 'string' && s.startsWith('0x');
}

export function isBytesHex(value: string, byteLength?: number): boolean {
  if (!is0x(value)) {
    return false;
  }

  if (byteLength !== undefined && value.length !== 2 * byteLength + 2) {
    return false;
  }

  if ((value.length - 2) % 2 !== 0) {
    return false;
  }

  const bytesHexRegex = /^0x[a-fA-F0-9]*$/;
  return bytesHexRegex.test(value);
}

export function isBytes32Hex(value: string): boolean {
  return isBytesHex(value, 32);
}

// ---------------------------------------------------------------------------
// List of fhe types involved in FHETest.sol
// ---------------------------------------------------------------------------

// Each FHE type to public-decrypt
export const decryptTestCases: readonly string[] = [
  'ebool',
  'euint8',
  'euint16',
  'euint32',
  'euint64',
  'euint128',
  'euint256',
  'eaddress',
] as const;

export const encryptTestCases: TypedValue[] = [
  { type: 'bool', value: true },
  { type: 'uint8', value: 42 },
  { type: 'uint16', value: 1234 },
  { type: 'uint32', value: 123456 },
  { type: 'uint64', value: 123456789n },
  { type: 'uint128', value: 123456789012345n },
  { type: 'uint256', value: 123456789012345678901234567890n },
  { type: 'address', value: '0x37AC010c1c566696326813b840319B58Bb5840E4' },
] as unknown as TypedValue[];

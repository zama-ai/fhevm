import type { FhevmChain } from '@fhevm/sdk/chains';
import type { EncryptedValue, TypedValue } from '@fhevm/sdk/types';
import { execFileSync } from 'node:child_process';
import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { mnemonicToAccount } from 'viem/accounts';
import { localcleartext } from './chains/localcleartext.js';
import { localstack } from './chains/localstack.js';
import { localstack_v11 } from './chains/localstack_v11.js';
import { localstack_v12 } from './chains/localstack_v12.js';
import { localstack_v13 } from './chains/localstack_v13.js';
import { localstack_v14 } from './chains/localstack_v14.js';
import { devnet } from './chains/devnet.js';
import { mainnet, sepolia, sepolia as testnet, polygonAmoy } from '@fhevm/sdk/chains';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export const FHE_TEST_CHAIN_NAMES = [
  'sepolia',
  'testnet',
  'mainnet',
  'devnet',
  'localcleartext',
  'localstack',
  'localstack_v11',
  'localstack_v12',
  'localstack_v13',
  'localstack_v14',
  'polygon_amoy',
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
};

// ---------------------------------------------------------------------------
// FHETest contract version
// ---------------------------------------------------------------------------

export function isV2(chainName: FheTestChainName) {
  return loadChainDefaults()[chainName]?.fheTestVersion === 'v2';
}

export function isCleartext(chainName: FheTestChainName) {
  return chainName === 'localcleartext';
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

function resolveChainName(): FheTestChainName {
  const chain = process.env.CHAIN ?? 'sepolia';
  if (!(FHE_TEST_CHAIN_NAMES as readonly string[]).includes(chain)) {
    throw new Error(
      `Invalid CHAIN env var: "${chain}". Expected one of: ${FHE_TEST_CHAIN_NAMES.map((c) => `"${c}"`).join(', ')}.`,
    );
  }
  return chain as FheTestChainName;
}

// ---------------------------------------------------------------------------
// Resolve FHETest address
// ---------------------------------------------------------------------------

function resolveFHETestAddress(chainName: FheTestChainName): string {
  const defaults = loadChainDefaults()[chainName];
  if (!defaults?.fheTestAddress) {
    throw new Error(`Missing fheTestAddress for "${chainName}" in test/fheTest/chains/chain-defaults.json.`);
  }
  return defaults.fheTestAddress;
}

function resolveFHETestVersion(chainName: FheTestChainName): FheTestVersion {
  const defaults = loadChainDefaults()[chainName];
  if (!defaults?.fheTestVersion) {
    throw new Error(`Missing fheTestVersion for "${chainName}" in test/fheTest/chains/chain-defaults.json.`);
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
    const p = resolve(__dirname, 'chains/chain-defaults.json');
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

function isLocalAnvilChain(chainName: FheTestChainName): boolean {
  return chainName === 'localcleartext' || chainName.startsWith('localstack');
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

export function prepareFheTestEnv(): FheTestBaseEnv {
  if (_baseEnv !== undefined) {
    return _baseEnv;
  }

  const testDir = resolve(__dirname, '..');
  const chainName: FheTestChainName = resolveChainName();
  const isLocalstack = chainName.startsWith('localstack');
  const envFilename = isLocalstack ? '.env.localstack' : `.env.${chainName}`;

  // Load shared secrets
  const sharedEnv = parseEnvFile(resolve(testDir, '.env'));
  // Load chain-specific env
  const chainEnv = parseEnvFile(resolve(testDir, envFilename));

  const defaults = loadChainDefaults()[chainName];

  const mnemonic = sharedEnv.MNEMONIC ?? process.env.MNEMONIC ?? defaults?.mnemonic;
  if (!mnemonic) {
    throw new Error(
      `MNEMONIC is missing for "${chainName}". Set it in test/.env, as the MNEMONIC env var, or add a mnemonic to test/fheTest/chains/chain-defaults.json.`,
    );
  }

  const zamaApiKey = sharedEnv.ZAMA_FHEVM_API_KEY ?? process.env.ZAMA_FHEVM_API_KEY;
  if (!zamaApiKey) {
    throw new Error('ZAMA_FHEVM_API_KEY is missing. Set it in test/.env or as an environment variable.');
  }

  const rpcUrl = chainEnv.RPC_URL ?? process.env.RPC_URL ?? defaults?.rpcUrl;
  if (!rpcUrl) {
    throw new Error(
      `RPC_URL is missing for "${chainName}". Set it in test/${envFilename}, as the RPC_URL env var, or add an entry to test/fheTest/chains/chain-defaults.json.`,
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
    polygon_amoy: polygonAmoy,
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

  _baseEnv = {
    chainName,
    fhevmChain,
    rpcUrl,
    mnemonic,
    zamaApiKey,
    fheTestAddress,
    fheTestVersion,
  };

  return _baseEnv;
}

// ---------------------------------------------------------------------------
// JSON.stringify with bigint support
// ---------------------------------------------------------------------------

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

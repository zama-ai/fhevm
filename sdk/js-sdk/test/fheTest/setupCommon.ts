import type { FhevmChain } from '@fhevm/sdk/chains';
import type { EncryptedValue, TypedValue } from '@fhevm/sdk/types';
import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { FHETestAddresses as FHETestAddressesv1 } from './abi-v1.js';
import { FHETestAddresses as FHETestAddressesv2 } from './abi-v2.js';
import { localhost } from './chains/localhost.js';
import { localhostFhevm } from './chains/localhostFhevm.js';
import { devnet } from './chains/devnet.js';
import { sepolia, mainnet } from '@fhevm/sdk/chains';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export type FheTestChainName = 'sepolia' | 'mainnet' | 'localhost' | 'localhostFhevm' | 'devnet';

export type FheTestBaseEnv = {
  readonly chainName: FheTestChainName;
  readonly fhevmChain: FhevmChain;
  readonly rpcUrl: string;
  readonly mnemonic: string;
  readonly zamaApiKey: string;
  readonly fheTestAddress: string;
};

// ---------------------------------------------------------------------------
// FHETest contract version
// ---------------------------------------------------------------------------

export function isV2(chainName: FheTestChainName) {
  return (
    chainName === 'localhostFhevm' || chainName === 'localhost' || chainName === 'devnet' || chainName === 'sepolia'
  );
}

export function isCleartext(chainName: FheTestChainName) {
  return chainName === 'localhost';
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
  if (
    chain !== 'sepolia' &&
    chain !== 'devnet' &&
    chain !== 'mainnet' &&
    chain !== 'localhost' &&
    chain !== 'localhostFhevm'
  ) {
    throw new Error(
      `Invalid CHAIN env var: "${chain}". Expected "sepolia", "mainnet", "devnet", "localhost" or "localhostFhevm".`,
    );
  }
  return chain;
}

// ---------------------------------------------------------------------------
// Resolve FHETest address
// ---------------------------------------------------------------------------

function resolveFHETestAddress(chainName: FheTestChainName): string {
  if (
    chainName === 'localhost' ||
    chainName === 'localhostFhevm' ||
    chainName === 'devnet' ||
    chainName === 'sepolia'
  ) {
    return FHETestAddressesv2[chainName];
  }
  return FHETestAddressesv1[chainName];
}

// ---------------------------------------------------------------------------
// Build base env
// ---------------------------------------------------------------------------

let _baseEnv: FheTestBaseEnv | undefined;

const LOCALHOST_FHEVM_MNEMONIC = 'test test test test test test test future home engine virtual motion';

export function getBaseEnv(): FheTestBaseEnv {
  if (_baseEnv !== undefined) {
    return _baseEnv;
  }

  const testDir = resolve(__dirname, '..');
  const chainName: FheTestChainName = resolveChainName();

  // Load shared secrets
  const sharedEnv = parseEnvFile(resolve(testDir, '.env'));
  // Load chain-specific env
  const chainEnv = parseEnvFile(resolve(testDir, `.env.${chainName}`));

  let mnemonic;
  if (chainName === 'localhostFhevm') {
    mnemonic = LOCALHOST_FHEVM_MNEMONIC;
  } else if (chainName === 'localhost') {
    mnemonic = LOCALHOST_FHEVM_MNEMONIC;
  } else {
    mnemonic = sharedEnv.MNEMONIC ?? process.env.MNEMONIC;
    if (!mnemonic) {
      throw new Error('MNEMONIC is missing. Set it in test/.env or as an environment variable.');
    }
  }

  const zamaApiKey = sharedEnv.ZAMA_FHEVM_API_KEY ?? process.env.ZAMA_FHEVM_API_KEY;
  if (!zamaApiKey) {
    throw new Error('ZAMA_FHEVM_API_KEY is missing. Set it in test/.env or as an environment variable.');
  }

  const rpcUrl = chainEnv.RPC_URL ?? process.env.RPC_URL;
  if (!rpcUrl) {
    throw new Error(`RPC_URL is missing. Set it in test/.env.${chainName} or as an environment variable.`);
  }

  const fheTestAddress = resolveFHETestAddress(chainName);

  const chainMap: Record<string, FhevmChain> = { localhostFhevm, localhost, sepolia, devnet, mainnet };
  const fhevmChain = chainMap[chainName];
  if (!fhevmChain) {
    const valid = Object.keys(chainMap)
      .map((k) => `"${k}"`)
      .join(', ');
    throw new Error(`Unsupported chain: "${chainName}". Expected one of ${valid}.`);
  }

  _baseEnv = {
    chainName,
    fhevmChain,
    rpcUrl,
    mnemonic,
    zamaApiKey,
    fheTestAddress,
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

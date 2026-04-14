import type { FhevmChain } from '@fhevm/sdk/chains';
import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { FHETestAddresses as FHETestAddressesv1 } from './abi-v1.js';
import { FHETestAddresses as FHETestAddressesv2 } from './abi-v2.js';
import { localhostFhevm } from './chains/localhostFhevm.js';
import { devnet } from './chains/devnet.js';
import { sepolia, mainnet } from '@fhevm/sdk/chains';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export type FheTestChainName = 'sepolia' | 'mainnet' | 'localhostFhevm' | 'devnet';

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
  return chainName === 'localhostFhevm' || chainName === 'devnet';
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
  if (chain !== 'sepolia' && chain !== 'devnet' && chain !== 'mainnet' && chain !== 'localhostFhevm') {
    throw new Error(`Invalid CHAIN env var: "${chain}". Expected "sepolia", "mainnet", "devnet" or "localhostFhevm".`);
  }
  return chain;
}

// ---------------------------------------------------------------------------
// Resolve FHETest address
// ---------------------------------------------------------------------------

function resolveFHETestAddress(chainName: FheTestChainName): string {
  if (chainName === 'localhostFhevm' || chainName === 'devnet') {
    return FHETestAddressesv2[chainName];
  }
  const key = chainName === 'sepolia' ? 'testnet' : chainName;
  return FHETestAddressesv1[key];
}

// ---------------------------------------------------------------------------
// Build base env
// ---------------------------------------------------------------------------

let _baseEnv: FheTestBaseEnv | undefined;

const FHEVM_MNEMONIC = 'test test test test test test test future home engine virtual motion';

export function getBaseEnv(): FheTestBaseEnv {
  if (_baseEnv !== undefined) {
    return _baseEnv;
  }

  const testDir = resolve(__dirname, '..');
  const chainName = resolveChainName();

  // Load shared secrets
  const sharedEnv = parseEnvFile(resolve(testDir, '.env'));
  // Load chain-specific env
  const chainEnv = parseEnvFile(resolve(testDir, `.env.${chainName}`));

  let mnemonic;
  if (chainName === 'localhostFhevm') {
    mnemonic = FHEVM_MNEMONIC;
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

  let fhevmChain: FhevmChain;
  if (chainName === 'localhostFhevm') {
    fhevmChain = localhostFhevm;
  } else if (chainName === 'sepolia') {
    fhevmChain = sepolia;
  } else if (chainName === 'devnet') {
    fhevmChain = devnet;
  } else if (chainName === 'mainnet') {
    fhevmChain = mainnet;
  } else {
    throw new Error(`Unsupported chain: "${chainName}". Expected "sepolia", "mainnet" or "localhostFhevm".`);
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

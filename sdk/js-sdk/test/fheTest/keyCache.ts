import type { FetchFheEncryptionKeyBytesReturnType as FheEncryptionKeyBytes } from '@fhevm/sdk/actions/chain';
import type { FheTestChainName } from './setupCommon.js';
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

////////////////////////////////////////////////////////////////////////////////

const KEYS_DIR = resolve(__dirname, '.keys');

export type CachedFheEncryptionKeyBytes = FheEncryptionKeyBytes & {
  readonly chain: FheTestChainName;
  readonly tfheVersion: string;
};

type ReadKeyFromCacheOptions = {
  readonly metadata?: FheEncryptionKeyBytes['metadata'] | undefined;
  readonly tfheVersion?: string | undefined;
};

function keyPath(chain: FheTestChainName): string {
  return resolve(KEYS_DIR, chain, 'key.json');
}

////////////////////////////////////////////////////////////////////////////////

export function hasKeyInCache(chain: FheTestChainName, options?: string | ReadKeyFromCacheOptions): boolean {
  return readKeyFromCache(chain, normalizeReadOptions(options)) !== undefined;
}

////////////////////////////////////////////////////////////////////////////////

export function readKeyFromCache(
  chain: FheTestChainName,
  options?: ReadKeyFromCacheOptions,
): CachedFheEncryptionKeyBytes | undefined {
  const path = keyPath(chain);
  if (!existsSync(path)) {
    return undefined;
  }

  const json = JSON.parse(readFileSync(path, 'utf-8')) as {
    chain?: unknown;
    metadata: { relayerUrl: string; chainId: number };
    publicKeyBytes: { id: string; bytes: string };
    crsBytes: { id: string; capacity: number; bytes: string };
    tfheVersion?: unknown;
  };

  if (json.chain !== chain || typeof json.tfheVersion !== 'string') {
    return undefined;
  }

  const cached = {
    chain,
    metadata: json.metadata,
    publicKeyBytes: {
      id: json.publicKeyBytes.id,
      bytes: Uint8Array.from(Buffer.from(json.publicKeyBytes.bytes, 'base64')),
    },
    crsBytes: {
      id: json.crsBytes.id,
      capacity: json.crsBytes.capacity,
      bytes: Uint8Array.from(Buffer.from(json.crsBytes.bytes, 'base64')),
    },
    tfheVersion: json.tfheVersion,
  } as CachedFheEncryptionKeyBytes;

  if (options?.tfheVersion !== undefined && cached.tfheVersion !== options.tfheVersion) {
    return undefined;
  }

  if (
    options?.metadata !== undefined &&
    (cached.metadata.chainId !== options.metadata.chainId || cached.metadata.relayerUrl !== options.metadata.relayerUrl)
  ) {
    return undefined;
  }

  return cached;
}

////////////////////////////////////////////////////////////////////////////////

export function writeKeyToCache(chain: FheTestChainName, bytes: FheEncryptionKeyBytes, tfheVersion: string): void {
  const path = keyPath(chain);
  const dir = resolve(KEYS_DIR, chain);

  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }

  const json = {
    chain,
    metadata: bytes.metadata,
    publicKeyBytes: {
      id: bytes.publicKeyBytes.id,
      bytes: Buffer.from(bytes.publicKeyBytes.bytes).toString('base64'),
    },
    crsBytes: {
      id: bytes.crsBytes.id,
      capacity: bytes.crsBytes.capacity,
      bytes: Buffer.from(bytes.crsBytes.bytes).toString('base64'),
    },
    tfheVersion,
  };

  writeFileSync(path, JSON.stringify(json, null, 2), 'utf-8');
}

function normalizeReadOptions(
  options: string | ReadKeyFromCacheOptions | undefined,
): ReadKeyFromCacheOptions | undefined {
  return typeof options === 'string' ? { tfheVersion: options } : options;
}

////////////////////////////////////////////////////////////////////////////////

export function clearKeyCache(chain?: FheTestChainName): void {
  if (chain !== undefined) {
    const dir = resolve(KEYS_DIR, chain);
    if (existsSync(dir)) {
      rmSync(dir, { recursive: true });
    }
  } else {
    if (existsSync(KEYS_DIR)) {
      rmSync(KEYS_DIR, { recursive: true });
    }
  }
}

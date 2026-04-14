import type { FetchFheEncryptionKeyBytesReturnType as FheEncryptionKeyBytes } from '@fhevm/sdk/actions/chain';
import type { FheTestChainName } from './ethers/setup.js';
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

////////////////////////////////////////////////////////////////////////////////

const KEYS_DIR = resolve(__dirname, '.keys');

function keyPath(chain: FheTestChainName): string {
  return resolve(KEYS_DIR, chain, 'key.json');
}

////////////////////////////////////////////////////////////////////////////////

export function hasKeyInCache(chain: FheTestChainName): boolean {
  return existsSync(keyPath(chain));
}

////////////////////////////////////////////////////////////////////////////////

export function readKeyFromCache(chain: FheTestChainName): FheEncryptionKeyBytes | undefined {
  const path = keyPath(chain);
  if (!existsSync(path)) {
    return undefined;
  }

  const json = JSON.parse(readFileSync(path, 'utf-8')) as {
    metadata: { relayerUrl: string; chainId: number };
    publicKeyBytes: { id: string; bytes: string };
    crsBytes: { id: string; capacity: number; bytes: string };
  };

  return {
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
  } as FheEncryptionKeyBytes;
}

////////////////////////////////////////////////////////////////////////////////

export function writeKeyToCache(chain: FheTestChainName, bytes: FheEncryptionKeyBytes): void {
  const path = keyPath(chain);
  const dir = resolve(KEYS_DIR, chain);

  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }

  const json = {
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
  };

  writeFileSync(path, JSON.stringify(json, null, 2), 'utf-8');
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

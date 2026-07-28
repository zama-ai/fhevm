// Mini-relayer key logic: load a `test/keys/key.<ver>.json` file and produce the
// exact relayer wire format the SDK expects.
//
// The SDK fetches keys in two steps (see
// src/core/modules/relayer/module/fetchFheEncryptionKeySource.ts):
//   1. GET {relayerUrl}/v2/keyurl  ->  JSON pointing at the byte URLs
//   2. GET <those URLs>            ->  the raw public-key / CRS bytes
//
// The CRS capacity is hard-coded to 2048 in the SDK, so we only ever serve the
// "2048" slot and assert the key file matches.

import { readFileSync } from 'node:fs';

export type LoadedKey = {
  readonly pub: { readonly id: string; readonly bytes: Uint8Array };
  readonly crs: { readonly id: string; readonly capacity: number; readonly bytes: Uint8Array };
};

/** Shape of the JSON files under `test/keys` (bytes are base64-encoded). */
type SerializedKeyFile = {
  readonly publicKeyBytes: { readonly id: string; readonly bytes: string };
  readonly crsBytes: { readonly id: string; readonly capacity: number; readonly bytes: string };
};

const _cache = new Map<string, LoadedKey>();

/** Loads and decodes a `test/keys/key.<ver>.json` file (memoized by path). */
export function loadKeyFile(path: string): LoadedKey {
  const cached = _cache.get(path);
  if (cached !== undefined) {
    return cached;
  }

  const json = JSON.parse(readFileSync(path, 'utf-8')) as SerializedKeyFile;
  const loaded: LoadedKey = {
    pub: { id: json.publicKeyBytes.id, bytes: _base64ToBytes(json.publicKeyBytes.bytes) },
    crs: {
      id: json.crsBytes.id,
      capacity: json.crsBytes.capacity,
      bytes: _base64ToBytes(json.crsBytes.bytes),
    },
  };

  if (loaded.crs.capacity !== 2048) {
    throw new Error(`Mini-relayer expects CRS capacity 2048, got ${String(loaded.crs.capacity)} for ${path}`);
  }

  _cache.set(path, loaded);
  return loaded;
}

/**
 * Builds the `/v2/keyurl` response body. `pubUrl` / `crsUrl` are the (same-origin)
 * URLs at which this gateway will serve the corresponding bytes.
 */
export function buildKeyUrlResponse(key: LoadedKey, pubUrl: string, crsUrl: string): unknown {
  return {
    response: {
      fheKeyInfo: [{ fhePublicKey: { dataId: key.pub.id, urls: [pubUrl] } }],
      crs: { '2048': { dataId: key.crs.id, urls: [crsUrl] } },
    },
  };
}

function _base64ToBytes(base64: string): Uint8Array {
  return Uint8Array.from(Buffer.from(base64, 'base64'));
}

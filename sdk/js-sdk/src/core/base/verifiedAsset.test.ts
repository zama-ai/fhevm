import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { describe, expect, it, vi } from 'vitest';
import type { Logger } from '../types/logger.js';
import { LoadableAsset, LoadableAssetList } from './verifiedAsset.js';

////////////////////////////////////////////////////////////////////////////////
//
// npx vitest run --config src/vitest.config.ts src/core/base/verifiedAsset.test.ts
//
////////////////////////////////////////////////////////////////////////////////

function createAsset(): LoadableAsset {
  return new LoadableAsset({
    name: 'tfhe-wasm',
    filename: 'asset.wasm',
    localRelativePath: 'v1/asset.wasm',
    sha256: '00',
  });
}

function createAssetList(): LoadableAssetList {
  return new LoadableAssetList([
    {
      name: 'tfhe-wasm-a',
      filename: 'asset-a.wasm',
      localRelativePath: 'v1/asset-a.wasm',
      sha256: '00',
    },
    {
      name: 'tfhe-wasm-b',
      filename: 'asset-b.wasm',
      localRelativePath: 'v1/asset-b.wasm',
      sha256: '11',
    },
  ]);
}

function throwingLogger(): Logger {
  return {
    debug: vi.fn(() => {
      throw new Error('logger failed');
    }),
    error: vi.fn(),
  };
}

function expectTerminalFallbackState(asset: LoadableAsset): void {
  expect(asset.resolved).toEqual(true);
  expect(asset.resolution).toEqual('none');
  expect(asset.url).toBeUndefined();
}

describe('VerifiedAsset', () => {
  it('stores an already validated resolved URL', () => {
    const asset = createAsset();
    const url = new URL('https://example.com/asset.wasm');

    asset.setResolved({ resolution: 'user', url });

    expect(asset.resolved).toEqual(true);
    expect(asset.resolution).toEqual('user');
    expect(asset.url).toBe(url);

    expect(() => asset.markUnavailable()).toThrow('asset already resolved');
  });

  it('marks unavailable assets with the terminal fallback state', () => {
    const asset = createAsset();

    asset.markUnavailable();

    expectTerminalFallbackState(asset);
    expect(() => asset.markUnavailable()).toThrow('asset already resolved');
  });
});

describe('VerifiedAssetList', () => {
  it('resolves every user asset as a group', async () => {
    const list = createAssetList();
    const urlsByFilename = new Map([
      ['asset-a.wasm', new URL('https://example.com/asset-a.wasm')],
      ['asset-b.wasm', new URL('https://example.com/asset-b.wasm')],
    ]);

    expect(list.resolved).toEqual(false);

    await expect(
      list.resolveUrls({
        resolution: 'user',
        resolveUrlFn: (file) => {
          const url = urlsByFilename.get(file);
          if (url === undefined) {
            throw new Error(`unexpected file: ${file}`);
          }
          return url;
        },
      }),
    ).resolves.toEqual([]);

    expect(list.resolved).toEqual(true);
    expect(list.assets.map((asset) => asset.resolution)).toEqual(['user', 'user']);
    expect(list.assets.map((asset) => asset.url?.toString())).toEqual([
      'https://example.com/asset-a.wasm',
      'https://example.com/asset-b.wasm',
    ]);
  });

  it('rethrows user resolver errors without committing any asset', async () => {
    const list = createAssetList();
    const originalError = new Error('user resolver failed');
    const logger = throwingLogger();

    expect(list.resolved).toEqual(false);

    await expect(
      list.resolveUrls({
        resolution: 'user',
        resolveUrlFn: (file) => {
          if (file === 'asset-b.wasm') {
            throw originalError;
          }
          return new URL(`https://example.com/${file}`);
        },
        logger,
      }),
    ).rejects.toBe(originalError);

    expect(list.resolved).toEqual(false);
    expect(logger.debug).toHaveBeenCalledOnce();
    for (const asset of list.assets) {
      expect(asset.resolved).toEqual(false);
      expect(() => asset.url).toThrow('asset not yet resolved');
    }

    await expect(
      list.resolveUrls({
        resolution: 'user',
        resolveUrlFn: (file) => new URL(`https://example.com/${file}`),
      }),
    ).resolves.toEqual([]);

    expect(list.resolved).toEqual(true);
  });

  it('marks the whole node asset group unavailable when one file is missing', async () => {
    const dir = await mkdtemp(join(tmpdir(), 'fhevm-verified-assets-'));
    try {
      const existingPath = join(dir, 'asset-a.wasm');
      const missingPath = join(dir, 'asset-b.wasm');
      await writeFile(existingPath, new Uint8Array([1, 2, 3]));

      const list = createAssetList();
      const logger = throwingLogger();
      const unavailableAssets = await list.resolveUrls({
        resolution: 'node',
        resolveUrlFn: (file) => pathToFileURL(file.endsWith('asset-a.wasm') ? existingPath : missingPath),
        logger,
      });

      expect(list.resolved).toEqual(true);
      expect(unavailableAssets.map((asset) => asset.filename)).toEqual(['asset-b.wasm']);
      expect(logger.debug).toHaveBeenCalledTimes(3);
      for (const asset of list.assets) {
        expectTerminalFallbackState(asset);
      }
    } finally {
      await rm(dir, { recursive: true, force: true });
    }
  });
});

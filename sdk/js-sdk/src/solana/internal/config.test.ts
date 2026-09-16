import type { SolanaRuntimeConfig } from './config.js';
import { describe, expect, it, vi } from 'vitest';

type ConfigModule = typeof import('./config.js');

async function loadConfigModule(): Promise<ConfigModule> {
  vi.resetModules();
  return import('./config.js');
}

describe('shared logger configuration semantics', () => {
  it.each(['viem', 'ethers'] as const)(
    'keeps %s configuration idempotent after copying logger callbacks',
    async (adapter) => {
      vi.resetModules();
      const { setFhevmRuntimeConfig } =
        adapter === 'viem'
          ? await import('../../viem/internal/config.js')
          : await import('../../ethers/internal/config.js');
      const logger = { debug: vi.fn(), warn: vi.fn(), error: vi.fn() };
      setFhevmRuntimeConfig({ logger });
      expect(() => setFhevmRuntimeConfig({ logger: { ...logger } })).not.toThrow();
      expect(() => setFhevmRuntimeConfig({ logger: { ...logger, warn: vi.fn() } })).toThrow();
    },
  );
});

describe('Solana runtime config', () => {
  it('allows an identical normalized config with the same logger callbacks', async () => {
    const { setFhevmRuntimeConfig } = await loadConfigModule();
    const debug = vi.fn();
    const warn = vi.fn();
    const error = vi.fn();
    const locateFile = (file: string): URL => new URL(file, 'https://example.test/');
    const config: SolanaRuntimeConfig = {
      locateFile,
      wasmAssetLoadMode: 'verified-blob',
      logger: { debug, warn, error },
      singleThread: false,
      numberOfThreads: 4,
      auth: { type: 'ApiKeyHeader', header: 'x-api-key', value: 'secret' },
    };

    setFhevmRuntimeConfig(config);

    expect(() =>
      setFhevmRuntimeConfig({
        ...config,
        logger: { debug, warn, error },
        auth: { type: 'ApiKeyHeader', header: 'x-api-key', value: 'secret' },
      }),
    ).not.toThrow();
  });

  it('rejects a changed logger callback', async () => {
    const { setFhevmRuntimeConfig } = await loadConfigModule();
    const logger = { debug: vi.fn(), warn: vi.fn(), error: vi.fn() };

    setFhevmRuntimeConfig({ logger });

    expect(() => setFhevmRuntimeConfig({ logger: { ...logger, debug: vi.fn() } })).toThrow(
      'FhevmRuntime config has already been set and cannot be changed.',
    );
  });

  it('rejects changed nested configuration', async () => {
    const { setFhevmRuntimeConfig } = await loadConfigModule();

    setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: 'first' } });

    expect(() => setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: 'second' } })).toThrow(
      'FhevmRuntime config has already been set and cannot be changed.',
    );
  });
  it('rejects version overrides rather than ignoring them', async () => {
    const { setFhevmRuntimeConfig } = await loadConfigModule();
    // @ts-expect-error Verify untyped callers are rejected too.
    expect(() => setFhevmRuntimeConfig({ moduleVersions: 'auto' })).toThrow('deployment-pinned');
  });
});

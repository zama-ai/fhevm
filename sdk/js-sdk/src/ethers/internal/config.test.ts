import type { FhevmRuntimeConfig } from '../../core/types/coreFhevmRuntime.js';
import { describe, expect, it, vi } from 'vitest';

type ConfigModule = typeof import('./config.js');

async function loadConfigModule(): Promise<ConfigModule> {
  vi.resetModules();
  return import('./config.js');
}

describe('ethers runtime config', () => {
  it('allows identical config objects', async () => {
    const { setFhevmRuntimeConfig } = await loadConfigModule();
    const config: FhevmRuntimeConfig = { singleThread: true, numberOfThreads: 4 };

    setFhevmRuntimeConfig(config);

    expect(() => setFhevmRuntimeConfig(config)).not.toThrow();
  });

  it('throws when called again with a different config', async () => {
    const { setFhevmRuntimeConfig } = await loadConfigModule();

    setFhevmRuntimeConfig({});

    expect(() => setFhevmRuntimeConfig({ singleThread: true })).toThrow(
      'FhevmRuntime config has already been set and cannot be changed.',
    );
  });
});

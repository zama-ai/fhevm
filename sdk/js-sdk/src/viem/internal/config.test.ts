import type { FhevmRuntimeConfig } from '../../core/types/coreFhevmRuntime.js';
import { describe, expect, it, vi } from 'vitest';

type ConfigModule = typeof import('./config.js');

async function loadConfigModule(): Promise<ConfigModule> {
  vi.resetModules();
  return import('./config.js');
}

describe('viem runtime config', () => {
  it('allows identical module version objects', async () => {
    const { setFhevmRuntimeConfig } = await loadConfigModule();
    const config: FhevmRuntimeConfig = { moduleVersions: { tfhe: '1.6.1', kms: '0.13.20-0' } };

    setFhevmRuntimeConfig(config);

    expect(() => setFhevmRuntimeConfig(config)).not.toThrow();
  });

  it('treats undefined module versions and auto module versions as different configs', async () => {
    const { setFhevmRuntimeConfig } = await loadConfigModule();

    setFhevmRuntimeConfig({});

    expect(() => setFhevmRuntimeConfig({ moduleVersions: 'auto' })).toThrow(
      'FhevmRuntime config has already been set and cannot be changed.',
    );
  });

  it('treats auto module versions and undefined module versions as different configs', async () => {
    const { setFhevmRuntimeConfig } = await loadConfigModule();

    setFhevmRuntimeConfig({ moduleVersions: 'auto' });

    expect(() => setFhevmRuntimeConfig({})).toThrow('FhevmRuntime config has already been set and cannot be changed.');
  });
});

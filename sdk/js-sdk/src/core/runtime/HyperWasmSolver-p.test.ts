import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { ResolvedFhevmOptions } from '../types/coreFhevmClient.js';
import type { FhevmModuleVersions } from '../types/moduleVersions.js';
import { describe, expect, it } from 'vitest';
import { sepolia } from '../chains/definitions/sepolia.js';
import { hyperWasmResolveTfheModuleVersion, hyperWasmResolveTkmsModuleVersion } from './HyperWasmSolver-p.js';

type ResolveParameters = Parameters<typeof hyperWasmResolveTfheModuleVersion>[0];

function makeParameters(parameters: {
  readonly clientModuleVersions?: FhevmModuleVersions | undefined;
  readonly runtimeModuleVersions?: FhevmModuleVersions | undefined;
}): ResolveParameters {
  return {
    chain: sepolia,
    runtime: {
      config: {
        moduleVersions: parameters.runtimeModuleVersions,
      },
    } as unknown as FhevmRuntime,
    options: Object.freeze<ResolvedFhevmOptions>({
      batchRpcCalls: false,
      moduleVersions: parameters.clientModuleVersions,
    }),
  };
}

describe('HyperWasmSolver', () => {
  it('prefers client TFHE module version over runtime fallback', async () => {
    await expect(
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: { tfhe: '1.6.1' },
          runtimeModuleVersions: { tfhe: '1.5.3' },
        }),
      ),
    ).resolves.toBe('1.6.1');
  });

  it('uses runtime TFHE fallback when client module version is missing that field', async () => {
    await expect(
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: { kms: '0.13.20-0' },
          runtimeModuleVersions: { tfhe: '1.5.3' },
        }),
      ),
    ).resolves.toBe('1.5.3');
  });

  it('prefers client TKMS module version over runtime fallback', async () => {
    await expect(
      hyperWasmResolveTkmsModuleVersion(
        makeParameters({
          clientModuleVersions: { kms: '0.13.20-0' },
          runtimeModuleVersions: { kms: '0.13.10' },
        }),
      ),
    ).resolves.toBe('0.13.20-0');
  });

  it('bypasses runtime fallback when client module versions are auto', async () => {
    await expect(
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: 'auto',
          runtimeModuleVersions: { tfhe: '1.5.3' },
        }),
      ),
    ).rejects.toThrow('Cannot auto-resolve TFHE WASM version without a native client.');
  });
});

import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmProtocolContext, ResolvedFhevmOptions } from '../types/coreFhevmClient.js';
import type { HostContractVersion } from '../types/hostContract.js';
import type { FhevmModuleVersions } from '../types/moduleVersions.js';
import type { UintNumber } from '../types/primitives.js';
import { describe, expect, it, vi } from 'vitest';
import { sepolia } from '../chains/definitions/sepolia.js';
import { hyperWasmResolveTfheModuleVersion, hyperWasmResolveTkmsModuleVersion } from './HyperWasmSolver-p.js';
import { protocolContextFromAclVersion } from './ProtocolVersionResolver-p.js';

const KNOWN_EXACT_ACL_WASM_CASES = [
  { acl: [0, 2, 0], tfhe: '1.5.3', kms: '0.13.10' },
  { acl: [0, 3, 0], tfhe: '1.5.3', kms: '0.13.10' },
  { acl: [0, 4, 0], tfhe: '1.6.2', kms: '0.13.20-0' },
  { acl: [0, 5, 0], tfhe: '1.6.2', kms: '0.13.20-0' },
] as const;

type ResolveParameters = Parameters<typeof hyperWasmResolveTfheModuleVersion>[0];

function makeParameters(parameters: {
  readonly clientModuleVersions?: FhevmModuleVersions | undefined;
  readonly runtimeModuleVersions?: FhevmModuleVersions | undefined;
  readonly logger?: FhevmRuntime['config']['logger'] | undefined;
}): ResolveParameters {
  return {
    runtime: {
      config: {
        logger: parameters.logger,
        moduleVersions: parameters.runtimeModuleVersions,
      },
    } as unknown as FhevmRuntime,
    options: Object.freeze<ResolvedFhevmOptions>({
      batchRpcCalls: false,
      moduleVersions: parameters.clientModuleVersions,
    }),
  };
}

function makeProtocolContext(parameters: {
  readonly protocolVersion: FhevmProtocolContext['protocolVersion'];
  readonly pubKeyCrsVersion: FhevmProtocolContext['pubKeyCrsVersion'];
}): FhevmProtocolContext {
  return Object.freeze({
    protocolVersion: parameters.protocolVersion,
    pubKeyCrsVersion: parameters.pubKeyCrsVersion,
  });
}

function makeAclVersion(major: number, minor: number, patch: number): HostContractVersion<'ACL'> {
  return {
    version: `ACL v${major}.${minor}.${patch}`,
    contractName: 'ACL',
    major: major as UintNumber,
    minor: minor as UintNumber,
    patch: patch as UintNumber,
  };
}

function makeChain(relayerUrl: string): typeof sepolia {
  return {
    ...sepolia,
    fhevm: {
      ...sepolia.fhevm,
      relayerUrl,
    },
  };
}

describe('HyperWasmSolver', () => {
  it('prefers client tfhe module version over runtime fallback', () => {
    expect(
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: { tfhe: '1.6.2' },
          runtimeModuleVersions: { tfhe: '1.5.3' },
        }),
      ),
    ).toBe('1.6.2');
  });

  it('uses runtime tfhe fallback when client module version is missing that field', () => {
    expect(
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: { kms: '0.13.20-0' },
          runtimeModuleVersions: { tfhe: '1.5.3' },
        }),
      ),
    ).toBe('1.5.3');
  });

  it('prefers client kms module version over runtime fallback', () => {
    expect(
      hyperWasmResolveTkmsModuleVersion(
        makeParameters({
          clientModuleVersions: { kms: '0.13.20-0' },
          runtimeModuleVersions: { kms: '0.13.10' },
        }),
      ),
    ).toBe('0.13.20-0');
  });

  it('bypasses runtime fallback when client module versions are auto', () => {
    expect(() =>
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: 'auto',
          runtimeModuleVersions: { tfhe: '1.5.3' },
        }),
      ),
    ).toThrow('Cannot auto-resolve tfhe WASM version without a resolved protocol context.');
  });

  it('throws by default when an explicit client tfhe module version is incompatible', () => {
    const protocolContext = makeProtocolContext({
      protocolVersion: { version: '0.12.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.5.4', comparator: 'eq' },
    });

    expect(() =>
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: { tfhe: '1.6.2' },
        }),
        protocolContext,
      ),
    ).toThrow('Explicit tfhe WASM version "1.6.2" is not compatible');
  });

  it('throws by default when an explicit runtime kms module version is incompatible', () => {
    const protocolContext = makeProtocolContext({
      protocolVersion: { version: '0.12.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.5.4', comparator: 'eq' },
    });

    expect(() =>
      hyperWasmResolveTkmsModuleVersion(
        makeParameters({
          runtimeModuleVersions: { kms: '0.13.20-0' },
        }),
        protocolContext,
      ),
    ).toThrow('Explicit kms WASM version "0.13.20-0" is not compatible');
  });

  it('allows compatible non-canonical explicit module versions', () => {
    const protocolContext = makeProtocolContext({
      protocolVersion: { version: '0.13.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.4.0-alpha.3', comparator: 'eq' },
    });

    expect(
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: { tfhe: '1.5.3' },
        }),
        protocolContext,
      ),
    ).toBe('1.5.3');
  });

  it('honors warn and off explicit module-version compatibility modes', () => {
    const protocolContext = makeProtocolContext({
      protocolVersion: { version: '0.12.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.5.4', comparator: 'eq' },
    });
    const warn = vi.fn();
    const logger = {
      debug: vi.fn(),
      warn,
      error: vi.fn(),
    };

    expect(
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: { tfhe: '1.6.2', checkCompatibility: 'warn' },
          logger,
        }),
        protocolContext,
      ),
    ).toBe('1.6.2');
    expect(warn).toHaveBeenCalledWith(expect.stringContaining('Explicit tfhe WASM version "1.6.2"'));

    expect(
      hyperWasmResolveTfheModuleVersion(
        makeParameters({
          clientModuleVersions: { tfhe: '1.6.2', checkCompatibility: 'off' },
          logger,
        }),
        protocolContext,
      ),
    ).toBe('1.6.2');
    expect(warn).toHaveBeenCalledTimes(1);
  });

  it('keeps WASM compatibility rules partitioned for supported protocol contexts', () => {
    const parameters = makeParameters({});

    const unambiguousCases = [
      {
        protocolContext: makeProtocolContext({
          protocolVersion: { version: '0.12.0', comparator: 'eq' },
          pubKeyCrsVersion: { version: '1.5.4', comparator: 'eq' },
        }),
        tfhe: '1.5.3',
        kms: '0.13.10',
      },
      {
        protocolContext: makeProtocolContext({
          protocolVersion: { version: '0.13.0', comparator: 'lt' },
          pubKeyCrsVersion: { version: '1.6.0', comparator: 'lt' },
        }),
        tfhe: '1.5.3',
        kms: '0.13.10',
      },
      {
        protocolContext: makeProtocolContext({
          protocolVersion: { version: '0.13.0', comparator: 'eq' },
          pubKeyCrsVersion: { version: '1.4.0-alpha.3', comparator: 'eq' },
        }),
        tfhe: '1.6.2',
        kms: '0.13.20-0',
      },
      {
        protocolContext: makeProtocolContext({
          protocolVersion: { version: '0.13.0', comparator: 'eq' },
          pubKeyCrsVersion: { version: '1.6.1', comparator: 'eq' },
        }),
        tfhe: '1.6.2',
        kms: '0.13.20-0',
      },
      {
        protocolContext: makeProtocolContext({
          protocolVersion: { version: '0.13.0', comparator: 'gt' },
          pubKeyCrsVersion: { version: '1.6.1', comparator: 'gt' },
        }),
        tfhe: '1.6.2',
        kms: '0.13.20-0',
      },
    ] as const satisfies ReadonlyArray<{
      readonly protocolContext: FhevmProtocolContext;
      readonly tfhe: ReturnType<typeof hyperWasmResolveTfheModuleVersion>;
      readonly kms: ReturnType<typeof hyperWasmResolveTkmsModuleVersion>;
    }>;

    for (const { protocolContext, tfhe, kms } of unambiguousCases) {
      expect(hyperWasmResolveTfheModuleVersion(parameters, protocolContext)).toBe(tfhe);
      expect(hyperWasmResolveTkmsModuleVersion(parameters, protocolContext)).toBe(kms);
    }

    const ambiguousCases = [
      makeProtocolContext({
        protocolVersion: { version: '0.12.0', comparator: 'gt' },
        pubKeyCrsVersion: { version: '1.6.1', comparator: 'eq' },
      }),
      makeProtocolContext({
        protocolVersion: { version: '0.14.0', comparator: 'lt' },
        pubKeyCrsVersion: { version: '1.6.1', comparator: 'eq' },
      }),
      makeProtocolContext({
        protocolVersion: { version: '0.13.0', comparator: 'eq' },
        pubKeyCrsVersion: { version: '1.6.1', comparator: 'lt' },
      }),
    ] as const satisfies readonly FhevmProtocolContext[];

    for (const protocolContext of ambiguousCases) {
      expect(() => hyperWasmResolveTfheModuleVersion(parameters, protocolContext)).toThrow(
        'Cannot auto-resolve tfhe WASM version from protocol context',
      );
      expect(() => hyperWasmResolveTkmsModuleVersion(parameters, protocolContext)).toThrow(
        'Cannot auto-resolve kms WASM version from protocol context',
      );
    }
  });

  it('auto-resolves WASM versions for every exact protocol context emitted by the protocol resolver', () => {
    const parameters = makeParameters({});
    const localstackLikeChain = makeChain('http://localhost:3000');

    for (const { acl, tfhe, kms } of KNOWN_EXACT_ACL_WASM_CASES) {
      const protocolContext = protocolContextFromAclVersion(
        localstackLikeChain,
        makeAclVersion(acl[0], acl[1], acl[2]),
      );

      expect(protocolContext.protocolVersion.comparator).toBe('eq');
      expect(protocolContext.pubKeyCrsVersion.comparator).toBe('eq');
      expect(hyperWasmResolveTfheModuleVersion(parameters, protocolContext)).toBe(tfhe);
      expect(hyperWasmResolveTkmsModuleVersion(parameters, protocolContext)).toBe(kms);
    }
  });

  it('maps protocol versions before 0.13.0 to legacy WASM versions', () => {
    const parameters = makeParameters({});
    const protocolContext = makeProtocolContext({
      protocolVersion: { version: '0.12.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.5.4', comparator: 'eq' },
    });

    expect(hyperWasmResolveTfheModuleVersion(parameters, protocolContext)).toBe('1.5.3');
    expect(hyperWasmResolveTkmsModuleVersion(parameters, protocolContext)).toBe('0.13.10');
  });

  it('maps protocol versions from 0.13.0 with current PubKey/CRS to current WASM versions', () => {
    const parameters = makeParameters({});
    const protocolContext = makeProtocolContext({
      protocolVersion: { version: '0.13.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.6.1', comparator: 'eq' },
    });

    expect(hyperWasmResolveTfheModuleVersion(parameters, protocolContext)).toBe('1.6.2');
    expect(hyperWasmResolveTkmsModuleVersion(parameters, protocolContext)).toBe('0.13.20-0');
  });

  it('maps protocol versions from 0.13.0 with legacy PubKey/CRS to current canonical WASM versions', () => {
    const parameters = makeParameters({});
    const protocolContext = makeProtocolContext({
      protocolVersion: { version: '0.13.0', comparator: 'eq' },
      pubKeyCrsVersion: { version: '1.4.0-alpha.3', comparator: 'eq' },
    });

    expect(hyperWasmResolveTfheModuleVersion(parameters, protocolContext)).toBe('1.6.2');
    expect(hyperWasmResolveTkmsModuleVersion(parameters, protocolContext)).toBe('0.13.20-0');
  });

  it('uses bounded protocol-context comparators when they are enough to select WASM versions', () => {
    const parameters = makeParameters({});

    const newerContext = makeProtocolContext({
      protocolVersion: { version: '0.14.0', comparator: 'gt' },
      pubKeyCrsVersion: { version: '1.6.1', comparator: 'gt' },
    });
    expect(hyperWasmResolveTfheModuleVersion(parameters, newerContext)).toBe('1.6.2');
    expect(hyperWasmResolveTkmsModuleVersion(parameters, newerContext)).toBe('0.13.20-0');

    const olderContext = makeProtocolContext({
      protocolVersion: { version: '0.11.0', comparator: 'lt' },
      pubKeyCrsVersion: { version: '1.6.0', comparator: 'lt' },
    });
    expect(hyperWasmResolveTfheModuleVersion(parameters, olderContext)).toBe('1.5.3');
    expect(hyperWasmResolveTkmsModuleVersion(parameters, olderContext)).toBe('0.13.10');

    const beforeSwitchContext = makeProtocolContext({
      protocolVersion: { version: '0.13.0', comparator: 'lt' },
      pubKeyCrsVersion: { version: '1.6.0', comparator: 'lt' },
    });
    expect(hyperWasmResolveTfheModuleVersion(parameters, beforeSwitchContext)).toBe('1.5.3');
    expect(hyperWasmResolveTkmsModuleVersion(parameters, beforeSwitchContext)).toBe('0.13.10');
  });

  it('throws when bounded protocol context comparators leave WASM auto-resolution ambiguous', () => {
    const parameters = makeParameters({});

    expect(() =>
      hyperWasmResolveTfheModuleVersion(
        parameters,
        makeProtocolContext({
          protocolVersion: { version: '0.12.0', comparator: 'gt' },
          pubKeyCrsVersion: { version: '1.6.1', comparator: 'eq' },
        }),
      ),
    ).toThrow('Cannot auto-resolve tfhe WASM version from protocol context');
    expect(() =>
      hyperWasmResolveTkmsModuleVersion(
        parameters,
        makeProtocolContext({
          protocolVersion: { version: '0.14.0', comparator: 'lt' },
          pubKeyCrsVersion: { version: '1.6.1', comparator: 'eq' },
        }),
      ),
    ).toThrow('Cannot auto-resolve kms WASM version from protocol context');
    expect(() =>
      hyperWasmResolveTfheModuleVersion(
        parameters,
        makeProtocolContext({
          protocolVersion: { version: '0.13.0', comparator: 'eq' },
          pubKeyCrsVersion: { version: '1.6.1', comparator: 'lt' },
        }),
      ),
    ).toThrow('Cannot auto-resolve tfhe WASM version from protocol context protocol=eq:0.13.0, pubKeyCrs=lt:1.6.1.');
  });
});

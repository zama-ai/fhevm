import type { ModuleVersionCompatibilityCheck, TfheVersion, TkmsVersion } from '../types/moduleVersions.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmProtocolContext, ResolvedFhevmOptions, VersionResolution } from '../types/coreFhevmClient.js';
import type { SemverRange } from '../base/semver.js';
import type { Logger } from '../types/logger.js';
import { semverComparatorImpliesRange } from '../base/semver.js';

////////////////////////////////////////////////////////////////////////////////

type ResolveParameters = {
  readonly runtime: FhevmRuntime;
  readonly options: ResolvedFhevmOptions;
};

type WasmModuleVersionByKey = {
  readonly tfhe: TfheVersion;
  readonly kms: TkmsVersion;
};

type WasmModuleKey = keyof WasmModuleVersionByKey;

type WasmModuleVersion<K extends WasmModuleKey> = WasmModuleVersionByKey[K];

type WasmCompatibilityRule = {
  readonly protocol: SemverRange;
  readonly pubKeyCrs: SemverRange;
  readonly tfhe: {
    readonly canonical: TfheVersion;
    readonly compatible: readonly [TfheVersion, ...TfheVersion[]];
  };
  readonly kms: {
    readonly canonical: TkmsVersion;
    readonly compatible: readonly [TkmsVersion, ...TkmsVersion[]];
  };
};

type HyperWasmSolverConfig = {
  readonly compatibilityRules: readonly WasmCompatibilityRule[];
};

type ResolveWasmCompatibilityRuleOptions<ExplicitVersion extends string = TfheVersion | TkmsVersion> =
  | {
      readonly purpose: 'auto';
    }
  | {
      readonly purpose: 'explicit';
      readonly explicitVersion: ExplicitVersion;
      readonly checkCompatibility: ModuleVersionCompatibilityCheck | undefined;
    };

type ExplicitResolveWasmCompatibilityRuleOptions<ExplicitVersion extends string = TfheVersion | TkmsVersion> = Extract<
  ResolveWasmCompatibilityRuleOptions<ExplicitVersion>,
  { readonly purpose: 'explicit' }
>;

const HYPER_WASM_SOLVER_CONFIG = {
  /**
   * Protocol-context to WASM-version compatibility mapping.
   *
   * Warning: auto-resolution requires exactly one rule to be implied by the
   * resolved protocol and PubKey/CRS-version comparators. The table does not
   * need to be sorted, but its ranges must be mutually exclusive for every
   * supported protocol context; overlapping implied ranges and missing ranges
   * both make auto-resolution ambiguous and throw.
   *
   * `canonical` is the version selected by auto-resolution. `compatible` is the
   * full allowlist for explicit module-version compatibility checks.
   */
  compatibilityRules: [
    {
      // protocol.version <= 0.12.x
      protocol: { version: '0.13.0', comparator: 'lt' },
      // pubKeyCrs.version <= 1.5.x (ex: mainnet 1.4.0-alpha.3)
      pubKeyCrs: { version: '1.6.0', comparator: 'lt' },
      tfhe: {
        canonical: '1.5.3',
        compatible: ['1.5.3'],
      },
      kms: {
        canonical: '0.13.10',
        compatible: ['0.13.10'],
      },
    },
    {
      // protocol.version >= 0.13.0
      protocol: { version: '0.13.0', comparator: 'ge' },
      // pubKeyCrs.version <= 1.5.x (ex: mainnet 1.4.0-alpha.3)
      pubKeyCrs: { version: '1.6.0', comparator: 'lt' },
      tfhe: {
        canonical: '1.6.1',
        compatible: ['1.5.3', '1.6.1'],
      },
      kms: {
        canonical: '0.13.20-0',
        compatible: ['0.13.10', '0.13.20-0'],
      },
    },
    {
      // protocol.version >= 0.13.0
      protocol: { version: '0.13.0', comparator: 'ge' },
      // pubKeyCrs.version >= 1.6.0 (ex: localstack_v13)
      pubKeyCrs: { version: '1.6.0', comparator: 'ge' },
      tfhe: {
        canonical: '1.6.1',
        compatible: ['1.6.1'],
      },
      kms: {
        canonical: '0.13.20-0',
        compatible: ['0.13.10', '0.13.20-0'],
      },
    },
  ],
} as const satisfies HyperWasmSolverConfig;

const WASM_COMPATIBILITY_RULES = HYPER_WASM_SOLVER_CONFIG.compatibilityRules;

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the tfhe WASM module version to load for an encryption-capable client.
 *
 * Resolution order:
 *
 * 1. Client option `moduleVersions: 'auto'` forces protocol-based auto-resolution
 *    and deliberately ignores the runtime fallback.
 * 2. Client option `moduleVersions.tfhe` wins over every fallback, then is
 *    checked against the protocol-context compatibility allowlist.
 * 3. Runtime config `moduleVersions.tfhe` is used as a global fallback, then is
 *    checked against the protocol-context compatibility allowlist.
 * 4. Otherwise, the SDK auto-resolves from the resolved protocol version.
 *
 * `checkCompatibility` only applies to explicit `tfhe` overrides. It does not
 * change auto-resolution.
 *
 * Examples:
 *
 * - Client override:
 *   `{ options.moduleVersions: { tfhe: '1.6.1' } }` -> `'1.6.1'`.
 * - Runtime fallback:
 *   `{ options.moduleVersions: undefined, runtime.config.moduleVersions: { tfhe: '1.5.3' } }` -> `'1.5.3'`.
 * - Client auto bypassing runtime fallback:
 *   `{ options.moduleVersions: 'auto', runtime.config.moduleVersions: { tfhe: '1.5.3' } }`
 *   resolves from `protocolContext`, not from `'1.5.3'`.
 * - Protocol-context auto-resolution:
 *   `protocolVersion = eq:0.12.0, pubKeyCrsVersion = eq:1.5.4` -> `'1.5.3'`.
 *   `protocolVersion = eq:0.13.0, pubKeyCrsVersion = eq:1.4.0-alpha.3` -> `'1.6.1'`.
 *   `protocolVersion = eq:0.13.0, pubKeyCrsVersion = eq:1.6.1` -> `'1.6.1'`.
 */
export function hyperWasmResolveTfheModuleVersion(
  parameters: ResolveParameters,
  protocolContext?: FhevmProtocolContext,
): TfheVersion {
  const resolveRuleOptions = _resolveWasmCompatibilityRuleOptions(parameters, 'tfhe');

  const rule = _resolveWasmCompatibilityRule(protocolContext, 'tfhe', resolveRuleOptions);

  if (resolveRuleOptions.purpose === 'explicit') {
    _handleModuleVersionCompatibilityIssue(
      'tfhe',
      parameters.runtime.config.logger,
      protocolContext,
      rule,
      resolveRuleOptions,
    );
    return resolveRuleOptions.explicitVersion;
  }

  if (rule === undefined) {
    throw new Error('Cannot auto-resolve tfhe WASM version without a resolved compatibility rule.');
  }

  return rule.tfhe.canonical;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the tkms WASM module version to load for a decryption-capable client.
 *
 * Resolution order:
 *
 * 1. Client option `moduleVersions: 'auto'` forces protocol-based auto-resolution
 *    and deliberately ignores the runtime fallback.
 * 2. Client option `moduleVersions.kms` wins over every fallback, then is
 *    checked against the protocol-context compatibility allowlist.
 * 3. Runtime config `moduleVersions.kms` is used as a global fallback, then is
 *    checked against the protocol-context compatibility allowlist.
 * 4. Otherwise, the SDK auto-resolves from the resolved protocol version.
 *
 * `checkCompatibility` only applies to explicit `kms` overrides. It does not
 * change auto-resolution.
 *
 * Examples:
 *
 * - Client override:
 *   `{ options.moduleVersions: { kms: '0.13.20-0' } }` -> `'0.13.20-0'`.
 * - Runtime fallback:
 *   `{ options.moduleVersions: undefined, runtime.config.moduleVersions: { kms: '0.13.10' } }` -> `'0.13.10'`.
 * - Client auto bypassing runtime fallback:
 *   `{ options.moduleVersions: 'auto', runtime.config.moduleVersions: { kms: '0.13.10' } }`
 *   resolves from `protocolContext`, not from `'0.13.10'`.
 * - Protocol-context auto-resolution:
 *   `protocolVersion = eq:0.12.0, pubKeyCrsVersion = eq:1.5.4` -> `'0.13.10'`.
 *   `protocolVersion = eq:0.13.0, pubKeyCrsVersion = eq:1.4.0-alpha.3` -> `'0.13.20-0'`.
 *   `protocolVersion = eq:0.13.0, pubKeyCrsVersion = eq:1.6.1` -> `'0.13.20-0'`.
 */
export function hyperWasmResolveTkmsModuleVersion(
  parameters: ResolveParameters,
  protocolContext?: FhevmProtocolContext,
): TkmsVersion {
  const resolveRuleOptions = _resolveWasmCompatibilityRuleOptions(parameters, 'kms');

  const rule = _resolveWasmCompatibilityRule(protocolContext, 'kms', resolveRuleOptions);

  if (resolveRuleOptions.purpose === 'explicit') {
    _handleModuleVersionCompatibilityIssue(
      'kms',
      parameters.runtime.config.logger,
      protocolContext,
      rule,
      resolveRuleOptions,
    );
    return resolveRuleOptions.explicitVersion;
  }

  if (rule === undefined) {
    throw new Error('Cannot auto-resolve kms WASM version without a resolved compatibility rule.');
  }

  return rule.kms.canonical;
}

////////////////////////////////////////////////////////////////////////////////
//
// Helpers
//
////////////////////////////////////////////////////////////////////////////////

function _resolveWasmCompatibilityRuleOptions<K extends WasmModuleKey>(
  parameters: ResolveParameters,
  moduleKey: K,
): ResolveWasmCompatibilityRuleOptions<WasmModuleVersion<K>> {
  const clientVersions = parameters.options.moduleVersions;
  if (clientVersions !== 'auto') {
    if (clientVersions?.[moduleKey] !== undefined) {
      return {
        purpose: 'explicit',
        explicitVersion: clientVersions[moduleKey] as WasmModuleVersion<K>,
        checkCompatibility: clientVersions.checkCompatibility,
      };
    }

    const runtimeVersions = parameters.runtime.config.moduleVersions;
    if (runtimeVersions !== 'auto' && runtimeVersions?.[moduleKey] !== undefined) {
      return {
        purpose: 'explicit',
        explicitVersion: runtimeVersions[moduleKey] as WasmModuleVersion<K>,
        checkCompatibility: runtimeVersions.checkCompatibility,
      };
    }
  }

  return { purpose: 'auto' };
}

function _resolveWasmCompatibilityRule(
  protocolContext: FhevmProtocolContext | undefined,
  moduleKey: WasmModuleKey,
  options: ResolveWasmCompatibilityRuleOptions,
): WasmCompatibilityRule | undefined {
  if (options.purpose === 'explicit' && _resolveModuleVersionCompatibilityCheck(options.checkCompatibility) === 'off') {
    return undefined;
  }
  if (protocolContext === undefined) {
    if (options.purpose === 'auto') {
      throw new Error(`Cannot auto-resolve ${moduleKey} WASM version without a resolved protocol context.`);
    }
    return undefined;
  }

  const rules = _findWasmCompatibilityRules(protocolContext);
  const rule = rules[0];
  if (rules.length === 1 && rule !== undefined) {
    return rule;
  }

  const message = _ambiguousWasmCompatibilityRuleMessage(protocolContext, moduleKey, options.purpose);
  if (options.purpose === 'auto') {
    throw new Error(message);
  }

  return undefined;
}

function _findWasmCompatibilityRules(protocolContext: FhevmProtocolContext): WasmCompatibilityRule[] {
  return WASM_COMPATIBILITY_RULES.filter(
    (entry) =>
      _versionResolutionImpliesRange(protocolContext.protocolVersion, entry.protocol) &&
      _versionResolutionImpliesRange(protocolContext.pubKeyCrsVersion, entry.pubKeyCrs),
  );
}

function _versionResolutionImpliesRange(versionResolution: VersionResolution<string>, range: SemverRange): boolean {
  return semverComparatorImpliesRange(versionResolution.version, versionResolution.comparator, range);
}

function _resolveModuleVersionCompatibilityCheck(
  checkCompatibility: ModuleVersionCompatibilityCheck | undefined,
): ModuleVersionCompatibilityCheck {
  return checkCompatibility ?? 'throw';
}

function _handleModuleVersionCompatibilityIssue(
  moduleKey: WasmModuleKey,
  logger: FhevmRuntime['config']['logger'],
  protocolContext: FhevmProtocolContext | undefined,
  rule: WasmCompatibilityRule | undefined,
  options: ExplicitResolveWasmCompatibilityRuleOptions,
): void {
  if (_resolveModuleVersionCompatibilityCheck(options.checkCompatibility) === 'off' || protocolContext === undefined) {
    return;
  }

  if (rule === undefined) {
    _handleModuleVersionCompatibilityPolicy(
      options.checkCompatibility,
      logger,
      _ambiguousWasmCompatibilityRuleMessage(protocolContext, moduleKey, options.purpose),
    );
    return;
  }

  const compatibleVersions = _getCompatibleModuleVersions(moduleKey, rule);
  if (!compatibleVersions.includes(options.explicitVersion)) {
    _handleModuleVersionCompatibilityPolicy(
      options.checkCompatibility,
      logger,
      _incompatibleModuleVersionMessage(moduleKey, options.explicitVersion, compatibleVersions, protocolContext),
    );
  }
}

function _handleModuleVersionCompatibilityPolicy(
  checkCompatibility: ModuleVersionCompatibilityCheck | undefined,
  logger: Logger | undefined,
  message: string,
): void {
  const resolvedCheckCompatibility = _resolveModuleVersionCompatibilityCheck(checkCompatibility);
  switch (resolvedCheckCompatibility) {
    case 'throw':
      throw new Error(message);
    case 'warn':
      if (logger?.warn !== undefined) {
        logger.warn(message);
      } else {
        console.warn(message);
      }
      return;
    case 'off':
      return;
    default: {
      const exhaustiveCheck: never = resolvedCheckCompatibility;
      throw new Error(`Unsupported module version compatibility check "${exhaustiveCheck}".`);
    }
  }
}

function _getCompatibleModuleVersions(moduleKey: WasmModuleKey, rule: WasmCompatibilityRule): readonly string[] {
  switch (moduleKey) {
    case 'tfhe':
      return rule.tfhe.compatible;
    case 'kms':
      return rule.kms.compatible;
    default: {
      const exhaustiveCheck: never = moduleKey;
      throw new Error(`Unsupported WASM module "${exhaustiveCheck}".`);
    }
  }
}

function _incompatibleModuleVersionMessage(
  moduleKey: WasmModuleKey,
  version: string,
  compatibleVersions: readonly string[],
  protocolContext: FhevmProtocolContext | undefined,
): string {
  const context =
    protocolContext === undefined
      ? 'without a resolved protocol context'
      : `with protocol context protocol=${_formatVersionResolution(protocolContext.protocolVersion)}, ` +
        `pubKeyCrs=${_formatVersionResolution(protocolContext.pubKeyCrsVersion)}`;

  return (
    `Explicit ${moduleKey} WASM version "${version}" is not compatible ${context}. ` +
    `Compatible versions: ${compatibleVersions.join(', ')}.`
  );
}

function _ambiguousWasmCompatibilityRuleMessage(
  protocolContext: FhevmProtocolContext,
  moduleKey: WasmModuleKey,
  purpose: 'auto' | 'explicit',
): string {
  const action =
    purpose === 'auto' ? `auto-resolve ${moduleKey} WASM version` : `check ${moduleKey} WASM version compatibility`;

  return (
    `Cannot ${action} from protocol context ` +
    `protocol=${_formatVersionResolution(protocolContext.protocolVersion)}, ` +
    `pubKeyCrs=${_formatVersionResolution(protocolContext.pubKeyCrsVersion)}. ` +
    `The resolved context does not prove exactly one WASM compatibility rule.`
  );
}

function _formatVersionResolution(versionResolution: VersionResolution<string>): string {
  return `${versionResolution.comparator}:${versionResolution.version}`;
}

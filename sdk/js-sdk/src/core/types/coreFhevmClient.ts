import type { FhevmChain } from './fhevmChain.js';
import type { FhevmRuntime } from './coreFhevmRuntime.js';
import type { FheEncryptionKeyBytes } from './fheEncryptionKey.js';
import type {
  FhevmDecryptModuleVersions,
  FhevmEncryptModuleVersions,
  FhevmModuleVersions,
  TfheVersion,
  TkmsVersion,
} from './moduleVersions.js';
import type { TrustedClient } from '../modules/ethereum/types.js';

////////////////////////////////////////////////////////////////////////////////

export type FhevmBaseOptions = {
  readonly batchRpcCalls?: boolean | undefined;
};

export type FhevmEncryptOptions = FhevmBaseOptions & {
  readonly fheEncryptionKey?: FheEncryptionKeyBytes | undefined;
  readonly moduleVersions?: FhevmEncryptModuleVersions | undefined;
};

export type FhevmDecryptOptions = FhevmBaseOptions & {
  readonly moduleVersions?: FhevmDecryptModuleVersions | undefined;
};

export type FhevmOptions = FhevmBaseOptions & {
  readonly fheEncryptionKey?: FheEncryptionKeyBytes | undefined;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
};

/**
 * An Ethereum chain definition (one that defines `kmsGeneration`) and a client
 * connected to it. Used only to read `KMSGeneration` when the host chain is
 * not Ethereum; every other read goes through the host-chain client.
 */
export type FhevmKeyVerification<client extends NativeClient = NativeClient> = {
  readonly chain: FhevmChain;
  readonly client: client;
};

export type ResolvedFhevmKeyVerification = {
  readonly chain: FhevmChain;
  readonly trustedClient: TrustedClient;
};

export type ResolvedFhevmOptions = {
  readonly batchRpcCalls: boolean;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
  /**
   * Key / CRS bytes provided by the caller. Used in place of a download, and
   * cached only once they pass the same check as downloaded bytes.
   */
  readonly fheEncryptionKey: FheEncryptionKeyBytes | undefined;
  readonly keyVerification: ResolvedFhevmKeyVerification | undefined;
};

////////////////////////////////////////////////////////////////////////////////

export type NativeClient = NonNullable<object>;
export type OptionalNativeClient = NativeClient | undefined;
export type OptionalFhevmChain = FhevmChain | undefined;

/**
 * Semver-compatible FHEVM protocol version.
 *
 * The protocol version describes the on-chain FHEVM protocol used by a given
 * chain.
 *
 * Example: `'0.14.0'`.
 */
export type ProtocolVersion = string;
export type PubKeyCrsVersion = string;

/**
 * Comparator between an actual resolved version and the version known by this
 * SDK.
 *
 * - `'eq'`: actual version equals the returned version.
 * - `'gt'`: actual version is strictly greater than the returned version.
 * - `'lt'`: actual version is strictly lower than the returned version.
 */
export type VersionResolutionComparator = 'eq' | 'lt' | 'gt';

/**
 * Result of resolving a version that may be exact or bounded by this SDK's
 * compatibility table.
 *
 * This structure is deliberately "honest": when the SDK cannot identify the
 * exact version, it returns a bounded relationship instead of
 * pretending that the version string is exact. Consumers that only need feature
 * gating should inspect both `version` and `comparator`.
 */
export type VersionResolution<version extends string> = {
  /**
   * The version known by this SDK.
   *
   * Interpret this value through {@link VersionResolution.comparator}; it may
   * be exact, a lower bound, or an upper bound.
   */
  readonly version: version;
  /**
   * Describes how the actual resolved version compares to
   * {@link VersionResolution.version}.
   */
  readonly comparator: VersionResolutionComparator;
};

export type ProtocolVersionResolution = VersionResolution<ProtocolVersion>;
export type PubKeyCrsVersionResolution = VersionResolution<PubKeyCrsVersion>;

export type FhevmProtocolContext = {
  readonly protocolVersion: ProtocolVersionResolution;
  readonly pubKeyCrsVersion: PubKeyCrsVersionResolution;
};

export type WithProtocolVersion = {
  readonly protocolVersion: ProtocolVersionResolution;
};

export type WithTfheVersion = WithProtocolVersion & {
  readonly tfheVersion: TfheVersion;
};

export type WithTkmsVersion = WithProtocolVersion & {
  readonly tkmsVersion: TkmsVersion;
};

export type FhevmExtension<
  actions extends Record<string, unknown> = Record<string, unknown>,
  runtime extends FhevmRuntime = FhevmRuntime,
> = {
  readonly actions: actions;
  readonly runtime: runtime;
  readonly init?: ((client: FhevmBase<FhevmChain>) => Promise<void>) | undefined;
};

export interface FhevmBase<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
> {
  readonly uid: string;
  readonly chain: chain; // undefined when no chain
  readonly runtime: runtime;
  readonly client: client; // undefined when no host
  readonly options: ResolvedFhevmOptions;
}

export interface Fhevm<
  chain extends FhevmChain | undefined = FhevmChain | undefined,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends OptionalNativeClient = NativeClient,
> extends FhevmBase<chain, runtime, client> {
  readonly protocolVersion: ProtocolVersionResolution;
  readonly extend: <const actions extends Record<string, unknown>, extendedRuntime extends FhevmRuntime>(
    actionsFactory: (client: FhevmBase<chain, FhevmRuntime, client>) => FhevmExtension<actions, extendedRuntime>,
  ) => this & actions & { readonly runtime: extendedRuntime };
  readonly init: () => Promise<void>;
  readonly ready: Promise<void>;
}

import type { ChecksummedAddress } from '../types/primitives.js';
import type { HostContractName, HostContractVersion, HostContractVersionString } from '../types/hostContract.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { assertIsNonEmptyString } from '../base/string.js';
import { getVersionAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { assertIsUintNumber } from '../base/uint.js';
import { CACHE_TTL_24H, createCachedFetch } from '../base/cachedFetch.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

type ReturnType = HostContractVersion;
type InvalidateVersionCacheOptions = {
  readonly includeInflight?: boolean | undefined;
};
type InvalidateVersionCacheParameters = Parameters & InvalidateVersionCacheOptions;

////////////////////////////////////////////////////////////////////////////////

const cachedGetVersion = createCachedFetch<Context, Parameters, ReturnType>({
  executeFn: _getVersion,
  cacheKeyFn: _getVersionCacheKey,
  // Host contract versions are immutable per deployment, so a long TTL is safe.
  ttlMs: CACHE_TTL_24H,
});

/**
 * Builds the cache key for a host contract version lookup.
 *
 * The runtime UID scopes the entry to one SDK runtime instance, while the
 * address scopes it to one deployed host contract within that runtime.
 *
 * Host-contract addresses are treated as globally unique deployment
 * identifiers across supported chains. If that invariant changes, include the
 * chain id in this key.
 */
function _getVersionCacheKey(context: Context, parameters: Parameters): string {
  return `${context.runtime.uid.toLowerCase()}:${parameters.address.toLowerCase()}`;
}

/**
 * Reads the version of a host contract at the given address.
 *
 * Results are cached per address to avoid redundant RPC calls — concurrent
 * callers share the same in-flight request (deduplication), and subsequent
 * calls return the cached value instantly.
 *
 * Although host contract versions are generally immutable per deployment,
 * upgrades can occur in rare cases (e.g. proxy re-pointing). To account for
 * this, cached entries expire after 24 hours and are automatically refetched.
 *
 * @param parameters.address - The checksummed address of the host contract.
 * @param parameters.forceRefresh - If `true`, invalidates the cached entry and
 *   makes a fresh RPC call. The new result is stored back in the cache.
 *   Use this after a known contract upgrade to ensure all subsequent callers
 *   see the updated version.
 */
export function getHostContractVersion(
  context: Context,
  parameters: Parameters & { readonly forceRefresh?: boolean },
): Promise<ReturnType> {
  return cachedGetVersion.execute(context, parameters);
}

/**
 * Invalidates cached host contract versions.
 *
 * When called with `context` and `address`, only that runtime/address entry is
 * invalidated. When called without arguments, all settled host-version cache
 * entries in the current JS realm are invalidated.
 *
 * In-flight entries are kept by default. Pass `includeInflight: true` to also
 * discard in-flight fetches; existing callers still receive their promises, but
 * those results will no longer be stored in the cache.
 */
export function invalidateVersionCache(options?: InvalidateVersionCacheOptions): void;
export function invalidateVersionCache(context: Context, parameters: InvalidateVersionCacheParameters): void;
export function invalidateVersionCache(
  contextOrOptions?: Context | InvalidateVersionCacheOptions,
  parameters?: InvalidateVersionCacheParameters,
): void {
  if (parameters === undefined) {
    const options = contextOrOptions as InvalidateVersionCacheOptions | undefined;
    cachedGetVersion.clear(_makeClearOptions(options));
    return;
  }

  cachedGetVersion.clear({
    key: _getVersionCacheKey(contextOrOptions as Context, parameters),
    ..._makeClearOptions(parameters),
  });
}

function _makeClearOptions(
  options: InvalidateVersionCacheOptions | undefined,
): { readonly includeInflight?: boolean } | undefined {
  if (options?.includeInflight === undefined) {
    return undefined;
  }
  return { includeInflight: options.includeInflight };
}

async function _getVersion(context: Context, parameters: Parameters): Promise<HostContractVersion> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getVersionAbi,
    args: [],
    functionName: getVersionAbi[0].name,
  });

  try {
    assertIsNonEmptyString(res);
  } catch (e) {
    throw new Error(`Invalid Version.`, {
      cause: e,
    });
  }

  return parseVersion(res);
}

/**
 * Parses a version string with format: `<contractName> v<major>.<minor>.<patch>`
 */
function parseVersion(version: string): HostContractVersion {
  const err = `Invalid version format: "${version}". Expected "<contractName> v<major>.<minor>.<patch>".`;

  // Split on " v" to separate contract name from semver
  const vIndex = version.lastIndexOf(' v');
  if (vIndex < 1) {
    throw new Error(err);
  }

  const contractName = version.slice(0, vIndex);
  const semver = version.slice(vIndex + 2);

  const parts = semver.split('.');
  if (parts.length !== 3) {
    throw new Error(err);
  }

  const [majorStr, minorStr, patchStr] = parts;
  const major = Number(majorStr);
  const minor = Number(minorStr);
  const patch = Number(patchStr);

  assertIsUintNumber(major, {});
  assertIsUintNumber(minor, {});
  assertIsUintNumber(patch, {});

  if (
    contractName !== 'ACL' &&
    contractName !== 'FHEVMExecutor' &&
    contractName !== 'InputVerifier' &&
    contractName !== 'KMSVerifier' &&
    contractName !== 'HCULimit' &&
    contractName !== 'ProtocolConfig'
  ) {
    throw new Error(err);
  }

  return Object.freeze({
    version: version as HostContractVersionString,
    contractName,
    major,
    minor,
    patch,
  });
}

/**
 * Returns `true` if the version is strictly before `major.minor` (patch ignored).
 */
export function isVersionStrictlyBefore(
  version: HostContractVersion,
  before: { readonly major: number; readonly minor: number },
): boolean {
  if (version.major < before.major) {
    return true;
  }
  if (version.major === before.major) {
    return version.minor < before.minor;
  }
  return false;
}

/**
 * Asserts that a {@link HostContractVersion} belongs to a specific contract.
 * Narrows the type to `HostContractVersion<name>` on success.
 *
 * @throws If `v.contractName` does not match the expected name.
 */
export function assertIsHostContractVersionOf<hostContractName extends HostContractName>(
  v: HostContractVersion,
  hostContractName: hostContractName,
): asserts v is HostContractVersion<hostContractName> {
  if (v.contractName !== hostContractName) {
    throw new Error(`Invalid contract name. Expecting '${hostContractName}', got ${v.contractName}.`);
  }
}

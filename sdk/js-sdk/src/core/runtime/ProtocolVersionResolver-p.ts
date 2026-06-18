import type {
  FhevmBase,
  FhevmProtocolContext,
  ProtocolVersion,
  ProtocolVersionResolution,
  PubKeyCrsVersion,
  PubKeyCrsVersionResolution,
} from '../types/coreFhevmClient.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { HostContractVersion } from '../types/hostContract.js';
import type { SemverInterval, SemverIntervalLowerBound, SemverIntervalUpperBound } from '../base/semver.js';
import { asAddress, addressToChecksummedAddress } from '../base/address.js';
import { compareSemver, isSemverInInterval, semverComparatorImpliesRange } from '../base/semver.js';
import { mainnet } from '../chains/definitions/mainnet.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { assertIsHostContractVersionOf, getHostContractVersion } from '../host-contracts/HostContractVersion-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ResolveProtocolVersionParameters = FhevmBase<FhevmChain>;
export type ResolveProtocolContextParameters = ResolveProtocolVersionParameters;

export async function resolveProtocolContext(
  parameters: ResolveProtocolContextParameters,
): Promise<FhevmProtocolContext> {
  const aclAddress = addressToChecksummedAddress(asAddress(parameters.chain.fhevm.contracts.acl.address));
  const aclVersion = await getHostContractVersion(parameters, { address: aclAddress });

  assertIsHostContractVersionOf(aclVersion, 'ACL');

  return protocolContextFromAclVersion(parameters.chain, aclVersion);
}

export async function resolveProtocolVersion(
  parameters: ResolveProtocolVersionParameters,
): Promise<ProtocolVersionResolution> {
  return (await resolveProtocolContext(parameters)).protocolVersion;
}

type AclVersionInterval = SemverInterval & {
  readonly lowerBound: SemverIntervalLowerBound & { readonly comparator: 'ge' };
  readonly upperBound: SemverIntervalUpperBound & { readonly comparator: 'lt' };
};

type ProtocolVersionByAclVersionRule = {
  readonly acl: AclVersionInterval;
  readonly protocolVersion: ProtocolVersion;
};

type AclProtocolVersionTableEntry = {
  readonly acl: SemverInterval;
  readonly protocolVersion: ProtocolVersion;
};

type PubKeyCrsVersionByProtocolVersionRule = {
  readonly protocol: SemverInterval;
  readonly pubKeyCrsVersion: PubKeyCrsVersion;
};

type PubKeyCrsVersionByRelayerUrlRule = {
  readonly relayerUrl: string;
  readonly pubKeyCrsVersion: PubKeyCrsVersion;
};

type ProtocolContextResolverConfig = {
  readonly acl: {
    readonly sortedProtocolByVersion: readonly ProtocolVersionByAclVersionRule[];
  };
  readonly pubKeyCrs: {
    readonly knownByRelayerUrl: readonly PubKeyCrsVersionByRelayerUrlRule[];
    readonly generatedByProtocolVersion: readonly PubKeyCrsVersionByProtocolVersionRule[];
  };
};

const PROTOCOL_CONTEXT_RESOLVER_CONFIG = {
  acl: {
    /**
     * Known ACL-version to protocol-version mapping.
     *
     * Warning: this table must stay sorted by ascending non-overlapping ACL
     * intervals. Each rule must use a closed lower bound and an open upper
     * bound, so the first rule also defines the oldest ACL version this SDK
     * knows.
     */
    sortedProtocolByVersion: [
      {
        acl: {
          lowerBound: { version: '0.2.0', comparator: 'ge' },
          upperBound: { version: '0.3.0', comparator: 'lt' },
        },
        protocolVersion: '0.11.0',
      },
      {
        acl: {
          lowerBound: { version: '0.3.0', comparator: 'ge' },
          upperBound: { version: '0.4.0', comparator: 'lt' },
        },
        protocolVersion: '0.12.0',
      },
      {
        acl: {
          lowerBound: { version: '0.4.0', comparator: 'ge' },
          upperBound: { version: '0.5.0', comparator: 'lt' },
        },
        protocolVersion: '0.13.0',
      },
      {
        acl: {
          lowerBound: { version: '0.5.0', comparator: 'ge' },
          upperBound: { version: '0.6.0', comparator: 'lt' },
        },
        protocolVersion: '0.14.0',
      },
    ],
  },
  pubKeyCrs: {
    /**
     * PubKey/CRS versions known to be served by specific relayers.
     *
     * This table is intentionally limited to stable, public relayer URLs. Local
     * URLs are not mode signals: a localhost endpoint can run cleartext,
     * localstack, or a custom setup.
     *
     * This is a snapshot of what those relayers serve when this SDK is
     * released. After release, key rotation can change the PubKey version, and
     * CRS removal can change the PubKey/CRS material this table describes.
     *
     * This override is not meant to exist forever. It is a compatibility bridge
     * until the SDK can resolve the PubKey/CRS version from a robust source,
     * such as a relayer response, a key endpoint, or an on-chain protocol
     * configuration signal. A URL match below returns an exact resolution only
     * because the SDK release knows what that relayer served at publication
     * time; it does not prove that the relayer still serves the same key
     * material after future key rotations, CRS removal, or protocol upgrades.
     */
    knownByRelayerUrl: [
      {
        relayerUrl: mainnet.fhevm.relayerUrl,
        pubKeyCrsVersion: '1.4.0-alpha.3',
      },
      {
        relayerUrl: sepolia.fhevm.relayerUrl,
        pubKeyCrsVersion: '1.4.0-alpha.3',
      },
      {
        relayerUrl: 'https://relayer.dev.zama.cloud',
        pubKeyCrsVersion: '1.4.0-alpha.3',
      },
    ],
    /**
     * PubKey/CRS version produced when new key material is generated for a
     * protocol line.
     *
     * This is not a chain override. It answers: "if the KMS generated fresh
     * PubKey/CRS material for this protocol, which serialized format would it
     * produce?"
     *
     * Warning: these intervals must describe known protocol lines without
     * overlaps. Unknown older/newer protocol bounds intentionally fall back to
     * bounded PubKey/CRS resolutions instead of being treated as exact.
     */
    generatedByProtocolVersion: [
      {
        protocol: {
          lowerBound: { version: '0.11.0', comparator: 'ge' },
          upperBound: { version: '0.12.0', comparator: 'lt' },
        },
        pubKeyCrsVersion: '1.5.1',
      },
      {
        protocol: {
          lowerBound: { version: '0.12.0', comparator: 'ge' },
          upperBound: { version: '0.13.0', comparator: 'lt' },
        },
        pubKeyCrsVersion: '1.5.4',
      },
      {
        protocol: {
          lowerBound: { version: '0.13.0', comparator: 'ge' },
          upperBound: { version: '0.14.0', comparator: 'le' },
        },
        pubKeyCrsVersion: '1.6.1',
      },
    ],
  },
} as const satisfies ProtocolContextResolverConfig;

const SORTED_PROTOCOL_VERSION_BY_ACL_VERSION = PROTOCOL_CONTEXT_RESOLVER_CONFIG.acl.sortedProtocolByVersion;
_assertSortedContiguousAclProtocolTable(SORTED_PROTOCOL_VERSION_BY_ACL_VERSION);

const GENERATED_PUB_KEY_CRS_VERSION_BY_PROTOCOL_VERSION =
  PROTOCOL_CONTEXT_RESOLVER_CONFIG.pubKeyCrs.generatedByProtocolVersion;
_assertSortedContiguousGeneratedPubKeyCrsProtocolTable(GENERATED_PUB_KEY_CRS_VERSION_BY_PROTOCOL_VERSION);

export function protocolContextFromAclVersion(
  chain: FhevmChain,
  aclVersion: HostContractVersion<'ACL'>,
): FhevmProtocolContext {
  const protocolVersion = protocolVersionFromAclVersion(aclVersion);
  return Object.freeze({
    protocolVersion,
    pubKeyCrsVersion: pubKeyCrsVersionFromProtocolVersion(chain, protocolVersion),
  });
}

/**
 * Maps an ACL host-contract version to a protocol-version resolution known by
 * this SDK release.
 *
 * This function is intentionally table-driven: it must not infer future
 * protocol versions from ACL version arithmetic. If the ACL version is newer
 * than the SDK's table, it returns `{ comparator: 'gt' }` with the highest
 * known protocol version as the SDK's strict lower bound: the exact protocol
 * version is unknown, but it is greater than that value. If the ACL version is
 * older than the SDK's table, it returns `{ comparator: 'lt' }` with the lowest
 * known protocol version as the SDK's strict upper bound.
 *
 * Examples:
 * - `ACL v0.4.0` -> `{ version: '0.13.0', comparator: 'eq' }`
 * - `ACL v0.4.1` -> `{ version: '0.13.0', comparator: 'eq' }`
 * - `ACL v0.6.0` -> `{ version: '0.14.0', comparator: 'gt' }`
 * - `ACL v0.1.0` -> `{ version: '0.11.0', comparator: 'lt' }`
 */
export function protocolVersionFromAclVersion(aclVersion: HostContractVersion<'ACL'>): ProtocolVersionResolution {
  const aclSemver = _formatAclVersion(aclVersion);

  const known = SORTED_PROTOCOL_VERSION_BY_ACL_VERSION.find((entry) => isSemverInInterval(aclSemver, entry.acl));
  if (known !== undefined) {
    return Object.freeze({ version: known.protocolVersion, comparator: 'eq' });
  }

  const oldestKnownProtocol = _getOldestKnownProtocolVersion();
  if (_isSemverBeforeLowerBound(aclSemver, oldestKnownProtocol.acl.lowerBound)) {
    // The ACL is older than the first interval, so the actual protocol is
    // strictly lower than the oldest protocol this SDK knows.
    return Object.freeze({ version: oldestKnownProtocol.protocolVersion, comparator: 'lt' });
  }

  const newestKnownProtocol = _getNewestKnownProtocolVersion();
  if (_isSemverAfterUpperBound(aclSemver, newestKnownProtocol.acl.upperBound)) {
    // The ACL is newer than the last interval, so the actual protocol is
    // strictly greater than the newest protocol this SDK knows.
    return Object.freeze({ version: newestKnownProtocol.protocolVersion, comparator: 'gt' });
  }

  throw new Error(
    `Cannot resolve protocol version from ACL version ${aclSemver}: the ACL compatibility table has a gap.`,
  );
}

function _getOldestKnownProtocolVersion(): (typeof SORTED_PROTOCOL_VERSION_BY_ACL_VERSION)[number] {
  return _getFirstConfiguredEntry(SORTED_PROTOCOL_VERSION_BY_ACL_VERSION, 'No known ACL protocol versions configured.');
}

function _getNewestKnownProtocolVersion(): (typeof SORTED_PROTOCOL_VERSION_BY_ACL_VERSION)[number] {
  return _getLastConfiguredEntry(SORTED_PROTOCOL_VERSION_BY_ACL_VERSION, 'No known ACL protocol versions configured.');
}

function _formatAclVersion(version: {
  readonly major: number;
  readonly minor: number;
  readonly patch: number;
}): string {
  return `${version.major}.${version.minor}.${version.patch}`;
}

function _assertSortedContiguousAclProtocolTable(entries: readonly AclProtocolVersionTableEntry[]): void {
  if (entries.length === 0) {
    throw new Error('No known ACL protocol versions configured.');
  }

  let previousUpperBound: SemverIntervalUpperBound | undefined;
  let previousProtocolVersion: ProtocolVersion | undefined;

  for (const [index, entry] of entries.entries()) {
    const lowerBound = entry.acl.lowerBound;
    const upperBound = entry.acl.upperBound;

    if (lowerBound === undefined || upperBound === undefined) {
      throw new Error(`ACL protocol rule at index ${index} must define both lower and upper bounds.`);
    }
    if (lowerBound.comparator !== 'ge' || upperBound.comparator !== 'lt') {
      throw new Error(`ACL protocol rule at index ${index} must use a [lower, upper) interval.`);
    }
    if (compareSemver(lowerBound.version, upperBound.version) >= 0) {
      throw new Error(`ACL protocol rule at index ${index} has an invalid interval.`);
    }
    if (previousUpperBound !== undefined && compareSemver(previousUpperBound.version, lowerBound.version) !== 0) {
      throw new Error(`ACL protocol rules must be contiguous at index ${index}.`);
    }
    if (previousProtocolVersion !== undefined && compareSemver(previousProtocolVersion, entry.protocolVersion) >= 0) {
      throw new Error(`ACL protocol versions must be strictly ascending at index ${index}.`);
    }

    previousUpperBound = upperBound;
    previousProtocolVersion = entry.protocolVersion;
  }
}

function _assertSortedContiguousGeneratedPubKeyCrsProtocolTable(
  entries: readonly PubKeyCrsVersionByProtocolVersionRule[],
): void {
  if (entries.length === 0) {
    throw new Error('No known generated PubKey/CRS versions configured.');
  }

  let previousUpperBound: SemverIntervalUpperBound | undefined;
  let previousPubKeyCrsVersion: PubKeyCrsVersion | undefined;

  for (const [index, entry] of entries.entries()) {
    const lowerBound = entry.protocol.lowerBound;
    const upperBound = entry.protocol.upperBound;
    const isLastEntry = index === entries.length - 1;

    if (lowerBound === undefined || upperBound === undefined) {
      throw new Error(`Generated PubKey/CRS protocol rule at index ${index} must define both lower and upper bounds.`);
    }
    if (lowerBound.comparator !== 'ge') {
      throw new Error(`Generated PubKey/CRS protocol rule at index ${index} must use a closed lower bound.`);
    }
    if (!isLastEntry && upperBound.comparator !== 'lt') {
      throw new Error(
        `Generated PubKey/CRS protocol rule at index ${index} must use an open upper bound, except the final rule may use a closed upper bound.`,
      );
    }
    if (compareSemver(lowerBound.version, upperBound.version) >= 0) {
      throw new Error(`Generated PubKey/CRS protocol rule at index ${index} has an invalid interval.`);
    }
    if (previousUpperBound !== undefined && compareSemver(previousUpperBound.version, lowerBound.version) !== 0) {
      throw new Error(`Generated PubKey/CRS protocol rules must be contiguous at index ${index}.`);
    }
    // The bounded fallback treats the first/last entries as the oldest/newest
    // PubKey/CRS versions, so the values must increase with the protocol line.
    if (
      previousPubKeyCrsVersion !== undefined &&
      compareSemver(previousPubKeyCrsVersion, entry.pubKeyCrsVersion) >= 0
    ) {
      throw new Error(`Generated PubKey/CRS versions must be strictly ascending at index ${index}.`);
    }

    previousUpperBound = upperBound;
    previousPubKeyCrsVersion = entry.pubKeyCrsVersion;
  }
}

function _isSemverBeforeLowerBound(version: string, lowerBound: SemverIntervalLowerBound): boolean {
  const comparison = compareSemver(version, lowerBound.version);
  return lowerBound.comparator === 'ge' ? comparison < 0 : comparison <= 0;
}

function _isSemverAfterUpperBound(version: string, upperBound: SemverIntervalUpperBound): boolean {
  const comparison = compareSemver(version, upperBound.version);
  return upperBound.comparator === 'lt' ? comparison >= 0 : comparison > 0;
}

export function pubKeyCrsVersionFromProtocolVersion(
  chain: FhevmChain,
  protocolVersion: ProtocolVersionResolution,
): PubKeyCrsVersionResolution {
  const relayerUrl = _normalizeRelayerUrl(chain.fhevm.relayerUrl);
  const knownByRelayerUrl = PROTOCOL_CONTEXT_RESOLVER_CONFIG.pubKeyCrs.knownByRelayerUrl.find(
    (entry) => _normalizeRelayerUrl(entry.relayerUrl) === relayerUrl,
  );
  if (knownByRelayerUrl !== undefined) {
    return _actualPubKeyCrsEquals(knownByRelayerUrl.pubKeyCrsVersion);
  }

  const known = GENERATED_PUB_KEY_CRS_VERSION_BY_PROTOCOL_VERSION.filter((entry) =>
    _protocolVersionResolutionImpliesInterval(protocolVersion, entry.protocol),
  );
  const knownRule = known[0];
  if (known.length === 1 && knownRule !== undefined) {
    return _actualPubKeyCrsEquals(knownRule.pubKeyCrsVersion);
  }
  if (known.length > 1) {
    throw new Error(
      `Ambiguous generated PubKey/CRS version for protocol ${protocolVersion.comparator}:${protocolVersion.version}.`,
    );
  }

  return _boundedPubKeyCrsVersionFromUnknownProtocolVersion(protocolVersion);
}

function _protocolVersionResolutionImpliesInterval(
  protocolVersion: ProtocolVersionResolution,
  interval: SemverInterval,
): boolean {
  if (
    interval.lowerBound !== undefined &&
    !semverComparatorImpliesRange(protocolVersion.version, protocolVersion.comparator, interval.lowerBound)
  ) {
    return false;
  }
  if (
    interval.upperBound !== undefined &&
    !semverComparatorImpliesRange(protocolVersion.version, protocolVersion.comparator, interval.upperBound)
  ) {
    return false;
  }
  return true;
}

function _boundedPubKeyCrsVersionFromUnknownProtocolVersion(
  protocolVersion: ProtocolVersionResolution,
): PubKeyCrsVersionResolution {
  const oldestKnownProtocolVersion = _getOldestKnownProtocolVersion().protocolVersion;
  const newestKnownProtocolVersion = _getNewestKnownProtocolVersion().protocolVersion;

  if (protocolVersion.comparator === 'lt' && compareSemver(protocolVersion.version, oldestKnownProtocolVersion) <= 0) {
    return Object.freeze({ version: _getOldestKnownPubKeyCrsVersion(), comparator: 'lt' });
  }
  if (protocolVersion.comparator === 'gt' && compareSemver(protocolVersion.version, newestKnownProtocolVersion) >= 0) {
    return Object.freeze({ version: _getNewestKnownPubKeyCrsVersion(), comparator: 'gt' });
  }
  if (protocolVersion.comparator === 'eq' && compareSemver(protocolVersion.version, oldestKnownProtocolVersion) < 0) {
    return Object.freeze({ version: _getOldestKnownPubKeyCrsVersion(), comparator: 'lt' });
  }
  if (protocolVersion.comparator === 'eq' && compareSemver(protocolVersion.version, newestKnownProtocolVersion) > 0) {
    return Object.freeze({ version: _getNewestKnownPubKeyCrsVersion(), comparator: 'gt' });
  }

  throw new Error(
    `Cannot resolve generated PubKey/CRS version from ambiguous protocol ${protocolVersion.comparator}:${protocolVersion.version}.`,
  );
}

function _getOldestKnownPubKeyCrsVersion(): PubKeyCrsVersion {
  return _getFirstConfiguredEntry(
    GENERATED_PUB_KEY_CRS_VERSION_BY_PROTOCOL_VERSION,
    'No known PubKey/CRS versions configured.',
  ).pubKeyCrsVersion;
}

function _getNewestKnownPubKeyCrsVersion(): PubKeyCrsVersion {
  return _getLastConfiguredEntry(
    GENERATED_PUB_KEY_CRS_VERSION_BY_PROTOCOL_VERSION,
    'No known PubKey/CRS versions configured.',
  ).pubKeyCrsVersion;
}

function _getFirstConfiguredEntry<T>(entries: readonly T[], errorMessage: string): T {
  const entry = entries[0];
  if (entry === undefined) {
    throw new Error(errorMessage);
  }
  return entry;
}

function _getLastConfiguredEntry<T>(entries: readonly T[], errorMessage: string): T {
  const entry = entries[entries.length - 1];
  if (entry === undefined) {
    throw new Error(errorMessage);
  }
  return entry;
}

function _actualPubKeyCrsEquals(version: PubKeyCrsVersion): PubKeyCrsVersionResolution {
  return Object.freeze({ version, comparator: 'eq' });
}

function _normalizeRelayerUrl(relayerUrl: string): string {
  return relayerUrl.replace(/\/+$/, '');
}

import type { PubKeyCrsVersion, PubKeyCrsVersionResolution } from '../types/coreFhevmClient.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import { mainnet } from '../chains/definitions/mainnet.js';
import { sepolia } from '../chains/definitions/sepolia.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * PubKey/CRS version produced when new key material is generated for the
 * protocol line this SDK targets.
 */
const GENERATED_PUB_KEY_CRS_VERSION: PubKeyCrsVersion = '1.6.1';

type PubKeyCrsVersionByRelayerUrlRule = {
  readonly relayerUrl: string;
  readonly pubKeyCrsVersion: PubKeyCrsVersion;
};

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
const PUB_KEY_CRS_VERSION_BY_RELAYER_URL: readonly PubKeyCrsVersionByRelayerUrlRule[] = [
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
];

/**
 * Returns the PubKey/CRS version for a chain, keyed on its relayer URL.
 */
export function pubKeyCrsVersionForChain(chain: FhevmChain): PubKeyCrsVersionResolution {
  const relayerUrl = _normalizeRelayerUrl(chain.fhevm.relayerUrl);
  const knownByRelayerUrl = PUB_KEY_CRS_VERSION_BY_RELAYER_URL.find(
    (entry) => _normalizeRelayerUrl(entry.relayerUrl) === relayerUrl,
  );

  return Object.freeze({
    version: knownByRelayerUrl?.pubKeyCrsVersion ?? GENERATED_PUB_KEY_CRS_VERSION,
    comparator: 'eq',
  });
}

function _normalizeRelayerUrl(relayerUrl: string): string {
  return relayerUrl.replace(/\/+$/, '');
}

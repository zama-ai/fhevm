import type {
  FhevmProtocolContext,
  ProtocolVersion,
  ProtocolVersionResolution,
  PubKeyCrsVersion,
  PubKeyCrsVersionResolution,
} from '../types/coreFhevmClient.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import { mainnet } from '../chains/definitions/mainnet.js';
import { sepolia } from '../chains/definitions/sepolia.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * The FHEVM protocol version this SDK targets.
 *
 * The SDK does not resolve the protocol version from on-chain contract
 * versions: each SDK release targets a single protocol line, and every chain
 * is assumed to run it.
 */
const FIXED_PROTOCOL_VERSION: ProtocolVersion = '0.15.0';

const FIXED_PROTOCOL_VERSION_RESOLUTION: ProtocolVersionResolution = Object.freeze({
  version: FIXED_PROTOCOL_VERSION,
  comparator: 'eq',
});

/**
 * PubKey/CRS version produced when new key material is generated for the
 * protocol line this SDK targets ({@link FIXED_PROTOCOL_VERSION}).
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
 * Returns the protocol context (protocol + PubKey/CRS versions) for a chain.
 *
 * The protocol version is always {@link FIXED_PROTOCOL_VERSION}; only the
 * PubKey/CRS version depends on the chain (via its relayer URL).
 */
export function protocolContextForChain(chain: FhevmChain): FhevmProtocolContext {
  return Object.freeze({
    protocolVersion: FIXED_PROTOCOL_VERSION_RESOLUTION,
    pubKeyCrsVersion: pubKeyCrsVersionForChain(chain),
  });
}

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

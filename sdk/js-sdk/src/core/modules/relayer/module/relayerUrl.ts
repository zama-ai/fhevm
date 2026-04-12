import { removeSuffix } from '../../../base/string.js';

export const ZamaMainnetRelayerBaseUrl = 'https://relayer.mainnet.zama.org';
export const ZamaMainnetRelayerUrlV2 = `${ZamaMainnetRelayerBaseUrl}/v2`;

export const ZamaSepoliaRelayerBaseUrl = 'https://relayer.testnet.zama.org';
export const ZamaSepoliaRelayerUrlV2 = `${ZamaSepoliaRelayerBaseUrl}/v2`;

export function parseZamaRelayerUrl(relayerUrl: unknown): string | null {
  if (
    relayerUrl === undefined ||
    relayerUrl === null ||
    typeof relayerUrl !== 'string'
  ) {
    return null;
  }

  const urlNoSlash = removeSuffix(relayerUrl, '/');
  if (!URL.canParse(urlNoSlash)) {
    return null;
  }

  if (
    urlNoSlash.startsWith(ZamaMainnetRelayerBaseUrl) ||
    urlNoSlash.startsWith(ZamaSepoliaRelayerBaseUrl)
  ) {
    const zamaUrls = [
      ZamaSepoliaRelayerBaseUrl,
      ZamaSepoliaRelayerUrlV2,
      ZamaMainnetRelayerBaseUrl,
      ZamaMainnetRelayerUrlV2,
    ];
    const isZamaUrl = zamaUrls.includes(urlNoSlash);
    if (isZamaUrl) {
      if (urlNoSlash.endsWith('/v2')) {
        return urlNoSlash;
      }
      return `${urlNoSlash}/v2`;
    }
    // malformed Zama url
    return null;
  }

  return urlNoSlash;
}

import type { Auth } from '../../types/auth.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';
import { ensureRelayerFeatures } from '../../relayerFeatures/ensureRelayerFeatures-p.js';
//import { SDK_PROTOCOL_API_MAJOR_VERSION, SDK_PROTOCOL_API_MINOR_VERSION } from '../../runtime/sdkProtocolApiVersion.js';

////////////////////////////////////////////////////////////////////////////////

export type CanUseUnifiedDecryptionPermitParameters = {
  readonly options?: { readonly auth?: Auth | undefined } | undefined;
};

export type CanUseUnifiedDecryptionPermitReturnType = boolean;

////////////////////////////////////////////////////////////////////////////////

/**
 * Reports whether this client can use **unified** decryption permits.
 *
 * The capability is gated by the relayer's feature list — specifically `supportsRouteV3` (the
 * unified user-decrypt route). The features are resolved **once per client** and pinned (see
 * `ensureRelayerFeatures`), so repeated calls are cheap and never drift.
 *
 * `options.auth` is only used to *resolve* the features on relayers that require a key (e.g.
 * mainnet); on a keyless relayer (e.g. Sepolia) it can be omitted. Once resolved, the result is
 * cached and subsequent calls need no auth. If the relayer requires a key and none is supplied, the
 * underlying resolution fails loudly rather than returning a default.
 */
export async function canUseUnifiedDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters?: CanUseUnifiedDecryptionPermitParameters,
): Promise<CanUseUnifiedDecryptionPermitReturnType> {
  await initPublicAction(fhevm);

  // // sdk api cap
  // if (SDK_PROTOCOL_API_MAJOR_VERSION === 0 && SDK_PROTOCOL_API_MINOR_VERSION <= 13) {
  //   throw new Error(
  //     `Unified (V2) decryption permits are not supported: this SDK uses protocol API v0.13.x, which only supports V1 decryption permits. Creating a unified permit requires an SDK using protocol API v0.14.0 or later.`,
  //   );
  // }

  const features = await ensureRelayerFeatures(fhevm, { auth: parameters?.options?.auth });

  return features.supportsRouteV3;
}

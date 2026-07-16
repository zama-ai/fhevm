import type { ProtocolVersionResolution } from '../types/coreFhevmClient.js';
import { isSemverStrictlyBefore } from '../base/semver.js';

//////////////////////////////////////////////////////////////////////////////
// userDecryptFlowVersion
//////////////////////////////////////////////////////////////////////////////

/**
 * Temporary kill switch for the relayer v3 routes.
 *
 * On protocol >= 0.14.0 user decryption normally switches to the V2 flow:
 * V2 EIP-712 permit, unified KMS payload and the relayer `v3/user-decrypt`
 * route. The v3 relayer routes are not available yet, so while this flag is
 * `false` every protocol version stays on the V1 flow (`v2/user-decrypt` and
 * `v2/delegated-user-decrypt` routes).
 *
 * Flip back to `true` to restore protocol-version-based selection.
 */
const RELAYER_V3_ROUTES_ENABLED: boolean = false;

/**
 * Returns `true` when the V2 user-decrypt flow (V2 permit, unified KMS
 * payload, relayer `v3/user-decrypt` route) must be used for the given
 * protocol version.
 *
 * Always returns `false` while {@link RELAYER_V3_ROUTES_ENABLED} is off.
 */
export function shouldUseUserDecryptV2(protocolVersion: ProtocolVersionResolution): boolean {
  if (!RELAYER_V3_ROUTES_ENABLED) {
    return false;
  }
  return !isSemverStrictlyBefore(protocolVersion.version, '0.14.0');
}

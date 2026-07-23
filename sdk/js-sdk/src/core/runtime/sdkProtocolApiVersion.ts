/**
 * The FHEVM protocol API version this SDK is using (currently `0.13.0`).
 *
 * This is a static compile-time constant, distinct from the *dynamic*
 * on-chain protocol version reported by `resolveProtocolVersion()` /
 * `fhevm.protocolVersion`.
 *
 * ## Supported protocol versions
 *
 * This SDK natively supports FHEVM protocol **v11, v12, and v13** — it knows
 * each of their on-chain APIs and encodes requests accordingly.
 *
 * It is also **guaranteed to work on v14 and later**: the FHEVM protocol
 * guarantees backward compatibility, so a chain running a newer protocol
 * version still accepts the v13 API this SDK speaks. Against such a chain the
 * SDK keeps using its v13 API — it does not opt into any v14+ feature it does
 * not yet know about (for example, the v14 API introduces V2 decryption
 * permits, which this SDK does not emit).
 */
export const SDK_PROTOCOL_API_MAJOR_VERSION: number = 0;
export const SDK_PROTOCOL_API_MINOR_VERSION: number = 13;
export const SDK_PROTOCOL_API_PATCH_VERSION: number = 0;

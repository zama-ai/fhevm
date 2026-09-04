/**
 * The FHEVM protocol API version this SDK is using (currently `0.15.0`).
 *
 * This is a static compile-time constant, distinct from the *dynamic*
 * on-chain protocol version reported by `resolveProtocolVersion()` /
 * `fhevm.protocolVersion`.
 *
 * ## Supported protocol versions
 *
 * This SDK natively supports FHEVM protocol **v11, v12, v13, v14, and v15** —
 * it knows each of their on-chain APIs and encodes requests accordingly.
 *
 * It is also **guaranteed to work on v16 and later**: the FHEVM protocol
 * guarantees backward compatibility, so a chain running a newer protocol
 * version still accepts the v15 API this SDK speaks. Against such a chain the
 * SDK keeps using its v15 API — it does not opt into any v16+ feature it does
 * not yet know about.
 */
export const SDK_PROTOCOL_API_MAJOR_VERSION: number = 0;
export const SDK_PROTOCOL_API_MINOR_VERSION: number = 15;
export const SDK_PROTOCOL_API_PATCH_VERSION: number = 0;

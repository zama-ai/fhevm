/**
 * The FHEVM protocol API version this SDK is using (currently `0.14.0`).
 *
 * This is a static compile-time constant, distinct from the *dynamic*
 * on-chain protocol version reported by `resolveProtocolVersion()` /
 * `fhevm.protocolVersion`.
 *
 * ## Supported protocol versions
 *
 * This SDK natively supports FHEVM protocol **v11, v12, v13, and v14** — it
 * knows each of their on-chain APIs and encodes requests accordingly.
 *
 * It is also **guaranteed to work on v15 and later**: the FHEVM protocol
 * guarantees backward compatibility, so a chain running a newer protocol
 * version still accepts the v14 API this SDK speaks. Against such a chain the
 * SDK keeps using its v14 API — it does not opt into any v15+ feature it does
 * not yet know about.
 */
export const SDK_PROTOCOL_API_MAJOR_VERSION: number = 0;
export const SDK_PROTOCOL_API_MINOR_VERSION: number = 14;
export const SDK_PROTOCOL_API_PATCH_VERSION: number = 0;

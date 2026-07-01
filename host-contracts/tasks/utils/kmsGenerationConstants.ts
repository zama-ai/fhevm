const REQUEST_TYPE_SHIFT = 248n;

// Values 1 and 2 are reserved for Gateway public/user decryption request IDs.
// Host KMS generation keeps the historical Gateway tags so migrated request IDs stay unchanged.
// Values 7 and 8 are consensus-confirmed ProtocolConfig lifecycle IDs.
const enum RequestType {
  _deprecated_ = 0,
  _gatewayPublicDecrypt_ = 1,
  _gatewayUserDecrypt_ = 2,
  PrepKeygen = 3,
  Keygen = 4,
  Crsgen = 5,
  _deprecatedKeyReshare_ = 6,
  KmsContext = 7,
  Epoch = 8,
}

export const KMS_CONTEXT_COUNTER_BASE = BigInt(RequestType.KmsContext) << REQUEST_TYPE_SHIFT;
export const PREP_KEYGEN_COUNTER_BASE = BigInt(RequestType.PrepKeygen) << REQUEST_TYPE_SHIFT;
export const KEY_COUNTER_BASE = BigInt(RequestType.Keygen) << REQUEST_TYPE_SHIFT;
export const CRS_COUNTER_BASE = BigInt(RequestType.Crsgen) << REQUEST_TYPE_SHIFT;

// Mirror of the host KMS identifier type-tag family. The single Solidity source of truth lives in
// host-contracts/contracts/shared/Constants.sol (shift + context tag) and KMSGeneration.sol (keygen
// tags). Keep the shift and tag values below in sync with that source.
const REQUEST_TYPE_SHIFT = 248n;

export const KMS_CONTEXT_COUNTER_BASE = 0x07n << REQUEST_TYPE_SHIFT;
export const PREP_KEYGEN_COUNTER_BASE = 0x03n << REQUEST_TYPE_SHIFT;
export const KEY_COUNTER_BASE = 0x04n << REQUEST_TYPE_SHIFT;
export const CRS_COUNTER_BASE = 0x05n << REQUEST_TYPE_SHIFT;

export {
  type DecryptPublicValueParameters,
  type DecryptPublicValueReturnType,
  decryptPublicValue,
} from './decryptPublicValue.js';

export type { RelayerPublicDecryptOptions, RelayerPublicDecryptProgressArgs } from '../../types/relayer.js';

export {
  type DecryptPublicValuesParameters,
  type DecryptPublicValuesReturnType,
  decryptPublicValues,
} from './decryptPublicValues.js';

export {
  type DecryptPublicValuesWithSignaturesParameters,
  type DecryptPublicValuesWithSignaturesReturnType,
  decryptPublicValuesWithSignatures,
} from './decryptPublicValuesWithSignatures.js';

export {
  type CanDecryptPublicValueParameters,
  type CanDecryptPublicValueReturnType,
  canDecryptPublicValue,
} from './canDecryptPublicValue.js';

export {
  type CanDecryptPublicValuesParameters,
  type CanDecryptPublicValuesReturnType,
  canDecryptPublicValues,
} from './canDecryptPublicValues.js';

export {
  type FetchEncryptedValuesParameters,
  type FetchEncryptedValuesReturnType,
  fetchEncryptedValues,
} from './fetchEncryptedValues.js';

// V1 permit (protocol v13 and below) — kept for chains/relayers not yet on v14.
export {
  type CreateUnsignedLegacyDecryptionPermitEip712Parameters,
  type CreateUnsignedLegacyDecryptionPermitEip712ReturnType,
  createUnsignedLegacyDecryptionPermitEip712,
} from './createUnsignedLegacyDecryptionPermitEip712.js';

export {
  type CanUseUnifiedDecryptionPermitParameters,
  type CanUseUnifiedDecryptionPermitReturnType,
  canUseUnifiedDecryptionPermit,
} from './canUseUnifiedDecryptionPermit.js';

// V1 permit (protocol v13 and below) — kept for chains/relayers not yet on v14.
export {
  type SignLegacyDecryptionPermitParameters,
  type SignLegacyDecryptionPermitReturnType,
  signLegacyDecryptionPermit,
} from './signLegacyDecryptionPermit.js';

// V2 permit (protocol v14 and above) — requires an SDK on protocol API v0.14.0+
// and a chain whose KMSVerifier/ProtocolConfig support the unified extraData v2.
export {
  type CreateUnsignedUnifiedDecryptionPermitEip712Parameters,
  type CreateUnsignedUnifiedDecryptionPermitEip712ReturnType,
  createUnsignedUnifiedDecryptionPermitEip712,
} from './createUnsignedUnifiedDecryptionPermitEip712.js';

// V2 permit (protocol v14 and above) — requires an SDK on protocol API v0.14.0+
// and a chain whose KMSVerifier/ProtocolConfig support the unified extraData v2.
export {
  type SignUnifiedDecryptionPermitParameters,
  type SignUnifiedDecryptionPermitReturnType,
  signUnifiedDecryptionPermit,
} from './signUnifiedDecryptionPermit.js';

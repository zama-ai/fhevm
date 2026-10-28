import type { Prettify } from '../../types/utils.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { ReconstructedPads, ReconstructPadsParameters } from '../../kms/reconstructPads-p.js';
import type {
  GenerateTkmsPrivateKeyModuleFunction,
  GetTkmsModuleInfoFunction,
  GetTkmsPublicKeyHexModuleFunction,
  InitTkmsModuleFunction,
  SerializeTkmsPrivateKeyModuleFunction,
} from '../decrypt/types.js';

////////////////////////////////////////////////////////////////////////////////
//
// EncryptTransciModule
//
// RFC-030 pad-based input encryption ("transciphering"). No TFHE: the pad is
// obtained from the KMS and the values are masked client-side. The module
// owns the client-side processing of the pad decryption responses, which is
// built on the TKMS WASM (ML-KEM transport key, threshold share decoding), and
// the generation of the transport key pair the pad shares are encrypted to.
//
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// reconstructPadsFunction
////////////////////////////////////////////////////////////////////////////////

export type ReconstructPadsModuleParameters = ReconstructPadsParameters;
export type ReconstructPadsModuleReturnType = ReconstructedPads;

export type ReconstructPadsModuleFunction = {
  /**
   * Opens and verifies the pad decryption responses, then reconstructs one pad
   * per witness label.
   */
  reconstructPads(parameters: ReconstructPadsModuleParameters): Promise<ReconstructPadsModuleReturnType>;
};

////////////////////////////////////////////////////////////////////////////////

export type WithEncryptTransciModule = {
  readonly encryptTransci: EncryptTransciModule;
};

export type EncryptTransciModule = Prettify<
  InitTkmsModuleFunction &
    GetTkmsModuleInfoFunction &
    ReconstructPadsModuleFunction &
    GenerateTkmsPrivateKeyModuleFunction &
    SerializeTkmsPrivateKeyModuleFunction &
    GetTkmsPublicKeyHexModuleFunction
>;

export type EncryptTransciModuleFactory = (runtime: FhevmRuntime) => {
  readonly encryptTransci: EncryptTransciModule;
};

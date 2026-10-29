import type { Prettify } from '../../types/utils.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { Bytes32Hex, BytesHex, ChecksummedAddress, Uint64BigInt } from '../../types/primitives.js';
import type { PadDecryptResponse, PadWitness } from '../relayer/types.js';
import type {
  GenerateTkmsPrivateKeyModuleFunction,
  GetTkmsModuleInfoFunction,
  GetTkmsPublicKeyHexModuleFunction,
  InitTkmsModuleFunction,
  SerializeTkmsPrivateKeyModuleFunction,
} from '../decrypt/types.js';

////////////////////////////////////////////////////////////////////////////////
//
// EncryptWithPadModule
//
// RFC-030 pad-based input encryption. No TFHE: the pad is
// obtained from the KMS and the values are masked client-side. The module
// owns the client-side processing of the pad decryption responses, which is
// built on the TKMS WASM (ML-KEM transport key, threshold share decoding), and
// the generation of the transport key pair the pad shares are encrypted to.
//
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// reconstructPadsFunction
////////////////////////////////////////////////////////////////////////////////

export type ReconstructPadsParameters = {
  readonly responses: readonly PadDecryptResponse[];
  /** The witnesses of the request, i.e. the client's own local record. */
  readonly witnesses: readonly PadWitness[];
  /** The request's signed `extraData`. */
  readonly extraData: BytesHex;
  /** The request's transport public key (`auth.publicKey`). */
  readonly transportPublicKey: BytesHex;
  /** KMS signers of the context named by `extraData`. */
  readonly kmsSigners: readonly ChecksummedAddress[];
  /**
   * Sharing degree `t` of the context named by `extraData` (its MPC threshold).
   * A response must be signed with this degree, and a pad needs 2t+1 of them:
   * the degree a response signs for itself is never trusted to set the bar.
   */
  readonly mpcThreshold: number;
  /** EIP-712 domain of the response signature (gateway chain, KMS `Decryption` domain). */
  readonly gatewayChainId: Uint64BigInt | number;
  readonly verifyingContractAddressDecryption: ChecksummedAddress;
};

/** Plaintext pads keyed by label; each is ⌈padLength/8⌉ bytes, LSB-first bit order. */
export type ReconstructedPads = ReadonlyMap<Bytes32Hex, Uint8Array>;

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

export type WithEncryptWithPadModule = {
  readonly encryptWithPad: EncryptWithPadModule;
};

export type EncryptWithPadModule = Prettify<
  InitTkmsModuleFunction &
    GetTkmsModuleInfoFunction &
    ReconstructPadsModuleFunction &
    GenerateTkmsPrivateKeyModuleFunction &
    SerializeTkmsPrivateKeyModuleFunction &
    GetTkmsPublicKeyHexModuleFunction
>;

export type EncryptWithPadModuleFactory = (runtime: FhevmRuntime) => {
  readonly encryptWithPad: EncryptWithPadModule;
};

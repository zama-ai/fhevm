import type { Prettify } from '../../types/utils.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { Bytes32Hex, BytesHex, ChecksummedAddress, Uint32Number, Uint64BigInt } from '../../types/primitives.js';
import type { PadDecryptResponse, PadEntry } from '../relayer/types.js';
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
// PadDecryptResponsePayload (decrypted content of a PadDecryptResponse)
////////////////////////////////////////////////////////////////////////////////

/**
 * One node's share of one pad, keyed by label (`PadShare` in RFC-030), signed on its own together with the
 * payload's `extraData`.
 */
export type PadShare = {
  readonly label: Bytes32Hex;
  /** Encoded ring elements, opaque: the encoding and its metadata are the KMS's. */
  readonly rawShares: BytesHex;
  /** This node's 1-indexed evaluation point. */
  readonly partyId: Uint32Number;
  /** The sharing polynomial degree `t`. */
  readonly degree: Uint32Number;
  /** 16 bytes: opaque id of the MPC session that produced the share. */
  readonly sessionId: BytesHex;
  /** EIP-712 signature over the share's `PadShareVerification`, which includes the payload's `extraData`. */
  readonly signature: BytesHex;
};

/** The decrypted content of a pad decryption response (`PadDecryptResponsePayload` in RFC-030). */
export type PadDecryptResponsePayload = {
  /** One per pad in the request; order carries no meaning. */
  readonly shares: readonly PadShare[];
  /** Response metadata naming the KMS context and epoch the request was served under. */
  readonly extraData: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////
// reconstructPadsFunction
////////////////////////////////////////////////////////////////////////////////

export type ReconstructPadsParameters = {
  readonly responses: readonly PadDecryptResponse[];
  /** The pads of the request, i.e. the client's own local record. */
  readonly pads: readonly PadEntry[];
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
  /**
   * EIP-712 domain of the response signatures (`fhevm-pad-decryption`, version 1): the chain id and address of
   * ProtocolConfig.
   */
  readonly protocolConfigChainId: Uint64BigInt | number;
  readonly protocolConfigAddress: ChecksummedAddress;
};

/** Plaintext pads keyed by label; each is ⌈padLength/8⌉ bytes, LSB-first bit order. */
export type ReconstructedPads = ReadonlyMap<Bytes32Hex, Uint8Array>;

export type ReconstructPadsModuleParameters = ReconstructPadsParameters;
export type ReconstructPadsModuleReturnType = ReconstructedPads;

export type ReconstructPadsModuleFunction = {
  /**
   * Opens and verifies the pad decryption responses, then reconstructs one pad
   * per requested label.
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

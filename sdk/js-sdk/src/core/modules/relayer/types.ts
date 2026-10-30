import type {
  RelayerCommonOptions,
  RelayerDelegatedUserDecryptOptions,
  RelayerInputProofOptions,
  RelayerKeyUrlOptions,
  RelayerPublicDecryptOptions,
  RelayerUserDecryptOptions,
} from '../../types/relayer.js';
import type { FheEncryptionKeyBytes, FheEncryptionKeySource } from '../../types/fheEncryptionKey.js';
import type { KmsSigncryptedShare } from '../../types/kms-p.js';
import type {
  KmsDelegatedUserDecryptEip712V1Message,
  KmsUserDecryptEip712V1Message,
  KmsUserDecryptEip712V2Message,
} from '../../types/kms.js';
import type {
  Bytes32Hex,
  Bytes65Hex,
  BytesHex,
  ChecksummedAddress,
  Uint16Number,
  Uint32Number,
  Uint64BigInt,
  Uint8Number,
} from '../../types/primitives.js';
import type { Prettify } from '../../types/utils.js';
import type { ZkProof } from '../../types/zkProof-p.js';
import type { Handle, InputHandle } from '../../types/encryptedTypes-p.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmClientFrozenContext } from '../../types/fhevmClientFrozenContext-p.js';
import type { Auth } from '../../types/auth.js';

////////////////////////////////////////////////////////////////////////////////
//
// RelayerModule
//
////////////////////////////////////////////////////////////////////////////////

export type RelayerClient = {
  readonly relayerUrl: string;
  readonly chainId: number;
};

export type RelayerClientWithRuntime = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

////////////////////////////////////////////////////////////////////////////////
// 1.1 fetchFheEncryptionKeySource
////////////////////////////////////////////////////////////////////////////////

export type FetchFheEncryptionKeySourceParameters = {
  readonly options?: RelayerKeyUrlOptions | undefined;
};

export type FetchFheEncryptionKeySourceReturnType = FheEncryptionKeySource;

export type FetchFheEncryptionKeySourceModuleFunction = {
  fetchFheEncryptionKeySource(
    relayerClient: RelayerClient,
    parameters: FetchFheEncryptionKeySourceParameters,
  ): Promise<FetchFheEncryptionKeySourceReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 1.2 fetchFheEncryptionKeyBytes
////////////////////////////////////////////////////////////////////////////////

export type FetchFheEncryptionKeyBytesParameters = {
  readonly options?: RelayerKeyUrlOptions | undefined;
};

export type FetchFheEncryptionKeyBytesReturnType = FheEncryptionKeyBytes;

export type FetchFheEncryptionKeyBytesModuleFunction = {
  fetchFheEncryptionKeyBytes(
    relayerClient: RelayerClient,
    parameters: FetchFheEncryptionKeyBytesParameters,
  ): Promise<FetchFheEncryptionKeyBytesReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 2. fetchCoprocessorSignatures
////////////////////////////////////////////////////////////////////////////////

export type FetchCoprocessorSignaturesParameters = {
  readonly payload: {
    readonly zkProof: ZkProof;
  };
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerInputProofOptions | undefined;
};

export type FetchCoprocessorSignaturesReturnType = {
  readonly handles: readonly InputHandle[];
  readonly coprocessorEip712Signatures: readonly Bytes65Hex[];
  readonly extraData: BytesHex;
};

export type FetchCoprocessorSignaturesModuleFunction = {
  fetchCoprocessorSignatures(
    relayerClient: RelayerClientWithRuntime,
    parameters: FetchCoprocessorSignaturesParameters,
  ): Promise<FetchCoprocessorSignaturesReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 3. fetchPublicDecrypt
////////////////////////////////////////////////////////////////////////////////

export type FetchPublicDecryptParameters = {
  readonly payload: {
    readonly orderedHandles: readonly Handle[];
    readonly extraData: BytesHex;
  };
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type FetchPublicDecryptReturnType = {
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly kmsPublicDecryptEip712Signatures: Bytes65Hex[];
  readonly extraData: BytesHex;
};

export type FetchPublicDecryptModuleFunction = {
  fetchPublicDecrypt(
    relayerClient: RelayerClientWithRuntime,
    parameters: FetchPublicDecryptParameters,
  ): Promise<FetchPublicDecryptReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 4. fetchUserDecrypt
////////////////////////////////////////////////////////////////////////////////

export type FetchUserDecryptParametersV1 = {
  readonly version: 1;
  readonly payload: {
    readonly handleContractPairs: ReadonlyArray<{
      readonly handle: Handle;
      readonly contractAddress: ChecksummedAddress;
    }>;
    readonly kmsDecryptEip712Signer: ChecksummedAddress;
    readonly kmsDecryptEip712Message: KmsUserDecryptEip712V1Message;
    readonly kmsDecryptEip712Signature: Bytes65Hex;
  };
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerUserDecryptOptions | undefined;
};

export type FetchUserDecryptParametersV2 = {
  readonly version: 2;
  readonly payload: {
    readonly handleContractPairs: ReadonlyArray<{
      readonly handle: Handle;
      readonly contractAddress: ChecksummedAddress;
      readonly ownerAddress: ChecksummedAddress;
    }>;
    readonly kmsDecryptEip712Signer: ChecksummedAddress;
    readonly kmsDecryptEip712Message: KmsUserDecryptEip712V2Message;
    // Variable length: 65-byte EOA, ERC-1271 blob, or empty `0x`.
    readonly kmsDecryptEip712Signature: BytesHex;
  };
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerUserDecryptOptions | undefined;
};

export type FetchUserDecryptParameters = FetchUserDecryptParametersV1 | FetchUserDecryptParametersV2;

export type FetchUserDecryptReturnType = readonly KmsSigncryptedShare[];

export type FetchUserDecryptModuleFunction = {
  fetchUserDecrypt(
    relayerClient: RelayerClientWithRuntime,
    parameters: FetchUserDecryptParameters,
  ): Promise<FetchUserDecryptReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 5. fetchDelegatedUserDecrypt
////////////////////////////////////////////////////////////////////////////////

export type FetchDelegatedUserDecryptParameters = {
  readonly version: 1;
  readonly payload: {
    readonly handleContractPairs: ReadonlyArray<{
      readonly handle: Handle;
      readonly contractAddress: ChecksummedAddress;
    }>;
    readonly kmsDecryptEip712Signer: ChecksummedAddress;
    readonly kmsDecryptEip712Message: KmsDelegatedUserDecryptEip712V1Message;
    readonly kmsDecryptEip712Signature: Bytes65Hex;
  };
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
};

export type FetchDelegatedUserDecryptReturnType = readonly KmsSigncryptedShare[];

export type FetchDelegatedUserDecryptModuleFunction = {
  fetchDelegatedUserDecrypt(
    relayerClient: RelayerClientWithRuntime,
    parameters: FetchDelegatedUserDecryptParameters,
  ): Promise<FetchDelegatedUserDecryptReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 6. fetchFeatures
////////////////////////////////////////////////////////////////////////////////

export type FetchFeaturesParameters = {
  readonly options?:
    | {
        readonly auth?: Auth | undefined;
      }
    | undefined;
};

export type FetchFeaturesReturnType = { readonly relayerSupportsV3UserDecryptRoute: boolean };

export type FetchFeaturesModuleFunction = {
  fetchFeatures(
    relayerClient: RelayerClientWithRuntime,
    parameters: FetchFeaturesParameters,
  ): Promise<FetchFeaturesReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 7. fetchPadShares (RFC-030 pad decryption)
////////////////////////////////////////////////////////////////////////////////

/**
 * One pad of an RFC-030 pad decryption request (`PadWitness`): the per-pad
 * witness fields, the pad's label and the number of pad bits to release.
 */
export type PadWitness = {
  readonly submitter: ChecksummedAddress;
  readonly consumer: ChecksummedAddress;
  /** Unix seconds after which the pad can no longer be decrypted. */
  readonly expirationTimestamp: Uint64BigInt;
  /** At most 16 entries. */
  readonly decryptors: readonly ChecksummedAddress[];
  /** 16 bytes. */
  readonly nonce: BytesHex;
  /** `hash(witness)`, see RFC-030. */
  readonly label: Bytes32Hex;
  /** Bits of the pad stream to release, in `[1, 2048]`. Not a witness field. */
  readonly padLength: Uint16Number;
};

/** One pad share, keyed by label (`PadShare`). */
export type PadShare = {
  readonly label: Bytes32Hex;
  /** 16 bytes: the MPC session that produced this share. */
  readonly sessionId: BytesHex;
  /** This node's share of the label's encrypted pad: a sequence of ring elements. */
  readonly share: BytesHex;
};

export type FetchPadSharesParameters = {
  readonly payload: {
    /** RFC-016 signed struct (`PadDecryptRequest.auth`). */
    readonly kmsDecryptEip712Message: KmsUserDecryptEip712V2Message;
    /** Variable length: 65-byte EOA, ERC-1271 blob, or empty `0x`. */
    readonly kmsDecryptEip712Signature: BytesHex;
    /** Shared witness fields, fixed for the whole request. */
    readonly witnessVersion: Uint8Number;
    readonly prf: Uint16Number;
    readonly chainId: Uint64BigInt;
    /** One entry per pad. */
    readonly witnesses: readonly PadWitness[];
  };
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerCommonOptions | undefined;
};

/**
 * One KMS node's answer to a pad decryption request (`PadDecryptResponse` in
 * RFC-030). `requestId` and `partyId` are unauthenticated routing hints; the
 * authenticated content is inside `encrypted`, i.e.
 * `HybridEncrypt_user(PadDecryptResponsePayload ‖ signature)`, which only the
 * holder of the request's transport private key can open.
 */
export type PadDecryptResponse = {
  readonly requestId: Bytes32Hex;
  readonly partyId: Uint32Number;
  readonly encrypted: BytesHex;
};

/** One response per KMS node that answered; order carries no meaning. */
export type FetchPadSharesReturnType = readonly PadDecryptResponse[];

export type FetchPadSharesModuleFunction = {
  fetchPadShares(
    relayerClient: RelayerClientWithRuntime,
    parameters: FetchPadSharesParameters,
  ): Promise<FetchPadSharesReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// RelayerModule
////////////////////////////////////////////////////////////////////////////////

export type RelayerModule = Prettify<
  FetchFheEncryptionKeySourceModuleFunction &
    FetchFheEncryptionKeyBytesModuleFunction &
    FetchCoprocessorSignaturesModuleFunction &
    FetchUserDecryptModuleFunction &
    FetchPublicDecryptModuleFunction &
    FetchDelegatedUserDecryptModuleFunction &
    FetchFeaturesModuleFunction &
    FetchPadSharesModuleFunction & {
      /**
       * `true` when the module serves placeholder key / CRS bytes (cleartext
       * mode, no FHE): they are not checked against KMSGeneration. Every other
       * module's key material is checked before use.
       */
      readonly servesPlaceholderFheEncryptionKey?: boolean | undefined;
    }
>;

// Relayer is a base module. It does not take any runtime argument
export type RelayerModuleFactory = () => {
  readonly relayer: RelayerModule;
};

export type WithRelayerModule = {
  readonly relayer: RelayerModule;
};

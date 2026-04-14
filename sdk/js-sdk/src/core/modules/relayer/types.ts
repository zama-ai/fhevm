import type {
  RelayerDelegatedUserDecryptOptions,
  RelayerInputProofOptions,
  RelayerKeyUrlOptions,
  RelayerPublicDecryptOptions,
  RelayerUserDecryptOptions,
} from '../../types/relayer.js';
import type { FheEncryptionKeyBytes, FheEncryptionKeySource } from '../../types/fheEncryptionKey.js';
import type { KmsSigncryptedShare } from '../../types/kms-p.js';
import type { KmsDelegatedUserDecryptEip712Message, KmsUserDecryptEip712Message } from '../../types/kms.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress } from '../../types/primitives.js';
import type { Prettify } from '../../types/utils.js';
import type { ZkProof } from '../../types/zkProof.js';
import type { Handle, InputHandle } from '../../types/encryptedTypes.js';

////////////////////////////////////////////////////////////////////////////////
//
// RelayerModule
//
////////////////////////////////////////////////////////////////////////////////

export type RelayerClient = {
  readonly relayerUrl: string;
  readonly chainId: number;
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
    readonly extraData: BytesHex;
  };
  readonly options?: RelayerInputProofOptions | undefined;
};

export type FetchCoprocessorSignaturesReturnType = {
  readonly handles: readonly InputHandle[];
  readonly coprocessorEip712Signatures: readonly Bytes65Hex[];
  readonly extraData: BytesHex;
};

export type FetchCoprocessorSignaturesModuleFunction = {
  fetchCoprocessorSignatures(
    relayerClient: RelayerClient,
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
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type FetchPublicDecryptReturnType = {
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly kmsPublicDecryptEIP712Signatures: Bytes65Hex[];
  readonly extraData: BytesHex;
};

export type FetchPublicDecryptModuleFunction = {
  fetchPublicDecrypt(
    relayerClient: RelayerClient,
    parameters: FetchPublicDecryptParameters,
  ): Promise<FetchPublicDecryptReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 4. fetchUserDecrypt
////////////////////////////////////////////////////////////////////////////////

export type FetchUserDecryptParameters = {
  readonly payload: {
    readonly handleContractPairs: ReadonlyArray<{
      readonly handle: Handle;
      readonly contractAddress: ChecksummedAddress;
    }>;
    readonly kmsDecryptEip712Signer: ChecksummedAddress;
    readonly kmsDecryptEip712Message: KmsUserDecryptEip712Message;
    readonly kmsDecryptEip712Signature: Bytes65Hex;
  };
  readonly options?: RelayerUserDecryptOptions | undefined;
};

export type FetchUserDecryptReturnType = readonly KmsSigncryptedShare[];

export type FetchUserDecryptModuleFunction = {
  fetchUserDecrypt(
    relayerClient: RelayerClient,
    parameters: FetchUserDecryptParameters,
  ): Promise<FetchUserDecryptReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 5. fetchDelegatedUserDecrypt
////////////////////////////////////////////////////////////////////////////////

export type FetchDelegatedUserDecryptParameters = {
  readonly payload: {
    readonly handleContractPairs: ReadonlyArray<{
      readonly handle: Handle;
      readonly contractAddress: ChecksummedAddress;
    }>;
    readonly kmsDecryptEip712Signer: ChecksummedAddress;
    readonly kmsDecryptEip712Message: KmsDelegatedUserDecryptEip712Message;
    readonly kmsDecryptEip712Signature: Bytes65Hex;
  };
  readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
};

export type FetchDelegatedUserDecryptReturnType = readonly KmsSigncryptedShare[];

export type FetchDelegatedUserDecryptModuleFunction = {
  fetchDelegatedUserDecrypt(
    relayerClient: RelayerClient,
    parameters: FetchDelegatedUserDecryptParameters,
  ): Promise<FetchDelegatedUserDecryptReturnType>;
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
    FetchDelegatedUserDecryptModuleFunction
>;

// Relayer is a base module. It does not take any runtime argument
export type RelayerModuleFactory = () => {
  readonly relayer: RelayerModule;
};

export type WithRelayerModule = {
  readonly relayer: RelayerModule;
};

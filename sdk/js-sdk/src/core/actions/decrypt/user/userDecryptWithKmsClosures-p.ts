import type {
  DecryptAndReconstructUserModuleFunction,
  GetTkmsPublicKeyHexUserModuleFunction,
} from "../../../modules/decrypt/types.js";
import type { RelayerFetchOptions } from "../../../modules/relayer/types.js";
import type { DecryptedFhevmHandle } from "../../../types/decryptedFhevmHandle.js";
import type { Fhevm } from "../../../types/coreFhevmClient.js";
import type { FhevmHandle } from "../../../types/fhevmHandle.js";
import type {
  KmsSigncryptedShares,
  KmsUserDecryptEIP712Message,
} from "../../../types/kms.js";
import type {
  Bytes65Hex,
  BytesHex,
  ChecksummedAddress,
} from "../../../types/primitives.js";
import type { Prettify } from "../../../types/utils.js";
import { fetchKmsSignedcryptedShares } from "./fetchKmsSignedcryptedShares.js";
import type { WithRelayer } from "../../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../../types/fhevmChain.js";

/*
    See: in KMS (eip712Domain)
    json.response[i].signature is an eip712 sig potentially on this message:

    struct UserDecryptResponseVerification {
        bytes publicKey;
        bytes32[] ctHandles;
        bytes userDecryptedShare;
        bytes extraData;
    }
}    
*/

////////////////////////////////////////////////////////////////////////////////

type UserDecryptWithKmsClosuresParameters = Prettify<
  {
    readonly handleContractPairs: ReadonlyArray<{
      readonly handle: FhevmHandle;
      readonly contractAddress: ChecksummedAddress;
    }>;
    readonly userDecryptEIP712Signer: ChecksummedAddress;
    readonly userDecryptEIP712Message: KmsUserDecryptEIP712Message;
    readonly userDecryptEIP712Signature: Bytes65Hex;
    readonly options?: RelayerFetchOptions;
  } & GetTkmsPublicKeyHexUserModuleFunction &
    DecryptAndReconstructUserModuleFunction
>;

export type UserDecryptWithKmsClosuresReturnType =
  readonly DecryptedFhevmHandle[];

////////////////////////////////////////////////////////////////////////////////
// userDecrypt
////////////////////////////////////////////////////////////////////////////////

export async function userDecryptWithKmsClosures(
  fhevm: Fhevm<FhevmChain, WithRelayer>,
  parameters: UserDecryptWithKmsClosuresParameters,
): Promise<UserDecryptWithKmsClosuresReturnType> {
  const { getTkmsPublicKeyHex, decryptAndReconstruct, ...rest } = parameters;

  const tkmsPublicKeyHex: BytesHex = await getTkmsPublicKeyHex();
  if (tkmsPublicKeyHex !== parameters.userDecryptEIP712Message.publicKey) {
    throw new Error("");
  }

  const kmsSigncryptedShares: KmsSigncryptedShares =
    await fetchKmsSignedcryptedShares(fhevm, rest);

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  const orderedDecryptedHandles: readonly DecryptedFhevmHandle[] =
    await decryptAndReconstruct({
      shares: kmsSigncryptedShares,
    });

  return orderedDecryptedHandles;
}

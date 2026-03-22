import { assertFhevmHandlesBelongToSameChainId } from "../../../handle/FhevmHandle.js";
import { createKmsSigncryptedShares } from "../../../kms/KmsSigncryptedShares-p.js";
import {
  assertKmsDecryptionBitLimit,
  assertKmsEIP712DeadlineValidity,
} from "../../../kms/utils.js";
import type { RelayerFetchOptions } from "../../../modules/relayer/types.js";
import type { Fhevm } from "../../../types/coreFhevmClient.js";
import type { WithRelayer } from "../../../types/coreFhevmRuntime.js";
import type { FhevmHandle } from "../../../types/fhevmHandle.js";
import type {
  KmsSigncryptedShare,
  KmsSigncryptedSharesMetadata,
} from "../../../types/kms-p.js";
import type {
  KmsSigncryptedShares,
  KmsUserDecryptEIP712Message,
} from "../../../types/kms.js";
import type { KmsSignersContext } from "../../../types/kmsSignersContext.js";
import type {
  Bytes65Hex,
  ChecksummedAddress,
  Uint64BigInt,
  UintNumber,
} from "../../../types/primitives.js";
import type { Prettify } from "../../../types/utils.js";
import { readKmsSignersContext } from "../readKmsSignersContext.js";
import { checkUserAllowedForDecryption } from "./checkUserAllowedForDecryption.js";
import { createKmsEIP712Domain } from "../../chain/createKmsEIP712Domain.js";
import { verifyKmsUserDecryptEIP712 } from "../../chain/verifyKmsUserDecryptEIP712.js";
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

type FetchKmsSignedcryptedSharesParameters = Prettify<{
  readonly handleContractPairs: ReadonlyArray<{
    handle: FhevmHandle;
    contractAddress: ChecksummedAddress;
  }>;
  readonly userDecryptEIP712Signer: ChecksummedAddress;
  readonly userDecryptEIP712Message: KmsUserDecryptEIP712Message;
  readonly userDecryptEIP712Signature: Bytes65Hex;
  readonly options?: RelayerFetchOptions;
}>;

export type FetchKmsSignedcryptedSharesReturnType = KmsSigncryptedShares;

////////////////////////////////////////////////////////////////////////////////
// fetchKmsSignedcryptedShares
////////////////////////////////////////////////////////////////////////////////

const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;
const MAX_USER_DECRYPT_DURATION_DAYS = 365 as UintNumber;

export async function fetchKmsSignedcryptedShares(
  fhevm: Fhevm<FhevmChain, WithRelayer>,
  parameters: FetchKmsSignedcryptedSharesParameters,
): Promise<FetchKmsSignedcryptedSharesReturnType> {
  const {
    handleContractPairs,
    userDecryptEIP712Signature,
    userDecryptEIP712Message,
    userDecryptEIP712Signer: userAddress,
  } = parameters;
  // 1. Check: At least one handle/contract pair is required
  if (handleContractPairs.length === 0) {
    throw Error(
      `handleContractPairs must not be empty, at least one handle/contract pair is required`,
    );
  }

  // 2. Check: At least one contract
  const contractAddressesLength =
    userDecryptEIP712Message.contractAddresses.length;
  if (contractAddressesLength === 0) {
    throw Error("contractAddresses is empty");
  }

  // 3. Check: No more that 10 contract addresses
  if (contractAddressesLength > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
    throw Error(
      `contractAddresses max length of ${MAX_USER_DECRYPT_CONTRACT_ADDRESSES} exceeded`,
    );
  }

  const fhevmHandles = handleContractPairs.map((pair) => pair.handle);
  Object.freeze(fhevmHandles);

  // 4. Check: All handles belong to the host chainId
  assertFhevmHandlesBelongToSameChainId(
    fhevmHandles,
    BigInt(fhevm.chain.id) as Uint64BigInt,
  );

  // 5. Check: 2048 bits limit
  assertKmsDecryptionBitLimit(fhevmHandles);

  // 6. Check: Expiration date
  assertKmsEIP712DeadlineValidity(
    userDecryptEIP712Message,
    MAX_USER_DECRYPT_DURATION_DAYS,
  );

  // 7. Check: ACL permissions
  await checkUserAllowedForDecryption(fhevm, {
    userAddress: parameters.userDecryptEIP712Signer,
    handleContractPairs: parameters.handleContractPairs,
  });

  const kmsUserDecryptEIP712Message: KmsUserDecryptEIP712Message =
    userDecryptEIP712Message;

  // 9. Verify the EIP712 signature
  await verifyKmsUserDecryptEIP712(fhevm, {
    signer: parameters.userDecryptEIP712Signer,
    message: kmsUserDecryptEIP712Message,
    signature: userDecryptEIP712Signature,
  });

  // 10. Fetch `KmsSignersContext` on-chain (cached)
  const kmsSignersContext: KmsSignersContext =
    await readKmsSignersContext(fhevm);

  // 11. Fetch `KmsSigncryptedShares` from the relayer
  const shares: readonly KmsSigncryptedShare[] =
    await fhevm.runtime.relayer.fetchUserDecrypt(
      { relayerUrl: fhevm.chain.fhevm.relayerUrl },
      {
        payload: {
          handleContractPairs,
          kmsUserDecryptEIP712Signer: userAddress,
          kmsUserDecryptEIP712Message,
          kmsUserDecryptEIP712Signature: userDecryptEIP712Signature,
        },
        options: parameters.options,
      },
    );

  // 12. Build the sealed `KmsSigncryptedShares` object
  const sharesMetadata: KmsSigncryptedSharesMetadata = {
    kmsSignersContext,
    eip712Domain: createKmsEIP712Domain(fhevm),
    eip712Signature: parameters.userDecryptEIP712Signature,
    eip712SignerAddress: userAddress,
    fhevmHandles,
  };

  return createKmsSigncryptedShares(sharesMetadata, shares);
}

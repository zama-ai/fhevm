import type { Fhevm } from "../../../types/coreFhevmClient.js";
import type { BytesHex, ChecksummedAddress } from "../../../types/primitives.js";
import { KmsContextCache } from "../../../kms/KmsContextCache-p.js";
import { buildRequestExtraData } from "../../../kms/extraData.js";
import { getTrustedClient } from "../../../runtime/CoreFhevm-p.js";

////////////////////////////////////////////////////////////////////////////////

export type GetExtraDataParameters = Record<string, never>;
export type GetExtraDataReturnType = BytesHex;

/**
 * Fetches the current KMS context ID and constructs the properly formatted
 * extraData value for user decryption requests.
 *
 * Starting with relayer v0.5.0-alpha.1, the extraData field carries a
 * versioned context identifier that ties each decryption request to a specific
 * KMS signer set. This enables the KMS to rotate its signer configuration
 * without breaking in-flight requests.
 *
 * @param fhevm - The FHEVM client instance
 * @param _parameters - Reserved for future use (empty object)
 * @returns A BytesHex value encoding the current KMS context (33 bytes)
 */
export async function getExtraData(
  fhevm: Fhevm,
  _parameters: GetExtraDataParameters,
): Promise<GetExtraDataReturnType> {
  if (!fhevm.chain) {
    throw new Error("Chain is required to fetch extraData");
  }

  const trustedClient = getTrustedClient(fhevm);

  // Create KmsContextCache instance
  const kmsContextCache = KmsContextCache.create({
    runtime: fhevm.runtime,
    kmsContractAddress: fhevm.chain.fhevm.contracts.kmsVerifier
      .address as ChecksummedAddress,
    hostPublicClient: trustedClient,
  });

  // Try to fetch current context ID
  // If the contract doesn't support context methods (pre-v0.5.0), return legacy extraData
  try {
    const contextId = await kmsContextCache.getCurrentContextId();
    // Build and return v1 extraData
    return buildRequestExtraData(contextId);
  } catch (error) {
    // Contract doesn't support getCurrentKmsContextId() - return legacy extraData (0x00)
    // This is expected for KMS contracts that haven't been upgraded yet
    return "0x00" as BytesHex;
  }
}

////////////////////////////////////////////////////////////////////////////////

import type { ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsExtraData } from '../types/kms-p.js';
import { assertIsChecksummedAddressArray } from '../base/address.js';
import { isUint8 } from '../base/uint.js';
import { getContextSignersAndThresholdFromExtraDataAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly kmsVerifierAddress: ChecksummedAddress;
  readonly extraData: KmsExtraData;
};

type ReturnType = {
  readonly threshold: Uint8Number;
  readonly signers: readonly ChecksummedAddress[];
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the KMS signers and threshold for a given extraData context.
 *
 * Not cached: the KMS context cache (see readKmsSignersContext-p.ts) holds the
 * result. The read also acts as an on-chain context validity check: it reverts
 * (InvalidKmsContext) for an unknown context.
 *
 * @param parameters.address - The checksummed address of the KMSVerifier contract.
 * @param parameters.extraData - The current context's extraData (v2: context and epoch ids).
 */
export async function getKmsContextSignersAndThresholdFromExtraData(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);

  const extraDataBytesHex = parameters.extraData.bytesHex;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: parameters.kmsVerifierAddress,
    abi: getContextSignersAndThresholdFromExtraDataAbi,
    args: [extraDataBytesHex],
    functionName: getContextSignersAndThresholdFromExtraDataAbi[0].name,
  });

  if (!Array.isArray(res) || res.length < 2) {
    throw new Error(`Invalid getContextSignersAndThresholdFromExtraData result.`);
  }

  const unknownSigners = res[0] as unknown;
  const unknownThreshold = res[1] as unknown;

  try {
    assertIsChecksummedAddressArray(unknownSigners, {});
  } catch (e) {
    throw new Error(`Invalid kms signers addresses.`, {
      cause: e,
    });
  }

  if (!isUint8(unknownThreshold)) {
    throw new Error('Invalid threshold.');
  }

  return Object.freeze({
    threshold: Number(unknownThreshold) as Uint8Number,
    signers: unknownSigners,
  });
}

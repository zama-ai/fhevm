import type { ChecksummedAddress, Uint256BigInt } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { getCurrentKmsContextAndEpochAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { assertIsUint256 } from '../base/uint.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly protocolConfigAddress: ChecksummedAddress;
};

type ReturnType = { readonly contextId: Uint256BigInt; readonly epochId: Uint256BigInt };

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the current KMS context ID and epoch ID from a ProtocolConfig contract.
 *
 * Not cached: the KMS context cache (see readKmsSignersContext-p.ts) holds
 * the result.
 *
 * @param parameters.address - The checksummed address of the ProtocolConfig contract.
 */
export async function getCurrentKmsContextAndEpoch(context: Context, parameters: Parameters): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: parameters.protocolConfigAddress,
    abi: getCurrentKmsContextAndEpochAbi,
    args: [],
    functionName: getCurrentKmsContextAndEpochAbi[0].name,
  });

  if (!Array.isArray(res) || res.length < 2) {
    throw new Error(`Invalid getCurrentKmsContextAndEpoch result.`);
  }

  const unknownContextId = res[0] as unknown;
  const unknownEpochId = res[1] as unknown;

  try {
    assertIsUint256(unknownContextId, {});
  } catch (e) {
    throw new Error(`Invalid KMS Context Id.`, { cause: e });
  }

  try {
    assertIsUint256(unknownEpochId, {});
  } catch (e) {
    throw new Error(`Invalid KMS Epoch Id.`, { cause: e });
  }

  return Object.freeze({
    contextId: BigInt(unknownContextId) as Uint256BigInt,
    epochId: BigInt(unknownEpochId) as Uint256BigInt,
  });
}

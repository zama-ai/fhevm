import type { ChecksummedAddress, Uint256BigInt } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { getVersion, isVersionStrictlyBefore } from './HostContractVersion-p.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { getKmsSignersAbi, getSignersForKmsContextAbi } from './abi-fragments/fragments.js';
import { assertIsChecksummedAddressArray } from '../base/address.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly address: ChecksummedAddress;
  readonly kmsContextId: Uint256BigInt;
};

type ReturnType = ChecksummedAddress[];

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the ordered signer list for a given KMS context ID from the KMSVerifier contract.
 *
 * This function is **not cached** — every call issues a fresh RPC request.
 * Use sparingly or wrap with a caching layer if repeated calls are expected.
 *
 * Returns an empty array for KMSVerifier versions before v0.2.0 (where
 * per-context signers were not yet supported).
 *
 * @param parameters.address - The checksummed address of the KMSVerifier contract.
 * @param parameters.kmsContextId - The context ID to query signers for.
 */
export async function getSignersForKmsContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  const version = await getVersion(context, parameters);
  // getCurrentKmsContextId has been introduced in KMSVerifier.sol v0.2.0
  if (isVersionStrictlyBefore(version, { major: 0, minor: 2 })) {
    if (parameters.kmsContextId === 0n) {
      return getKmsSigners(context, parameters);
    }
    throw new Error('Invalid context id');
  }

  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getSignersForKmsContextAbi,
    args: [parameters.kmsContextId],
    functionName: getSignersForKmsContextAbi[0].name,
  });

  try {
    assertIsChecksummedAddressArray(res, {});
  } catch (e) {
    throw new Error(`Invalid signers for KMS Context Id ${parameters.kmsContextId}.`, {
      cause: e,
    });
  }

  return res;
}

export async function getKmsSigners(
  context: Context,
  parameters: Omit<Parameters, 'kmsContextId'>,
): Promise<ReturnType> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getKmsSignersAbi,
    args: [],
    functionName: getKmsSignersAbi[0].name,
  });

  try {
    assertIsChecksummedAddressArray(res, {});
  } catch (e) {
    throw new Error(`Invalid KMS signers.`, {
      cause: e,
    });
  }

  return res;
}

import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { isChecksummedAddress } from '../base/address.js';
import { isUint64BigInt } from '../base/uint.js';
import { eip712DomainAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

export type Eip712DomainReturnType = {
  name: string;
  version: string;
  chainId: Uint64BigInt;
  verifyingContract: ChecksummedAddress;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 * Reads the EIP-712 domain from the given contract.
 * The result is not cached; each call performs a fresh on-chain read.
 */
export async function eip712Domain(context: Context, parameters: Parameters): Promise<Eip712DomainReturnType> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: eip712DomainAbi,
    args: [],
    functionName: eip712DomainAbi[0].name,
  });

  if (!Array.isArray(res) || res.length < 5) {
    throw new Error(`Invalid eip712Domain result.`);
  }

  const unknownName = res[1] as unknown;
  const unknownVersion = res[2] as unknown;
  const unknownChainId = res[3] as unknown;
  const unknownVerifyingContract = res[4] as unknown;

  if (typeof unknownName !== 'string') {
    throw new Error('Invalid EIP-712 name version.');
  }

  if (typeof unknownVersion !== 'string') {
    throw new Error('Invalid EIP-712 domain version.');
  }

  if (!isUint64BigInt(unknownChainId)) {
    throw new Error('Invalid EIP-712 domain chainId.');
  }

  if (!isChecksummedAddress(unknownVerifyingContract)) {
    throw new Error('Invalid EIP-712 domain chainId.');
  }

  return {
    name: unknownName,
    version: unknownVersion,
    chainId: unknownChainId,
    verifyingContract: unknownVerifyingContract,
  };
}

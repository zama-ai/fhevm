import { isChecksummedAddress } from '../../base/address.js';
import { isUint64BigInt } from '../../base/uint.js';
import { eip712DomainAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type {
  ChecksummedAddress,
  Uint64BigInt,
} from '../../types/primitives.js';

export type Eip712DomainParameters = {
  readonly address: ChecksummedAddress;
};

export type Eip712DomainReturnType = {
  name: string;
  version: string;
  chainId: Uint64BigInt;
  verifyingContract: ChecksummedAddress;
};

export async function eip712Domain(
  fhevm: Fhevm,
  parameters: Eip712DomainParameters,
): Promise<Eip712DomainReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
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

import type { KmsSignersContext } from '../../types/kmsSignersContext.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import { readKmsSignersContext as readKmsSignersContext_ } from '../../host-contracts/readKmsSignersContext-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ReadKmsSignersContextReturnType = KmsSignersContext;

////////////////////////////////////////////////////////////////////////////////

export async function readKmsSignersContext(fhevm: Fhevm<FhevmChain>): Promise<ReadKmsSignersContextReturnType> {
  return readKmsSignersContext_(fhevm, {
    address: fhevm.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
  });
}

////////////////////////////////////////////////////////////////////////////////

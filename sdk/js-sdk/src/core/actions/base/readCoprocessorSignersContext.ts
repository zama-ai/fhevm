import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { CoprocessorSignersContext } from '../../types/coprocessorSignersContext.js';
import { readCoprocessorSignersContext as readCoprocessorSignersContext_ } from '../../host-contracts/readCoprocessorSignersContext-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ReadCoprocessorSignersContextReturnType = CoprocessorSignersContext;

////////////////////////////////////////////////////////////////////////////////

export async function readCoprocessorSignersContext(
  fhevm: Fhevm<FhevmChain>,
): Promise<ReadCoprocessorSignersContextReturnType> {
  return readCoprocessorSignersContext_(fhevm, {
    address: fhevm.chain.fhevm.contracts.inputVerifier
      .address as ChecksummedAddress,
  });
}

////////////////////////////////////////////////////////////////////////////////

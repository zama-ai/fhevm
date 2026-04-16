import type { CoprocessorEip712Domain, CoprocessorEip712Message } from '../../types/coprocessor.js';
import type { CoprocessorSignersContext } from '../../types/coprocessorSignersContext.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress, Uint64BigInt } from '../../types/primitives.js';
import type { InputHandle } from '../../types/encryptedTypes-p.js';
import { recoverSigners } from '../../utils-p/runtime/recoverSigners.js';
import { coprocessorEip712PrimaryType, coprocessorEip712Types } from '../../coprocessor/coprocessorEip712Types.js';
import { createCoprocessorEip712Domain } from '../../coprocessor/createCoprocessorEip712Domain.js';
import { assertCoprocessorSignerThreshold } from '../../host-contracts/CoprocessorSignersContext-p.js';
import { readCoprocessorSignersContext } from './readCoprocessorSignersContext.js';

////////////////////////////////////////////////////////////////////////////////

export type VerifyHandlesCoprocessorSignaturesParameters = {
  readonly coprocessorSignatures: readonly Bytes65Hex[];
  readonly inputHandles: readonly InputHandle[];
  readonly userAddress: ChecksummedAddress;
  readonly contractAddress: ChecksummedAddress;
  readonly chainId: Uint64BigInt;
  readonly extraData: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////
// verifyHandlesCoprocessorSignatures
////////////////////////////////////////////////////////////////////////////////

export async function verifyHandlesCoprocessorSignatures(
  fhevm: Fhevm<FhevmChain>,
  parameters: VerifyHandlesCoprocessorSignaturesParameters,
): Promise<void> {
  // Use hex strings (not Uint8Array) for EIP-712 message fields.
  // viem's hashTypedData expects hex strings for bytes32 fields.
  const message: CoprocessorEip712Message = {
    ctHandles: parameters.inputHandles.map((h) => h.bytes32Hex),
    userAddress: parameters.userAddress,
    contractAddress: parameters.contractAddress,
    contractChainId: parameters.chainId,
    extraData: parameters.extraData,
  };

  const domain: CoprocessorEip712Domain = createCoprocessorEip712Domain({
    gatewayChainId: fhevm.chain.fhevm.gateway.id,
    verifyingContractAddressInputVerification: fhevm.chain.fhevm.gateway.contracts.inputVerification.address,
  });

  // 1. Verify signatures
  const recoveredAddresses = await recoverSigners(fhevm, {
    domain,
    primaryType: coprocessorEip712PrimaryType,
    types: coprocessorEip712Types,
    signatures: parameters.coprocessorSignatures,
    message,
  });

  const coprocessorSignersContext: CoprocessorSignersContext = await readCoprocessorSignersContext(fhevm);

  // 2. Verify signature theshold is reached
  assertCoprocessorSignerThreshold(coprocessorSignersContext, recoveredAddresses);
}

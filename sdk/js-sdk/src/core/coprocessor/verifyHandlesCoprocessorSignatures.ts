import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { CoprocessorEip712Domain, CoprocessorEip712Message } from '../types/coprocessor.js';
import type { CoprocessorSignersContext } from '../types/coprocessorSignersContext.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import { recoverSigners } from '../utils-p/runtime/recoverSigners.js';
import { coprocessorEip712PrimaryType, coprocessorEip712Types } from './coprocessorEip712Types.js';
import { createCoprocessorEip712Domain } from './createCoprocessorEip712Domain.js';
import { assertCoprocessorSignerThreshold } from '../host-contracts/CoprocessorSignersContext-p.js';
import { readCoprocessorSignersContext } from '../host-contracts/readCoprocessorSignersContext-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly coprocessorSignatures: readonly Bytes65Hex[];
  readonly handles: readonly InputHandle[];
  readonly userAddress: ChecksummedAddress;
  readonly contractAddress: ChecksummedAddress;
  readonly chainId: Uint64BigInt;
  readonly extraData: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////
// verifyHandlesCoprocessorSignatures
////////////////////////////////////////////////////////////////////////////////

export async function verifyHandlesCoprocessorSignatures(context: Context, parameters: Parameters): Promise<void> {
  // Use hex strings (not Uint8Array) for EIP-712 message fields.
  // viem's hashTypedData expects hex strings for bytes32 fields.
  const message: CoprocessorEip712Message = {
    ctHandles: parameters.handles.map((h) => h.bytes32Hex),
    userAddress: parameters.userAddress,
    contractAddress: parameters.contractAddress,
    contractChainId: parameters.chainId,
    extraData: parameters.extraData,
  };

  const domain: CoprocessorEip712Domain = createCoprocessorEip712Domain({
    gatewayChainId: context.chain.fhevm.gateway.id,
    verifyingContractAddressInputVerification: context.chain.fhevm.gateway.contracts.inputVerification.address,
  });

  // 1. Verify signatures
  const recoveredAddresses = await recoverSigners(context, {
    domain,
    primaryType: coprocessorEip712PrimaryType,
    types: coprocessorEip712Types,
    signatures: parameters.coprocessorSignatures,
    message,
  });

  const coprocessorSignersContext: CoprocessorSignersContext = await readCoprocessorSignersContext(context, {
    address: context.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress,
  });

  // 2. Verify signature threshold is reached
  assertCoprocessorSignerThreshold(coprocessorSignersContext, recoveredAddresses);
}

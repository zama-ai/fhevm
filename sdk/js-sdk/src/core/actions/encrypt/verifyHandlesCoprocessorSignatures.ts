import {
  coprocessorEIP712PrimaryType,
  coprocessorEIP712Types,
} from "../../coprocessor/coprocessorEIP712Types.js";
import { createCoprocessorEIP712Domain } from "../../coprocessor/createCoprocessorEIP712Domain.js";
import { assertCoprocessorSignerThreshold } from "../../host-contracts/CoprocessorSignersContext-p.js";
import type {
  CoprocessorEIP712Domain,
  CoprocessorEIP712Message,
} from "../../types/coprocessor.js";
import type { CoprocessorSignersContext } from "../../types/coprocessorSignersContext.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { FhevmHandle } from "../../types/fhevmHandle.js";
import type {
  Bytes32,
  Bytes65Hex,
  BytesHex,
  ChecksummedAddress,
  Uint64BigInt,
} from "../../types/primitives.js";
import { recoverSigners } from "../runtime/recoverSigners.js";
import { readCoprocessorSignersContext } from "./readCoprocessorSignersContext.js";

////////////////////////////////////////////////////////////////////////////////

export type VerifyHandlesCoprocessorSignaturesParameters = {
  readonly coprocessorSignatures: readonly Bytes65Hex[];
  readonly handles: readonly FhevmHandle[];
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
  const handlesBytes32: Bytes32[] = parameters.handles.map((h) => h.bytes32);

  const message: CoprocessorEIP712Message = {
    ctHandles: handlesBytes32,
    userAddress: parameters.userAddress,
    contractAddress: parameters.contractAddress,
    contractChainId: parameters.chainId,
    extraData: parameters.extraData,
  };

  const domain: CoprocessorEIP712Domain = createCoprocessorEIP712Domain({
    gatewayChainId: fhevm.chain.fhevm.gateway.id,
    verifyingContractAddressInputVerification:
      fhevm.chain.fhevm.gateway.contracts.inputVerification.address,
  });

  // 1. Verify signatures
  const recoveredAddresses = await recoverSigners(fhevm, {
    domain,
    primaryType: coprocessorEIP712PrimaryType,
    types: coprocessorEIP712Types,
    signatures: parameters.coprocessorSignatures,
    message,
  });

  const coprocessorSignersContext: CoprocessorSignersContext =
    await readCoprocessorSignersContext(fhevm);

  // 2. Verify signature theshold is reached
  assertCoprocessorSignerThreshold(
    coprocessorSignersContext,
    recoveredAddresses,
  );
}

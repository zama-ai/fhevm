// The coprocessor's part of an input, played by the cleartext client: the cleartext host reads each
// input's plaintext from the attestation's `extraData`, and verifies the attestation exactly as the
// production host does, so the client signs the same EIP-712 message with a registered key.
import { keccak_256 } from '@noble/hashes/sha3.js';
import type { BytesHex } from '../../core/types/primitives.js';
import type { SolanaZkProof } from '../../core/types/zkProof-p.js';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaSubmitInputProofParameters, SolanaSubmitInputProofResult } from '../actions/submitInputProof.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { bytesToHex, concatBytes, hexToBytes } from '../../core/base/bytes.js';
import { buildInputProofMetaData } from '../../core/coprocessor/buildInputProofMetaData-p.js';
import { createCoprocessorEip712Domain } from '../../core/coprocessor/createCoprocessorEip712Domain.js';
import { InputProofError } from '../../core/errors/InputProofError.js';
import { unpackWithProofPacked } from '../../core/modules/encrypt/mock.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { findHostConfigPda } from '../internal/generated/zamaHost/pdas/hostConfig.js';
import { fetchHostConfig } from '../internal/generated/zamaHost/accounts/hostConfig.js';
import { signAsCleartextParty } from './parties.js';
import { CIPHERTEXT_VERIFICATION_TYPE, INPUT_VALUE_LEN } from '../internal/hostConstants.js';
import { uint256ToBytes32 } from '../../core/base/uint.js';
import { eip712Digest, keccakUtf8 } from '../internal/eip712.js';

////////////////////////////////////////////////////////////////////////////////

/** The coprocessor `CiphertextVerification` digest the Solana host verifies (`zama_host::eip712`). */
export function ciphertextVerificationDigest(parameters: {
  readonly gatewayChainId: bigint;
  readonly inputVerificationContract: Uint8Array;
  readonly ctHandles: readonly Uint8Array[];
  readonly userAddress: Uint8Array;
  readonly contractAddress: Uint8Array;
  readonly contractChainId: bigint;
  readonly extraData: Uint8Array;
}): BytesHex {
  const domain = createCoprocessorEip712Domain({
    gatewayChainId: parameters.gatewayChainId,
    verifyingContractAddressInputVerification: bytesToHex(parameters.inputVerificationContract),
  });
  const struct = keccak_256(
    concatBytes(
      keccakUtf8(CIPHERTEXT_VERIFICATION_TYPE),
      keccak_256(concatBytes(...parameters.ctHandles)),
      parameters.userAddress,
      parameters.contractAddress,
      uint256ToBytes32(parameters.contractChainId),
      keccak_256(parameters.extraData),
    ),
  );
  return bytesToHex(eip712Digest(domain, struct));
}

/** The attestation `extraData` carrying the plaintexts of a cleartext input proof, in handle order. */
export function cleartextInputExtraData(inputProof: SolanaZkProof): Uint8Array {
  const handles = inputProof.getInputHandles();
  const words = unpackWithProofPacked({
    ciphertextWithZKProofBytes: inputProof.ciphertextWithZkProof,
    metaData: buildInputProofMetaData(inputProof),
    count: handles.length,
  });
  if (words === undefined) {
    throw new InputProofError({ message: 'Input proof was not built by the cleartext encrypt module' });
  }
  const values = handles.map((handle, index) => {
    const length = INPUT_VALUE_LEN[handle.fheTypeId];
    const packed = words[index];
    if (length === undefined || packed === undefined) {
      throw new InputProofError({ message: `The Solana host does not support FHE type ${handle.fheType}` });
    }
    return packed.subarray(-length);
  });
  // The host takes at most 16 handles per attestation, so this is at most 16 × 16 = 256 bytes, its
  // extraData limit.
  return concatBytes(...values);
}

/**
 * Attests a cleartext input proof as the host's registered coprocessors would, reading the signer
 * set, threshold and EIP-712 domain from the live `HostConfig`.
 */
export function cleartextSubmitInputProof(rpc: SolanaRpc) {
  return async (
    context: { readonly solanaChain: FhevmSolanaChain },
    parameters: SolanaSubmitInputProofParameters,
  ): Promise<SolanaSubmitInputProofResult> => {
    const { inputProof } = parameters;
    const handles = inputProof.getInputHandles();
    if (handles.length === 0) {
      throw new InputProofError({ message: 'Input proof must contain at least one external handle' });
    }
    if (inputProof.chainId !== context.solanaChain.id) {
      throw new InputProofError({
        message: `Input proof chain id ${inputProof.chainId} does not match Solana client chain id ${context.solanaChain.id}`,
      });
    }
    const extraData = cleartextInputExtraData(inputProof);

    const programAddress = solanaHostProgram(context.solanaChain);
    const [hostConfigAddress] = await findHostConfigPda({ programAddress });
    const { data: config } = await fetchHostConfig(rpc, hostConfigAddress, { commitment: 'confirmed' });
    const digest = ciphertextVerificationDigest({
      gatewayChainId: config.gatewayChainId,
      inputVerificationContract: new Uint8Array(config.inputVerificationContract),
      ctHandles: handles.map((handle) => hexToBytes(handle.bytes32Hex)),
      userAddress: hexToBytes(inputProof.userAddress),
      contractAddress: hexToBytes(inputProof.contractAddress),
      contractChainId: inputProof.chainId,
      extraData,
    });
    return {
      handles,
      signatures: signAsCleartextParty(
        'coprocessor',
        config.coprocessorSigners.slice(0, config.coprocessorSignerCount),
        config.coprocessorThreshold,
        digest,
      ),
      extraData: bytesToHex(extraData),
    };
  };
}

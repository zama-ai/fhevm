// The coprocessor's part of an input, played by the cleartext client: the cleartext host reads each
// input's plaintext from the attestation's `extraData`, and verifies the attestation exactly as the
// production host does, so the client signs the same EIP-712 message with a registered key.
import { fetchEncodedAccount } from '@solana/kit';
import { keccak_256 } from '@noble/hashes/sha3.js';
import type { FheTypeId } from '../../core/types/fheType.js';
import type { BytesHex } from '../../core/types/primitives.js';
import type { SolanaZkProof } from '../../core/types/zkProof-p.js';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaSubmitInputProofParameters, SolanaSubmitInputProofResult } from '../actions/submitInputProof.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { bytesToHex, concatBytes, hexToBytes } from '../../core/base/bytes.js';
import { InputProofError } from '../../core/errors/InputProofError.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import { findHostConfigPda } from '../internal/generated/zamaHost/pdas/hostConfig.js';
import { getHostConfigDecoder } from '../internal/generated/zamaHost/accounts/hostConfig.js';
import { signAsCleartextParty } from './parties.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * One value of a cleartext input proof, as the mock encrypt module packs it: an 8-byte nonce, the
 * 128-byte Solana input metadata, then the value as a 32-byte big-endian word.
 */
const PACKED_VALUE_LEN = 8 + 128 + 32;
/** Bytes each shipped FHE type takes in `extraData` (`zama_host::cleartext::value_len`). */
const VALUE_LEN: Partial<Record<FheTypeId, number>> = { 0: 1, 2: 1, 3: 2, 4: 4, 5: 8, 6: 16 };

const DOMAIN_TYPE = 'EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)';
// RFC-021: the host identities are bytes32, where the EVM message has `address`.
const CIPHERTEXT_VERIFICATION_TYPE =
  'CiphertextVerification(bytes32[] ctHandles,bytes32 userAddress,bytes32 contractAddress,uint256 contractChainId,bytes extraData)';

////////////////////////////////////////////////////////////////////////////////

const utf8 = (text: string): Uint8Array => new TextEncoder().encode(text);

function word(value: bigint): Uint8Array {
  return hexToBytes(`0x${value.toString(16).padStart(64, '0')}`);
}

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
  const verifyingContract = new Uint8Array(32);
  verifyingContract.set(parameters.inputVerificationContract, 12);
  const domain = keccak_256(
    concatBytes(
      keccak_256(utf8(DOMAIN_TYPE)),
      keccak_256(utf8('InputVerification')),
      keccak_256(utf8('1')),
      word(parameters.gatewayChainId),
      verifyingContract,
    ),
  );
  const struct = keccak_256(
    concatBytes(
      keccak_256(utf8(CIPHERTEXT_VERIFICATION_TYPE)),
      keccak_256(concatBytes(...parameters.ctHandles)),
      parameters.userAddress,
      parameters.contractAddress,
      word(parameters.contractChainId),
      keccak_256(parameters.extraData),
    ),
  );
  return bytesToHex(keccak_256(concatBytes(Uint8Array.of(0x19, 0x01), domain, struct)));
}

/** The attestation `extraData` carrying the plaintexts of a cleartext input proof, in handle order. */
export function cleartextInputExtraData(inputProof: SolanaZkProof): Uint8Array {
  const handles = inputProof.getInputHandles();
  const packed = inputProof.ciphertextWithZkProof;
  if (packed.length !== handles.length * PACKED_VALUE_LEN) {
    throw new InputProofError({ message: 'Input proof was not built by the cleartext encrypt module' });
  }
  const values = handles.map((handle, index) => {
    const length = VALUE_LEN[handle.fheTypeId];
    if (length === undefined) {
      throw new InputProofError({ message: `The Solana host does not support FHE type ${handle.fheType}` });
    }
    const end = (index + 1) * PACKED_VALUE_LEN;
    return packed.subarray(end - length, end);
  });
  // The proof builder's 2048-bit cap keeps this within the host's 256-byte extraData limit.
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
    const account = await fetchEncodedAccount(rpc, hostConfigAddress, { commitment: 'confirmed' });
    if (!account.exists || account.programAddress !== programAddress) {
      throw new Error(`No HostConfig of ${programAddress} at ${hostConfigAddress}`);
    }
    const config = getHostConfigDecoder().decode(account.data);
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

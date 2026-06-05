import { uint256ToBytes32 } from '../base/uint.js';
import { isAddress } from '../base/address.js';
import { hexToBytes20, hexToBytes32, isBytes32Hex } from '../base/bytes.js';
import { ZkProofError } from '../errors/ZkProofError.js';

/**
 * RFC-021 reserves the high bit of the u64 chain id as the host `chain_type`
 * marker: when set, the host chain is Solana rather than an EVM chain.
 */
export const SOLANA_CHAIN_TYPE_BIT = 1n << 63n;

/** True when the chain-type high bit marks this as a Solana host chain. */
export function isSolanaHostChainId(chainId: bigint | number): boolean {
  return (BigInt(chainId) & SOLANA_CHAIN_TYPE_BIT) !== 0n;
}

/**
 * Assembles the auxiliary data that the input ZK proof is bound to.
 *
 * The prover (this SDK) and the coprocessor's zkproof-worker verifier must agree
 * on this layout byte-for-byte, or proof verification fails. The host chain type
 * selects the layout:
 *
 * - EVM hosts: `contract(20) || user(20) || acl(20) || chainId(32)` = 92 bytes.
 * - Solana hosts (RFC-021): `contract(32) || user(32) || acl(32) || chainId(32)`
 *   = 128 bytes, where the three identities are bytes32 host addresses.
 *
 * The chain id is always the trailing 32-byte big-endian word and carries the
 * chain-type high bit verbatim. This mirrors `ZkData::assemble` in
 * `coprocessor/fhevm-engine/zkproof-worker/src/auxiliary.rs`.
 */
export function buildInputProofMetaData(params: {
  readonly chainId: bigint | number;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly aclContractAddress: string;
}): Uint8Array {
  const { chainId, contractAddress, userAddress, aclContractAddress } = params;
  const chainIdBytes32 = uint256ToBytes32(chainId);

  if (isSolanaHostChainId(chainId)) {
    assertBytes32Identity(contractAddress, 'contract address');
    assertBytes32Identity(userAddress, 'user address');
    assertBytes32Identity(aclContractAddress, 'ACL contract address');

    const metaData = new Uint8Array(128);
    metaData.set(hexToBytes32(contractAddress), 0);
    metaData.set(hexToBytes32(userAddress), 32);
    metaData.set(hexToBytes32(aclContractAddress), 64);
    metaData.set(chainIdBytes32, 96);
    return metaData;
  }

  assertAddress(contractAddress, 'contract address');
  assertAddress(userAddress, 'user address');
  assertAddress(aclContractAddress, 'ACL contract address');

  const metaData = new Uint8Array(92);
  metaData.set(hexToBytes20(contractAddress), 0);
  metaData.set(hexToBytes20(userAddress), 20);
  metaData.set(hexToBytes20(aclContractAddress), 40);
  metaData.set(chainIdBytes32, 60);
  return metaData;
}

function assertAddress(value: string, subject: string): void {
  if (!isAddress(value)) {
    throw new ZkProofError({ message: `Invalid ${subject}: ${value}` });
  }
}

function assertBytes32Identity(value: string, subject: string): void {
  if (!isBytes32Hex(value)) {
    throw new ZkProofError({
      message: `Invalid bytes32 ${subject}: ${value}`,
    });
  }
}

import { keccak_256 } from '@noble/hashes/sha3.js';
import { concatBytes, hexToBytes } from '../../core/base/bytes.js';
import { uint256ToBytes32 } from '../../core/base/uint.js';
import { EIP712_DOMAIN_TYPE } from './hostConstants.js';

////////////////////////////////////////////////////////////////////////////////
// The EIP-712 v4 digests the Solana host verifies (`zama_host::eip712`), for the certificates of
// both the KMS and the coprocessors.
////////////////////////////////////////////////////////////////////////////////

const utf8 = new TextEncoder();

export function keccakUtf8(text: string): Uint8Array {
  return keccak_256(utf8.encode(text));
}

/** `keccak256(0x19 ‖ 0x01 ‖ domainSeparator ‖ structHash)`. */
export function eip712Digest(
  domain: {
    readonly name: string;
    readonly version: string;
    readonly chainId: bigint;
    readonly verifyingContract: string;
  },
  structHash: Uint8Array,
): Uint8Array {
  const domainSeparator = keccak_256(
    concatBytes(
      keccakUtf8(EIP712_DOMAIN_TYPE),
      keccakUtf8(domain.name),
      keccakUtf8(domain.version),
      uint256ToBytes32(domain.chainId),
      concatBytes(new Uint8Array(12), hexToBytes(domain.verifyingContract)),
    ),
  );
  return keccak_256(concatBytes(Uint8Array.of(0x19, 0x01), domainSeparator, structHash));
}

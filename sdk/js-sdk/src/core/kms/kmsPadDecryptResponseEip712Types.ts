import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import { addressToChecksummedAddress, assertIsAddress } from '../base/address.js';
import { assertIsUint64 } from '../base/uint.js';

/**
 * The EIP-712 types each KMS node signs over its pad decryption response (RFC-030, tech-spec#564): one
 * `PadShareVerification` per share, covering the payload's `extraData`, and one `PadDecryptResponseVerification`
 * over the encrypted payload.
 *
 * The field names, types and order below follow the RFC's Solidity structs verbatim and must be kept in sync with
 * it as the spec settles.
 *
 * CRITICAL: Field order is authoritative — it determines the EIP-712 type hash.
 */
export const kmsPadDecryptResponseEip712Types = {
  EIP712Domain: [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' },
  ] as const,
  PadShareVerification: [
    { name: 'label', type: 'bytes32' },
    { name: 'rawShares', type: 'bytes' },
    { name: 'partyId', type: 'uint32' },
    { name: 'degree', type: 'uint32' },
    { name: 'sessionId', type: 'bytes16' },
    { name: 'extraData', type: 'bytes' },
  ] as const,
  PadDecryptResponseVerification: [
    { name: 'requestId', type: 'bytes32' },
    { name: 'encryptedPayload', type: 'bytes' },
  ] as const,
} as const;

export const kmsPadShareEip712PrimaryType = 'PadShareVerification' as const;
export const kmsPadDecryptResponseEip712PrimaryType = 'PadDecryptResponseVerification' as const;

Object.freeze(kmsPadDecryptResponseEip712Types);
Object.freeze(kmsPadDecryptResponseEip712Types.EIP712Domain);
Object.freeze(kmsPadDecryptResponseEip712Types.PadShareVerification);
Object.freeze(kmsPadDecryptResponseEip712Types.PadDecryptResponseVerification);
kmsPadDecryptResponseEip712Types.EIP712Domain.forEach(Object.freeze);
kmsPadDecryptResponseEip712Types.PadShareVerification.forEach(Object.freeze);
kmsPadDecryptResponseEip712Types.PadDecryptResponseVerification.forEach(Object.freeze);

////////////////////////////////////////////////////////////////////////////////

/** The EIP-712 domain both pad decryption response signatures are made under. */
export type KmsPadDecryptionEip712Domain = {
  readonly name: 'fhevm-pad-decryption';
  readonly version: '1';
  readonly chainId: Uint64BigInt;
  readonly verifyingContract: ChecksummedAddress;
};

export type CreateKmsPadDecryptionEip712DomainParameters = {
  /** Chain id of the chain ProtocolConfig is deployed on. */
  readonly protocolConfigChainId: number | bigint;
  readonly protocolConfigAddress: string;
};

/**
 * The fixed EIP-712 domain of pad decryption responses (RFC-030): `fhevm-pad-decryption`, version `1`, anchored on
 * ProtocolConfig's chain id and address, so that a signature is valid for one protocol deployment only.
 */
export function createKmsPadDecryptionEip712Domain(
  parameters: CreateKmsPadDecryptionEip712DomainParameters,
): KmsPadDecryptionEip712Domain {
  const { protocolConfigChainId, protocolConfigAddress } = parameters;
  assertIsUint64(protocolConfigChainId, { subject: 'protocolConfigChainId' });
  assertIsAddress(protocolConfigAddress, {});
  return Object.freeze({
    name: 'fhevm-pad-decryption',
    version: '1',
    chainId: BigInt(protocolConfigChainId) as Uint64BigInt,
    verifyingContract: addressToChecksummedAddress(protocolConfigAddress),
  } as const);
}

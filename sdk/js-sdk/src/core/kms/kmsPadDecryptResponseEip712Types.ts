/**
 * The EIP-712 type each KMS node signs over its pad decryption response payload (RFC-030).
 *
 * RFC-030 names this struct but does not pin its EIP-712 encoding yet; the
 * field names, types and order below follow the RFC's Solidity struct
 * verbatim and must be kept in sync with it as the spec settles.
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
    { name: 'contextId', type: 'bytes32' },
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

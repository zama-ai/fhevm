////////////////////////////////////////////////////////////////////////////////
// KmsPadDecryptResponseEip712Types (RFC-030)
////////////////////////////////////////////////////////////////////////////////

// The EIP-712 type each KMS node signs over its pad decryption response payload.
//
// RFC-030 names this struct but does not pin its EIP-712 encoding yet; the
// field names, types and order below follow the RFC's Solidity struct
// verbatim and must be kept in sync with it as the spec settles.
//
// CRITICAL: Field order is authoritative — it determines the EIP-712 type hash.
export const kmsPadDecryptResponseEip712Types = {
  EIP712Domain: [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' },
  ] as const,
  PadDecryptResponsePayload: [
    { name: 'shareFormat', type: 'uint8' },
    { name: 'extraData', type: 'bytes' },
    { name: 'sessionId', type: 'bytes32' },
    { name: 'degree', type: 'uint32' },
    { name: 'partyId', type: 'uint32' },
    { name: 'pad_shares', type: 'PadShare[]' },
  ] as const,
  PadShare: [
    { name: 'label', type: 'bytes32' },
    { name: 'share', type: 'bytes' },
  ] as const,
} as const;

export const kmsPadDecryptResponseEip712PrimaryType = 'PadDecryptResponsePayload' as const;

Object.freeze(kmsPadDecryptResponseEip712Types);
Object.freeze(kmsPadDecryptResponseEip712Types.EIP712Domain);
Object.freeze(kmsPadDecryptResponseEip712Types.PadDecryptResponsePayload);
Object.freeze(kmsPadDecryptResponseEip712Types.PadShare);
kmsPadDecryptResponseEip712Types.EIP712Domain.forEach(Object.freeze);
kmsPadDecryptResponseEip712Types.PadDecryptResponsePayload.forEach(Object.freeze);
kmsPadDecryptResponseEip712Types.PadShare.forEach(Object.freeze);

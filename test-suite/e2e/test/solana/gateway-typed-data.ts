import { ethers } from 'ethers';

export type TypedDataField = { name: string; type: string };
export type TypedDataTypes = Record<string, TypedDataField[]>;
export type TypedDataDomain = {
  name: string;
  version: string;
  chainId: number;
  verifyingContract: string;
};

const EIP712_DOMAIN_TYPE = 'EIP712Domain';

function stripDomainType(types: TypedDataTypes): TypedDataTypes {
  return Object.fromEntries(
    Object.entries(types).filter(([typeName]) => typeName !== EIP712_DOMAIN_TYPE),
  );
}

export function normalizeAddress(value: string): string {
  return ethers.getAddress(value).toLowerCase();
}

// Returns the EIP-55 checksummed address (mixed case). Required by TKMS.
export function checksumAddress(value: string): string {
  return ethers.getAddress(value);
}

export function isAddress(value: string): boolean {
  return ethers.isAddress(value);
}

export function hashTypedData(
  domain: TypedDataDomain,
  types: TypedDataTypes,
  value: Record<string, unknown>,
): string {
  return ethers.TypedDataEncoder.hash(domain, stripDomainType(types), value);
}

export function verifyTypedDataSigner(
  domain: TypedDataDomain,
  types: TypedDataTypes,
  value: Record<string, unknown>,
  signature: string,
): string {
  return ethers.verifyTypedData(domain, stripDomainType(types), value, signature).toLowerCase();
}

// Derives an EVM-compatible client address from a 32-byte native identity.
// Used as a KMS client identifier — not for on-chain verification.
// Returns a checksummed (EIP-55) address as required by the TKMS library.
export function nativeClientAddressFromIdentity(identity: string): string {
  const hash = ethers.keccak256(ethers.getBytes(identity));
  return ethers.getAddress('0x' + hash.slice(-40));
}

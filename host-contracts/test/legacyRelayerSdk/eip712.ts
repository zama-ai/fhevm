import { getAddress, hexlify, isAddress, isHexString } from 'ethers';

import type { EIP712 } from './types';

// Same limits as @fhevm/sdk (sdk/js-sdk/src/core/kms/SignedDecryptionPermitV1-p.ts)
const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;
const MAX_USER_DECRYPT_DURATION_DAYS = 365;
const MAX_UINT64 = 2n ** 64n - 1n;
const MAX_UINT256 = 2n ** 256n - 1n;

// Lowercase, or mixed-case with a valid EIP-55 checksum (same rule as @fhevm/sdk's isAddress)
const isStrictAddress = (value: unknown): value is string =>
  typeof value === 'string' && isAddress(value) && (value === value.toLowerCase() || getAddress(value) === value);

const parseUint = (value: unknown, name: string, max: bigint): bigint => {
  const isUint =
    (typeof value === 'number' && Number.isSafeInteger(value) && value >= 0) ||
    (typeof value === 'string' && /^(0|[1-9][0-9]*)$/.test(value));
  if (!isUint || BigInt(value as number | string) > max) {
    throw new Error(`${name} is not a valid unsigned integer, got ${String(value)}`);
  }
  return BigInt(value as number | string);
};

// publicKey: Uint8Array, or even-length hex string with or without 0x prefix.
// Returns the 0x-prefixed hex string (same rule as @fhevm/sdk's _verifyPublicKeyArg)
const verifyPublicKeyArg = (publicKey: unknown): string => {
  if (publicKey === null || publicKey === undefined) {
    throw new Error('Missing publicKey argument.');
  }
  if (typeof publicKey === 'string') {
    const publicKeyBytesHex = publicKey.startsWith('0x') ? publicKey : `0x${publicKey}`;
    if (!isHexString(publicKeyBytesHex, true)) {
      throw new Error('Invalid publicKey argument.');
    }
    return publicKeyBytesHex;
  }
  if (publicKey instanceof Uint8Array) {
    return hexlify(publicKey);
  }
  throw new Error('Invalid publicKey argument.');
};

export const createEIP712Mocked =
  (verifyingContract: string, chainId: number) =>
  (
    publicKey: string | Uint8Array,
    contractAddresses: string[],
    startTimestamp: string | number,
    durationDays: string | number,
  ): EIP712 => {
    parseUint(chainId, 'chainId', MAX_UINT64);
    if (!isStrictAddress(verifyingContract)) {
      throw new Error('Invalid verifying contract address.');
    }

    const publicKeyBytesHex = verifyPublicKeyArg(publicKey);

    if (!Array.isArray(contractAddresses) || !contractAddresses.every(isStrictAddress)) {
      throw new Error('Invalid contract address.');
    }
    if (contractAddresses.length === 0) {
      throw new Error('contractAddresses is empty');
    }
    if (contractAddresses.length > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
      throw new Error(`contractAddresses max length of ${MAX_USER_DECRYPT_CONTRACT_ADDRESSES} exceeded`);
    }

    parseUint(startTimestamp, 'startTimestamp', MAX_UINT256);
    const days = parseUint(durationDays, 'durationDays', MAX_UINT256);
    if (days < 1n) {
      throw new Error(`durationDays must be at least 1 day, got ${days}`);
    }
    if (days > BigInt(MAX_USER_DECRYPT_DURATION_DAYS)) {
      throw new Error(`durationDays is above max duration of ${MAX_USER_DECRYPT_DURATION_DAYS}`);
    }

    return {
      domain: {
        name: 'Decryption',
        version: '1',
        chainId,
        verifyingContract,
      },
      primaryType: 'UserDecryptRequestVerification',
      types: {
        EIP712Domain: [
          { name: 'name', type: 'string' },
          { name: 'version', type: 'string' },
          { name: 'chainId', type: 'uint256' },
          { name: 'verifyingContract', type: 'address' },
        ],
        UserDecryptRequestVerification: [
          { name: 'publicKey', type: 'bytes' },
          { name: 'contractAddresses', type: 'address[]' },
          { name: 'startTimestamp', type: 'uint256' },
          { name: 'durationDays', type: 'uint256' },
          { name: 'extraData', type: 'bytes' },
        ],
      },
      message: {
        publicKey: publicKeyBytesHex,
        contractAddresses: contractAddresses.map((c) => getAddress(c)),
        startTimestamp: startTimestamp.toString(),
        durationDays: durationDays.toString(),
        extraData: '0x00',
      },
    };
  };

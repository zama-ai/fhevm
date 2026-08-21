// The budget and the width table, checked against the contract that enforces them.
//
// The normative table is the Gateway's `FHETypeBitSizes`, at the point where the budget is enforced;
// everything here is a mirror. A mirror is only worth having if something notices when it stops
// matching, so this test reads the Solidity sources and compares them field by field: the cap in
// `Decryption.sol`, the per-type widths in `FHETypeBitSizes.sol`, and the enum positions in
// `FheType.sol` that turn a type name into the byte a handle carries.
//
// Reading the sources rather than a generated artifact is deliberate: the contracts are in this
// repository, and a copied number that CI never compares is how the two drift apart in the first
// place.

import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { MAX_DECRYPTION_REQUEST_BITS, decryptionRequestBitsOfHandle } from './decryptionRequestBudget.js';

const CONTRACTS = new URL('../../../../../gateway-contracts/contracts/', import.meta.url);
const read = (relative: string): string => readFileSync(new URL(relative, CONTRACTS), 'utf8');

/** `enum FheType { … }` in declaration order: the position is the value a handle's type byte holds. */
function fheTypeOrder(): readonly string[] {
  const source = read('shared/FheType.sol');
  const body = /enum FheType \{([^}]*)\}/.exec(source)?.[1];
  if (body === undefined) {
    throw new Error('FheType.sol does not declare an enum FheType');
  }
  return body
    .split(',')
    .map((entry) => entry.replace(/\/\/.*$/gm, '').trim())
    .filter((entry) => entry.length > 0);
}

/** The `getBitSize` branches: FHE type name to its cleartext bit width. */
function solidityBitSizes(): ReadonlyMap<string, number> {
  const source = read('libraries/FHETypeBitSizes.sol');

  const constants = new Map<string, number>();
  for (const match of source.matchAll(/uint16 internal constant (\w+) = (\d+);/g)) {
    const [, name, value] = match;
    if (name === undefined || value === undefined) {
      throw new Error(`FHETypeBitSizes.sol has a size constant this test cannot read: ${match[0]}`);
    }
    constants.set(name, Number(value));
  }
  expect(constants.size, 'FHETypeBitSizes.sol declares its sizes as named constants').toBeGreaterThan(0);

  const sizes = new Map<string, number>();
  for (const match of source.matchAll(/fheType == FheType\.(\w+)\)\s*\{\s*return (\w+);/g)) {
    const [, typeName, constantName] = match;
    if (typeName === undefined || constantName === undefined) {
      throw new Error(`FHETypeBitSizes.sol has a branch this test cannot read: ${match[0]}`);
    }
    const bits = constants.get(constantName);
    if (bits === undefined) {
      throw new Error(`FHETypeBitSizes.sol returns ${constantName} for ${typeName} but never declares it`);
    }
    sizes.set(typeName, bits);
  }
  return sizes;
}

/** A handle carrying a given FHE type byte: the type lives at byte 30, the version at byte 31. */
const handleOfFheTypeId = (fheTypeId: number): Uint8Array => {
  const handle = new Uint8Array(32).fill(0xa1);
  handle[30] = fheTypeId;
  handle[31] = 0;
  return handle;
};

////////////////////////////////////////////////////////////////////////////////

describe('the decryption-request budget', () => {
  it('is the cap the Gateway enforces', () => {
    const declared = /MAX_DECRYPTION_REQUEST_BITS = (\d+);/.exec(read('Decryption.sol'))?.[1];
    expect(declared, 'Decryption.sol declares MAX_DECRYPTION_REQUEST_BITS').toBeDefined();
    expect(MAX_DECRYPTION_REQUEST_BITS).toBe(Number(declared));
  });
});

describe('the per-handle width', () => {
  const order = fheTypeOrder();
  const sizes = solidityBitSizes();
  const sized = [...sizes.keys()].map((typeName) => {
    const fheTypeId = order.indexOf(typeName);
    if (fheTypeId < 0) {
      throw new Error(`FHETypeBitSizes.sol sizes ${typeName}, which enum FheType does not declare`);
    }
    return [typeName, fheTypeId, sizes.get(typeName) ?? 0] as const;
  });

  it('covers every type the contract assigns a size to', () => {
    // The contract is the normative table, so this is a coverage claim about the mirror: a type the
    // Gateway would accept and this SDK cannot measure is a request refused locally and accepted on
    // chain — a client blocked from something it is entitled to.
    expect(sized.length).toBeGreaterThan(0);
    for (const [typeName, fheTypeId, bits] of sized) {
      expect(decryptionRequestBitsOfHandle(handleOfFheTypeId(fheTypeId)), `${typeName} (id ${fheTypeId})`).toBe(bits);
    }
  });

  // The other direction: a type the contract does not size must have no width here either. A mirror
  // that invented a width would let a request through the pre-check and into a revert.
  it('gives no width to a type the contract does not size', () => {
    const unsized = order
      .map((typeName, fheTypeId) => [typeName, fheTypeId] as const)
      .filter(([typeName]) => !sizes.has(typeName));
    expect(unsized.length, 'enum FheType declares types the size table omits').toBeGreaterThan(0);

    for (const [typeName, fheTypeId] of unsized) {
      expect(
        decryptionRequestBitsOfHandle(handleOfFheTypeId(fheTypeId)),
        `${typeName} (id ${fheTypeId})`,
      ).toBeUndefined();
    }
  });

  it('gives no width to bytes that are not a handle', () => {
    expect(decryptionRequestBitsOfHandle(new Uint8Array(31))).toBeUndefined();
    expect(decryptionRequestBitsOfHandle(new Uint8Array(33))).toBeUndefined();
    expect(decryptionRequestBitsOfHandle(new Uint8Array(0))).toBeUndefined();
  });

  // The boundary pair the budget is about, expressed in the smallest type the table carries: the cap
  // is a multiple of it, so exactly-full and one-over differ by a single handle.
  it('measures the boundary pair the budget turns on', () => {
    const smallest = Math.min(...sized.map(([, , bits]) => bits));
    const smallestId = sized.find(([, , bits]) => bits === smallest)?.[1] ?? 0;
    const handle = handleOfFheTypeId(smallestId);
    const perHandle = decryptionRequestBitsOfHandle(handle);
    expect(perHandle).toBe(smallest);

    expect(MAX_DECRYPTION_REQUEST_BITS % smallest).toBe(0);
    const full = MAX_DECRYPTION_REQUEST_BITS / smallest;
    expect(full * smallest).toBe(MAX_DECRYPTION_REQUEST_BITS);
    expect((full + 1) * smallest).toBeGreaterThan(MAX_DECRYPTION_REQUEST_BITS);
  });
});

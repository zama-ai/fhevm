import type { ChecksummedAddress } from '../types/primitives.js';
import { describe, it, expect, vi } from 'vitest';
import { ZkProofError } from '../errors/ZkProofError.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { AddressError } from '../base/errors/AddressError.js';
import { ChecksummedAddressError } from '../base/errors/ChecksummedAddressError.js';
import { asBytesHex } from '../base/bytes.js';
import { createZkProofBuilder } from './ZkProofBuilder-p.js';
import {
  isZkProof,
  assertIsZkProof,
  toZkProof,
  zkProofGetUnsafeRawBytes,
  zkProofToExternalEncryptedValues,
} from './ZkProof-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/coprocessor/ZkProofBuilder.test.ts
////////////////////////////////////////////////////////////////////////////////

vi.mock('../key/fetchFheEncryptionKey.js', () => ({
  fetchFheEncryptionKeyWasm: vi.fn().mockResolvedValue({} as any),
}));

////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////

const aclContractAddress = '0x325ea1b59F28e9e1C51d3B5b47b7D3965CC5D8C8' as ChecksummedAddress;
const chainId = 1234;
const contractAddress = '0xa5e1defb98EFe38EBb2D958CEe052410247F4c80' as ChecksummedAddress;
const userAddress = '0x8ba1f109551bD432803012645Ac136ddd64DBA72' as ChecksummedAddress;

function makeMockContext(overrides?: { aclAddress?: string; chainId?: number }) {
  const acl = { address: overrides?.aclAddress ?? aclContractAddress };
  return {
    chain: {
      id: overrides?.chainId ?? chainId,
      fhevm: {
        relayerUrl: 'http://mock',
        contracts: { acl },
      },
    },
    runtime: {
      config: { moduleVersions: { tfhe: '1.6.1' } },
    },
  } as any;
}

////////////////////////////////////////////////////////////////////////////////

describe('ZkProofBuilder', () => {
  it('throws errors', async () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const builder = createZkProofBuilder() as any;
    builder.addUint128(BigInt(0));

    const invalidAddress = '0x0';
    const invalidShortAddress = '0x8ba1f109551bd432803012645ac136ddd64d';
    const context = makeMockContext();

    /// Invalid User Address
    await expect(
      builder.build(context, { contractAddress, userAddress: invalidAddress, extraData: '0x00' }),
    ).rejects.toThrow(new ZkProofError({ message: `Invalid user address: ${invalidAddress}` }));

    /// Invalid User Address (wrong length, non-checksummed)
    await expect(
      builder.build(context, { contractAddress, userAddress: invalidShortAddress, extraData: '0x00' }),
    ).rejects.toThrow(new ZkProofError({ message: `Invalid user address: ${invalidShortAddress}` }));

    /// Invalid Contract Address
    await expect(
      builder.build(context, { contractAddress: invalidAddress, userAddress, extraData: '0x00' }),
    ).rejects.toThrow(new ZkProofError({ message: `Invalid contract address: ${invalidAddress}` }));

    /// Invalid Contract Address (wrong length, non-checksummed)
    await expect(
      builder.build(context, { contractAddress: invalidShortAddress, userAddress, extraData: '0x00' }),
    ).rejects.toThrow(new ZkProofError({ message: `Invalid contract address: ${invalidShortAddress}` }));

    /// Invalid values for add* methods
    expect(() => builder.addBool('hello')).toThrow(InvalidTypeError);
    expect(() => builder.addBool({})).toThrow(InvalidTypeError);
    expect(() => builder.addBool(29393)).toThrow(InvalidTypeError);
    expect(() => builder.addUint8(2 ** 8)).toThrow(InvalidTypeError);
    expect(() => builder.addUint16(2 ** 16)).toThrow(InvalidTypeError);
    expect(() => builder.addUint32(2 ** 32)).toThrow(InvalidTypeError);
    expect(() => builder.addUint64(0xffffffffffffffffn + 1n)).toThrow(InvalidTypeError);
    expect(() => builder.addUint128(0xffffffffffffffffffffffffffffffffn + 1n)).toThrow(InvalidTypeError);
    expect(() => builder.addUint256(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffn + 1n)).toThrow(
      InvalidTypeError,
    );
    expect(() => builder.addAddress('0x00')).toThrow(AddressError);
  });

  it('throws if total bits is above 2048', () => {
    const builder = createZkProofBuilder();
    for (let i = 0; i < 8; ++i) {
      builder.addUint256(BigInt(123456789) * BigInt(i + 1));
    }
    expect(() => builder.addBool(false)).toThrow(
      'Packing more than 2048 bits in a single input ciphertext is unsupported',
    );
  });

  it('count and totalBits getters', () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const builder = createZkProofBuilder() as any;
    expect(builder.count).toBe(0);
    expect(builder.totalBits).toBe(0);

    builder.addBool(true);
    expect(builder.count).toBe(1);
    expect(builder.totalBits).toBe(2); // ebool = 2 bits

    builder.addUint32(123);
    expect(builder.count).toBe(2);
    expect(builder.totalBits).toBe(34); // 2 + 32
  });

  it('getBits returns copy of bits array', () => {
    const builder = createZkProofBuilder();
    builder.addBool(true);
    builder.addUint8(42);

    const bits = builder.getBits();
    expect(bits).toEqual([2, 8]);

    // Verify it's a copy, not the original
    (bits as number[]).push(16);
    expect(builder.getBits()).toEqual([2, 8]);
  });

  it('methods return this for chaining', () => {
    const builder = createZkProofBuilder();

    const result = builder
      .addBool(true)
      .addUint8(1)
      .addUint16(2)
      .addUint32(3)
      .addUint64(BigInt(4))
      .addUint128(BigInt(5))
      .addUint256(BigInt(6))
      .addAddress('0xa5e1defb98EFe38EBb2D958CEe052410247F4c80');

    expect(result).toBe(builder);
  });

  it('throws on invalid ACL address', async () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const builder = createZkProofBuilder() as any;
    builder.addUint8(1);

    const context = makeMockContext({ aclAddress: '0x0' });

    await expect(builder.build(context, { contractAddress, userAddress, extraData: '0x00' })).rejects.toThrow(
      new ZkProofError({ message: 'Invalid ACL address: 0x0' }),
    );
  });

  it('throws on invalid chainId', async () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const builder = createZkProofBuilder() as any;
    builder.addUint8(1);

    const context = makeMockContext({ chainId: -1 });

    await expect(builder.build(context, { contractAddress, userAddress, extraData: '0x00' })).rejects.toThrow(
      new ZkProofError({ message: 'Invalid chain ID uint64: -1' }),
    );
  });

  it('throws on negative values', () => {
    const builder = createZkProofBuilder();

    expect(() => builder.addUint8(-1)).toThrow();
    expect(() => builder.addUint16(-1)).toThrow();
    expect(() => builder.addUint32(-1)).toThrow();
    expect(() => builder.addUint64(BigInt(-1))).toThrow();
    expect(() => builder.addUint128(BigInt(-1))).toThrow();
    expect(() => builder.addUint256(BigInt(-1))).toThrow();
  });

  it('throws on null/undefined values for addBool', () => {
    const builder = createZkProofBuilder();

    expect(() => builder.addBool(null as any)).toThrow();
    expect(() => builder.addBool(undefined as any)).toThrow();
  });

  it('throws if packing more than 256 variables', () => {
    const builder = createZkProofBuilder();

    for (let i = 0; i < 256; ++i) {
      builder.addBool(true);
    }
    expect(() => builder.addBool(true)).toThrow(
      `Packing more than 256 variables in a single input ciphertext is unsupported`,
    );
  });
});

////////////////////////////////////////////////////////////////////////////////

// Minimal ciphertext — content is irrelevant for non-WASM tests.
const DUMMY_CT = new Uint8Array([0xde, 0xad, 0xbe, 0xef]);
const DUMMY_CT_HEX = '0xdeadbeef';
const EXTRA_DATA = asBytesHex('0x00');

function makeMockParser(encryptionBits: number[]) {
  return {
    parserFn: {
      parseTFHEProvenCompactCiphertextList: vi.fn().mockResolvedValue({ encryptionBits }),
    },
    tfheVersion: '1.6.1',
  } as any;
}

async function makeZkProof(overrides?: {
  chainId?: bigint | number;
  aclContractAddress?: string;
  contractAddress?: string;
  userAddress?: string;
  ciphertextWithZkProof?: Uint8Array | string;
  encryptionBits?: number[];
  extraData?: string;
}) {
  return toZkProof(
    {
      chainId: overrides?.chainId ?? chainId,
      aclContractAddress: overrides?.aclContractAddress ?? aclContractAddress,
      contractAddress: overrides?.contractAddress ?? contractAddress,
      userAddress: overrides?.userAddress ?? userAddress,
      ciphertextWithZkProof: overrides?.ciphertextWithZkProof ?? DUMMY_CT,
      encryptionBits: overrides?.encryptionBits ?? [2],
    },
    asBytesHex(overrides?.extraData ?? '0x00'),
  );
}

////////////////////////////////////////////////////////////////////////////////

describe('ZkProof', () => {
  describe('toZkProof', () => {
    it('creates a valid ZkProof from ZkProofLike', async () => {
      const zkProof = await makeZkProof();
      expect(isZkProof(zkProof)).toBe(true);
    });

    it('is idempotent when passed an existing ZkProof', async () => {
      const zkProof = await makeZkProof();
      const same = await toZkProof(zkProof, EXTRA_DATA);
      expect(same).toBe(zkProof);
    });

    it('accepts a hex string for ciphertextWithZkProof', async () => {
      const zkProof = await makeZkProof({ ciphertextWithZkProof: DUMMY_CT_HEX });
      expect(zkProof.ciphertextWithZkProof).toEqual(DUMMY_CT);
    });

    it('normalizes lowercase addresses to checksummed', async () => {
      const lowercase = contractAddress.toLowerCase();
      const zkProof = await makeZkProof({ contractAddress: lowercase });
      expect(zkProof.contractAddress).toBe(contractAddress);
    });

    it('copies ciphertextWithZkProof by default', async () => {
      const original = new Uint8Array([1, 2, 3]);
      const zkProof = await makeZkProof({ ciphertextWithZkProof: original });
      original[0] = 99;
      expect(zkProof.ciphertextWithZkProof[0]).toBe(1);
    });

    it('takes ownership with copy:false', async () => {
      const original = new Uint8Array([1, 2, 3]);
      const zkProof = await toZkProof(
        {
          chainId,
          aclContractAddress,
          contractAddress,
          userAddress,
          ciphertextWithZkProof: original,
          encryptionBits: [2],
        },
        EXTRA_DATA,
        { copy: false },
      );
      original[0] = 99;
      expect(zkProof.ciphertextWithZkProof[0]).toBe(99);
    });

    it('extracts encryptionBits from parser when not provided', async () => {
      const zkProof = await toZkProof(
        { chainId, aclContractAddress, contractAddress, userAddress, ciphertextWithZkProof: DUMMY_CT },
        EXTRA_DATA,
        { zkProofParser: makeMockParser([8]) },
      );
      expect(zkProof.encryptionBits).toEqual([8]);
    });

    it('accepts provided encryptionBits when parser agrees', async () => {
      const zkProof = await toZkProof(
        {
          chainId,
          aclContractAddress,
          contractAddress,
          userAddress,
          ciphertextWithZkProof: DUMMY_CT,
          encryptionBits: [32],
        },
        EXTRA_DATA,
        { zkProofParser: makeMockParser([32]) },
      );
      expect(zkProof.encryptionBits).toEqual([32]);
    });

    it('throws on invalid chainId', async () => {
      await expect(makeZkProof({ chainId: -1 })).rejects.toThrow(InvalidTypeError);
    });

    it('throws on invalid aclContractAddress', async () => {
      await expect(makeZkProof({ aclContractAddress: '0x0' })).rejects.toThrow(AddressError);
    });

    it('throws on invalid contractAddress', async () => {
      await expect(makeZkProof({ contractAddress: '0x0' })).rejects.toThrow(AddressError);
    });

    it('throws on invalid userAddress', async () => {
      await expect(makeZkProof({ userAddress: '0x0' })).rejects.toThrow(AddressError);
    });

    it('throws on empty ciphertextWithZkProof', async () => {
      await expect(makeZkProof({ ciphertextWithZkProof: new Uint8Array(0) })).rejects.toThrow(
        new ZkProofError({ message: 'ciphertextWithZkProof argument should not be empty' }),
      );
    });

    it('throws when encryptionBits missing and no parser provided', async () => {
      await expect(
        toZkProof(
          { chainId, aclContractAddress, contractAddress, userAddress, ciphertextWithZkProof: DUMMY_CT },
          EXTRA_DATA,
        ),
      ).rejects.toThrow(new ZkProofError({ message: 'Missing encryption bits' }));
    });

    it('throws on invalid encryptionBits value', async () => {
      // 4 is not a valid EncryptionBits value (euint4 is deprecated)
      await expect(makeZkProof({ encryptionBits: [4] })).rejects.toThrow(InvalidTypeError);
    });

    it('throws when parser count mismatches provided encryptionBits', async () => {
      await expect(
        toZkProof(
          {
            chainId,
            aclContractAddress,
            contractAddress,
            userAddress,
            ciphertextWithZkProof: DUMMY_CT,
            encryptionBits: [2, 8],
          },
          EXTRA_DATA,
          { zkProofParser: makeMockParser([8]) }, // parser returns 1 item, provided has 2
        ),
      ).rejects.toThrow(new ZkProofError({ message: 'Encryption count mismatch, expected 2, got 1.' }));
    });

    it('throws when parser type mismatches provided encryptionBits', async () => {
      await expect(
        toZkProof(
          {
            chainId,
            aclContractAddress,
            contractAddress,
            userAddress,
            ciphertextWithZkProof: DUMMY_CT,
            encryptionBits: [2],
          },
          EXTRA_DATA,
          { zkProofParser: makeMockParser([8]) }, // parser says uint8, provided says ebool
        ),
      ).rejects.toThrow(new ZkProofError({ message: 'Encryption type mismatch at index 0.' }));
    });
  });

  describe('isZkProof / assertIsZkProof', () => {
    it('isZkProof returns true for a ZkProof', async () => {
      const zkProof = await makeZkProof();
      expect(isZkProof(zkProof)).toBe(true);
    });

    it('isZkProof returns false for non-ZkProof values', () => {
      expect(isZkProof(null)).toBe(false);
      expect(isZkProof(undefined)).toBe(false);
      expect(isZkProof('string')).toBe(false);
      expect(isZkProof({})).toBe(false);
      expect(isZkProof(42)).toBe(false);
    });

    it('assertIsZkProof passes for a valid ZkProof', async () => {
      const zkProof = await makeZkProof();
      expect(() => assertIsZkProof(zkProof, {})).not.toThrow();
    });

    it('assertIsZkProof throws InvalidTypeError for non-ZkProof', () => {
      expect(() => assertIsZkProof('not-a-proof', {})).toThrow(InvalidTypeError);
      expect(() => assertIsZkProof(null, {})).toThrow(InvalidTypeError);
    });
  });

  describe('getters', () => {
    it('exposes all fields correctly', async () => {
      const zkProof = await makeZkProof({ encryptionBits: [2, 32] });

      expect(zkProof.chainId).toBe(BigInt(chainId));
      expect(zkProof.aclContractAddress).toBe(aclContractAddress);
      expect(zkProof.contractAddress).toBe(contractAddress);
      expect(zkProof.userAddress).toBe(userAddress);
      expect(zkProof.encryptionBits).toEqual([2, 32]);
      // ebool(2)→fheTypeId 0, euint32(32)→fheTypeId 4
      expect((zkProof as any).fheTypeIds).toEqual([0, 4]);
      expect(zkProof.ciphertextWithZkProof).toEqual(DUMMY_CT);
    });

    it('ciphertextWithZkProof getter returns a fresh copy each time', async () => {
      const zkProof = await makeZkProof();
      const a = zkProof.ciphertextWithZkProof;
      const b = zkProof.ciphertextWithZkProof;
      expect(a).not.toBe(b);
      expect(a).toEqual(b);
      a[0] = 0xff;
      expect(zkProof.ciphertextWithZkProof[0]).toBe(DUMMY_CT[0]);
    });

    it('extraData is accessible via getExtraData()', async () => {
      const zkProof = await makeZkProof({ extraData: '0xabcd' });
      expect(zkProof.getExtraData()).toBe(asBytesHex('0xabcd'));
    });
  });

  describe('toString / toJSON', () => {
    it('toString includes metadata without exposing ciphertext content', async () => {
      const zkProof = await makeZkProof({ encryptionBits: [8, 32] });
      const str = zkProof.toString();
      expect(str).toContain(String(chainId));
      expect(str).toContain(contractAddress);
      expect(str).toContain(userAddress);
      expect(str).toContain('2'); // 2 encrypted values
    });

    it('toJSON returns a plain object with the expected shape', async () => {
      const zkProof = await makeZkProof({ encryptionBits: [2, 8] });
      const json = (zkProof as any).toJSON();

      expect(json.chainId).toBe(chainId); // small chainId → number
      expect(json.aclContractAddress).toBe(aclContractAddress);
      expect(json.contractAddress).toBe(contractAddress);
      expect(json.userAddress).toBe(userAddress);
      expect(json.ciphertextWithZkProof).toBe(DUMMY_CT_HEX);
      expect(json.encryptionBits).toEqual([2, 8]);
      expect(json.fheTypeIds).toEqual([0, 2]); // ebool→0, uint8→2
    });

    it('toJSON keeps chainId as bigint when above MAX_SAFE_INTEGER', async () => {
      const largeChainId = BigInt(Number.MAX_SAFE_INTEGER) + 1n;
      const zkProof = await makeZkProof({ chainId: largeChainId });
      expect((zkProof as any).toJSON().chainId).toBe(largeChainId);
    });
  });

  describe('getInputHandles', () => {
    it('returns one handle per encrypted value', async () => {
      const zkProof = await makeZkProof({ encryptionBits: [2, 8, 32] });
      const handles = zkProof.getInputHandles();
      expect(handles).toHaveLength(3);
    });

    it('each handle has a valid 32-byte hex representation', async () => {
      const zkProof = await makeZkProof({ encryptionBits: [8] });
      const [handle] = zkProof.getInputHandles();
      expect(handle!.bytes32Hex).toMatch(/^0x[0-9a-f]{64}$/);
    });

    it('handles are deterministic for identical inputs', async () => {
      const a = await makeZkProof({ encryptionBits: [8, 32] });
      const b = await makeZkProof({ encryptionBits: [8, 32] });
      expect(a.getInputHandles().map((h) => h.bytes32Hex)).toEqual(b.getInputHandles().map((h) => h.bytes32Hex));
    });

    it('result is cached — same array reference on repeated calls', async () => {
      const zkProof = await makeZkProof({ encryptionBits: [8] });
      expect(zkProof.getInputHandles()).toBe(zkProof.getInputHandles());
    });

    it('different ciphertexts produce different handles', async () => {
      const a = await makeZkProof({ ciphertextWithZkProof: new Uint8Array([1, 2, 3, 4]), encryptionBits: [8] });
      const b = await makeZkProof({ ciphertextWithZkProof: new Uint8Array([5, 6, 7, 8]), encryptionBits: [8] });
      expect(a.getInputHandles()[0]!.bytes32Hex).not.toBe(b.getInputHandles()[0]!.bytes32Hex);
    });

    it('different chainIds produce different handles', async () => {
      const a = await makeZkProof({ chainId: 1, encryptionBits: [8] });
      const b = await makeZkProof({ chainId: 2, encryptionBits: [8] });
      expect(a.getInputHandles()[0]!.bytes32Hex).not.toBe(b.getInputHandles()[0]!.bytes32Hex);
    });
  });

  describe('zkProofGetUnsafeRawBytes', () => {
    it('returns the internal bytes without copying', async () => {
      const zkProof = await makeZkProof();
      const unsafe1 = zkProofGetUnsafeRawBytes(zkProof);
      const unsafe2 = zkProofGetUnsafeRawBytes(zkProof);
      expect(unsafe1).toBe(unsafe2); // same reference
      expect(unsafe1).not.toBe(zkProof.ciphertextWithZkProof); // getter makes a copy
      expect(unsafe1).toEqual(DUMMY_CT);
    });

    it('throws when passed a non-ZkProof', () => {
      expect(() => zkProofGetUnsafeRawBytes({} as any)).toThrow('Unauthorized');
    });
  });

  describe('zkProofToExternalEncryptedValues', () => {
    it('delegates to getInputHandles for a ZkProof instance', async () => {
      const zkProof = await makeZkProof({ encryptionBits: [2, 8] });
      const handles = await zkProofToExternalEncryptedValues(zkProof);
      expect(handles).toBe(zkProof.getInputHandles());
    });

    it('computes handles from a plain ZkProofLike with checksummed ACL', async () => {
      const handles = await zkProofToExternalEncryptedValues({
        chainId: BigInt(chainId),
        aclContractAddress,
        contractAddress,
        userAddress,
        ciphertextWithZkProof: DUMMY_CT,
        encryptionBits: [8],
      });
      expect(handles).toHaveLength(1);
      expect(handles[0]!.bytes32Hex).toMatch(/^0x[0-9a-f]{64}$/);
    });

    it('throws ChecksummedAddressError for non-checksummed ACL in ZkProofLike', async () => {
      await expect(
        zkProofToExternalEncryptedValues({
          chainId,
          aclContractAddress: aclContractAddress.toLowerCase(),
          contractAddress,
          userAddress,
          ciphertextWithZkProof: DUMMY_CT,
          encryptionBits: [8],
        }),
      ).rejects.toThrow(ChecksummedAddressError);
    });
  });
});

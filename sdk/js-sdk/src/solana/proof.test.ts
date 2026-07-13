import { describe, expect, it } from 'vitest';
import {
  bytesToHex,
  decodeMmrProofTransportBlob,
  deriveValueKey,
  hexToBytes,
  historicalAccessLeafCommitment,
  MAX_MMR_SIBLINGS,
  MMR_MODE_HISTORICAL,
  mmrLeafNode,
  mmrNode,
  mmrVerify,
  publicDecryptLeafCommitment,
  verifyHistoricalAccessProof,
  verifyPublicDecryptProof,
  type MmrProof,
} from './proof.js';

// Every vector below is the actual output of the Rust shared crate
// (`solana/crates/zama-solana-acl`), captured by running
// `cargo run -p kms-worker --example mmr_vectors` (kms-connector/crates/kms-worker/examples/
// mmr_vectors.rs) against the SAME inputs used here. This pins the TS hashing to be
// byte-identical to the Rust implementation the on-chain program and the KMS connector run —
// not merely internally consistent with itself.
const domain = new Uint8Array(32).fill(1);
const app = new Uint8Array(32).fill(2);
const label = new Uint8Array(32).fill(3);
const account = new Uint8Array(32).fill(4);
const handle = new Uint8Array(32).fill(5);
const subject = new Uint8Array(32).fill(6);

function concatBytes(...parts: readonly Uint8Array[]): Uint8Array {
  const total = parts.reduce((n, p) => n + p.length, 0);
  const out = new Uint8Array(total);
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
}

function u32LE(value: number): Uint8Array {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setUint32(0, value, true);
  return out;
}

function u64LE(value: bigint): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, true);
  return out;
}

function proofBlob(mode: number, leafIndex: bigint, siblings: readonly Uint8Array[]): Uint8Array {
  return concatBytes(new Uint8Array([mode]), u64LE(leafIndex), u32LE(siblings.length), ...siblings);
}

describe('hexToBytes', () => {
  it('accepts unprefixed, 0x-prefixed, and 0X-prefixed hex', () => {
    expect(hexToBytes('00aF')).toEqual(new Uint8Array([0x00, 0xaf]));
    expect(hexToBytes('0x00aF')).toEqual(new Uint8Array([0x00, 0xaf]));
    expect(hexToBytes('0X00aF')).toEqual(new Uint8Array([0x00, 0xaf]));
  });

  it('round-trips bytes through the exported hex helpers', () => {
    const bytes = new Uint8Array([0x00, 0x12, 0xab, 0xff]);
    expect(hexToBytes(bytesToHex(bytes))).toEqual(bytes);
  });

  it('rejects malformed characters instead of coercing them to zero', () => {
    expect(() => hexToBytes('0xzz')).toThrow('hexToBytes: invalid hex string: 0xzz');
    expect(() => hexToBytes('0y00')).toThrow('hexToBytes: invalid hex string: 0y00');
  });

  it('rejects odd-length hex', () => {
    expect(() => hexToBytes('0x123')).toThrow('hexToBytes: odd-length hex string: 0x123');
  });
});

describe('decodeMmrProofTransportBlob', () => {
  const sibling = new Uint8Array(32).fill(0x42);

  it('decodes a canonical mode-prefixed Borsh MmrProof', () => {
    const decoded = decodeMmrProofTransportBlob(proofBlob(MMR_MODE_HISTORICAL, 7n, [sibling]));
    expect(decoded.mode).toBe(MMR_MODE_HISTORICAL);
    expect(decoded.proof.leafIndex).toBe(7n);
    expect(decoded.proof.siblings).toEqual([sibling]);
  });

  it('rejects trailing bytes after the Borsh MmrProof', () => {
    const canonical = proofBlob(MMR_MODE_HISTORICAL, 7n, [sibling]);
    const withTrailing = concatBytes(canonical, new Uint8Array([0xde, 0xad]));
    expect(() => decodeMmrProofTransportBlob(withTrailing)).toThrow(/trailing byte/);
  });
});

describe('deriveValueKey', () => {
  it('matches the Rust crate vector', () => {
    const valueKey = deriveValueKey(domain, app, label);
    expect(bytesToHex(valueKey)).toBe('0xcb421159e2c7709e401334c46b4bcee90093cb616d040fca9c1dc9a14ad77820');
  });
});

describe('leaf commitments', () => {
  it('historicalAccessLeafCommitment matches the Rust crate vector', () => {
    const commitment = historicalAccessLeafCommitment(account, 0n, handle, subject);
    expect(bytesToHex(commitment)).toBe('0x22844bf8442e4ed2541819f2d087bf66430d798aa90bfe9bb7119cdd0efdc089');
  });

  it('publicDecryptLeafCommitment matches the Rust crate vector', () => {
    const commitment = publicDecryptLeafCommitment(account, 0n, handle);
    expect(bytesToHex(commitment)).toBe('0x778f26fcf5fc5e99bb41a656e35b1d22e51523de4e82beb20f042108c379130c');
  });
});

describe('mmr node hashing', () => {
  const hist = hexToBytes('0x22844bf8442e4ed2541819f2d087bf66430d798aa90bfe9bb7119cdd0efdc089');
  const pub = hexToBytes('0x778f26fcf5fc5e99bb41a656e35b1d22e51523de4e82beb20f042108c379130c');

  it('mmrLeafNode matches the Rust crate vector', () => {
    expect(bytesToHex(mmrLeafNode(hist))).toBe('0x85b18302d4f5ac95de84eaef50f3badab330f686be31006b8f5036eae81ece29');
  });

  it('mmrNode matches the Rust crate vector', () => {
    expect(bytesToHex(mmrNode(hist, pub))).toBe('0x9b0e61c2adae588290493cec461368ce6384bf984640174a45d13ca7990041dd');
  });
});

describe('mmrVerify against a real 3-leaf MMR (Rust crate vectors)', () => {
  // Built by the Rust example: three historical-access leaves for handles [0,0,...],
  // [1,1,...], [2,2,...] all against `subject`, appended in order.
  const leaves = [0, 1, 2].map((i) =>
    historicalAccessLeafCommitment(account, BigInt(i), new Uint8Array(32).fill(i), subject),
  );
  const peaks = [
    hexToBytes('0xd4342d5c4e6c2f3d48957506fa885ca9824c44d79e395f2089320ff47b30e279'),
    hexToBytes('0x648e8f50ef511d05e44f67f12a788ed23983379c70ade469e196eb01f8bc11a4'),
  ];
  const leafCount = 3n;

  it('accepts the proof for leaf 1 against the Rust-derived peaks', () => {
    const proof: MmrProof = {
      leafIndex: 1n,
      siblings: [hexToBytes('0x37721a2329065e637682b98191177ff06d059819ee95def4d16cd518436a516e')],
    };
    expect(mmrVerify(peaks, leafCount, leaves[1]!, proof)).toBe(true);
  });

  it('rejects a tampered commitment (substitution)', () => {
    const proof: MmrProof = {
      leafIndex: 1n,
      siblings: [hexToBytes('0x37721a2329065e637682b98191177ff06d059819ee95def4d16cd518436a516e')],
    };
    const tampered = historicalAccessLeafCommitment(account, 1n, new Uint8Array(32).fill(0x99), subject);
    expect(mmrVerify(peaks, leafCount, tampered, proof)).toBe(false);
  });

  it('rejects a proof at a stale/wrong leaf_count (drift)', () => {
    const proof: MmrProof = {
      leafIndex: 1n,
      siblings: [hexToBytes('0x37721a2329065e637682b98191177ff06d059819ee95def4d16cd518436a516e')],
    };
    expect(mmrVerify(peaks, 4n, leaves[1]!, proof)).toBe(false);
  });

  it('rejects an out-of-range leaf_index', () => {
    const proof: MmrProof = { leafIndex: 5n, siblings: [] };
    expect(mmrVerify(peaks, leafCount, leaves[0]!, proof)).toBe(false);
  });

  it('rejects a sibling-count cap violation', () => {
    const oversized: MmrProof = {
      leafIndex: 0n,
      siblings: Array.from({ length: MAX_MMR_SIBLINGS + 1 }, () => new Uint8Array(32)),
    };
    expect(mmrVerify(peaks, leafCount, leaves[0]!, oversized)).toBe(false);
  });

  it('verifyHistoricalAccessProof end-to-end against the live peaks', () => {
    const proof: MmrProof = {
      leafIndex: 1n,
      siblings: [hexToBytes('0x37721a2329065e637682b98191177ff06d059819ee95def4d16cd518436a516e')],
    };
    expect(verifyHistoricalAccessProof(account, peaks, leafCount, new Uint8Array(32).fill(1), subject, proof)).toBe(
      true,
    );
  });

  it('verifyPublicDecryptProof rejects a historical leaf under the public-decrypt domain', () => {
    // Same account/leaf_index/handle shape but hashed under the WRONG domain prefix must not
    // verify — domain separation between historical and public leaves is load-bearing.
    const proof: MmrProof = { leafIndex: 0n, siblings: [] };
    expect(verifyPublicDecryptProof(account, peaks, leafCount, new Uint8Array(32).fill(0), proof)).toBe(false);
  });
});

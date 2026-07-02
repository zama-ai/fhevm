import { describe, it, expect } from 'vitest';
import { gunzip, inflateRaw, inflateDecompress } from './inflate.js';

type Format = 'gzip' | 'deflate' | 'deflate-raw';

// Compress with the platform `CompressionStream` (the exact inverse of what the SDK
// uses) so the vendored inflater is validated against a reference compressor rather
// than hand-rolled fixtures.
async function compress(data: Uint8Array, format: Format): Promise<Uint8Array> {
  // A Uint8Array is a valid BlobPart at runtime; cast around TS 5.7's ArrayBufferLike
  // width (Uint8Array<ArrayBufferLike> vs the expected Uint8Array<ArrayBuffer>).
  const stream = new Blob([data as unknown as BlobPart]).stream().pipeThrough(new CompressionStream(format));
  return new Uint8Array(await new Response(stream).arrayBuffer());
}

function bytesEqual(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) {
    return false;
  }
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) {
      return false;
    }
  }
  return true;
}

// Deterministic pseudo-random bytes (LCG) — incompressible, so the compressor emits
// stored blocks, exercising that decode path. Deterministic = reproducible failures.
function pseudoRandomBytes(n: number): Uint8Array {
  const out = new Uint8Array(n);
  let s = 0x12345678;
  for (let i = 0; i < n; i++) {
    s = (s * 1103515245 + 12345) & 0x7fffffff;
    out[i] = s & 0xff;
  }
  return out;
}

const text = new TextEncoder();

const FIXTURES: ReadonlyArray<{ readonly name: string; readonly data: Uint8Array }> = [
  { name: 'empty', data: new Uint8Array(0) },
  { name: 'single byte', data: new Uint8Array([0x42]) },
  { name: 'short text', data: text.encode('hello world!') },
  // Highly repetitive → many LZ77 back-references (dynamic-Huffman blocks).
  { name: 'repetitive', data: text.encode('the quick brown fox '.repeat(4000)) },
  // Every byte value 0..255.
  { name: 'all-byte-values', data: Uint8Array.from({ length: 256 }, (_unused, i) => i) },
  // Incompressible → stored blocks.
  { name: 'incompressible-40k', data: pseudoRandomBytes(40_000) },
  // Large → forces multiple blocks and several output-buffer growths.
  { name: 'large-256k', data: pseudoRandomBytes(256_000) },
];

describe('inflate', () => {
  describe('gunzip (gzip container)', () => {
    for (const { name, data } of FIXTURES) {
      it(`round-trips ${name}`, async () => {
        const out = gunzip(await compress(data, 'gzip'));
        expect(out.length).toBe(data.length);
        expect(bytesEqual(out, data)).toBe(true);
      });
    }
  });

  describe('inflateRaw (raw DEFLATE)', () => {
    for (const { name, data } of FIXTURES) {
      it(`round-trips ${name}`, async () => {
        const out = inflateRaw(await compress(data, 'deflate-raw'));
        expect(bytesEqual(out, data)).toBe(true);
      });
    }

    it('produces correct output regardless of sizeHint (buffer grows)', async () => {
      const data = pseudoRandomBytes(100_000);
      const raw = await compress(data, 'deflate-raw');
      expect(bytesEqual(inflateRaw(raw, 1), data)).toBe(true); // tiny hint → many grows
      expect(bytesEqual(inflateRaw(raw, data.length), data)).toBe(true); // exact hint
      expect(bytesEqual(inflateRaw(raw), data)).toBe(true); // no hint
    });
  });

  describe('inflateDecompress (dispatch)', () => {
    const data = text.encode('FHEVM '.repeat(2000));
    for (const format of ['gzip', 'deflate', 'deflate-raw'] as const) {
      it(`handles ${format}`, async () => {
        const out = inflateDecompress(await compress(data, format), format);
        expect(bytesEqual(out, data)).toBe(true);
      });
    }
  });

  describe('errors', () => {
    it('gunzip throws on a non-gzip stream', () => {
      const notGzip = new Uint8Array(20).fill(0x41);
      expect(() => gunzip(notGzip)).toThrow(/gzip/i);
    });

    it('gunzip throws on a too-short input', () => {
      expect(() => gunzip(new Uint8Array([0x1f, 0x8b, 0x08]))).toThrow(/gzip/i);
    });
  });
});

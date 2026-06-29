/* eslint-disable @typescript-eslint/no-non-null-assertion */
// Vendored, dependency-free DEFLATE / gzip / zlib inflater.
//
// Why this exists: the SDK embeds its WASM as gzip-compressed base64 and normally
// inflates it with the platform `DecompressionStream`. Some runtimes have no usable
// `DecompressionStream` — older browsers (Firefox <113, Safari <16.4) and the
// Next.js Edge Runtime (which exposes a global stub that throws on construction, see
// `supportsDecompressionStream` in environment.ts). For those, callers fall back to
// this pure-JS inflater, which runs in any JS runtime with no platform API and no
// third-party dependency.
//
// The DEFLATE decoder is a TypeScript reimplementation of the public-domain "tinf"
// inflate (Joergen Ibsen), itself derived from Mark Adler's zlib "puff" reference.
// It supports all RFC 1951 block types (stored, fixed-Huffman, dynamic-Huffman) and
// adds the RFC 1952 (gzip) and RFC 1950 (zlib) container framings. Per-call state is
// allocated fresh (no shared mutable globals); only the immutable static tables are
// shared. Decompression happens once per WASM load, so allocation cost is irrelevant.

type Tree = {
  /** Number of codes of each bit length (index = length, 0..15). */
  readonly counts: Uint16Array;
  /** Code -> symbol translation table, grouped by code length. */
  readonly symbols: Uint16Array;
};

type InflateState = {
  readonly source: Uint8Array;
  pos: number; // next byte to read from source
  tag: number; // bit buffer (LSB-first)
  bitcount: number; // valid bits currently in `tag`
  dest: Uint8Array<ArrayBuffer>; // output buffer (grown on demand)
  destLen: number; // bytes written to `dest`
};

function newTree(maxSymbols: number): Tree {
  return { counts: new Uint16Array(16), symbols: new Uint16Array(maxSymbols) };
}

// ---- Immutable static tables (built once at module load) --------------------

// Length codes 257..285: extra-bit counts and base lengths (index = symbol - 257).
const LENGTH_BITS = new Uint8Array(30);
const LENGTH_BASE = new Uint16Array(30);
// Distance codes 0..29: extra-bit counts and base distances.
const DIST_BITS = new Uint8Array(30);
const DIST_BASE = new Uint16Array(30);
// Order in which code-length-code lengths are stored (RFC 1951 §3.2.7).
const CLC_ORDER = new Uint8Array([16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15]);
const FIXED_LTREE = newTree(288);
const FIXED_DTREE = newTree(30);

function buildBitsBase(bits: Uint8Array, base: Uint16Array, delta: number, first: number): void {
  for (let i = 0; i < delta; i++) {
    bits[i] = 0;
  }
  for (let i = 0; i < 30 - delta; i++) {
    bits[i + delta] = Math.floor(i / delta);
  }
  let sum = first;
  for (let i = 0; i < 30; i++) {
    base[i] = sum;
    sum += 1 << bits[i]!;
  }
}

function buildFixedTrees(lt: Tree, dt: Tree): void {
  for (let i = 0; i < 7; i++) {
    lt.counts[i] = 0;
  }
  lt.counts[7] = 24;
  lt.counts[8] = 152;
  lt.counts[9] = 112;
  for (let i = 0; i < 24; i++) {
    lt.symbols[i] = 256 + i;
  }
  for (let i = 0; i < 144; i++) {
    lt.symbols[24 + i] = i;
  }
  for (let i = 0; i < 8; i++) {
    lt.symbols[24 + 144 + i] = 280 + i;
  }
  for (let i = 0; i < 112; i++) {
    lt.symbols[24 + 144 + 8 + i] = 144 + i;
  }
  for (let i = 0; i < 5; i++) {
    dt.counts[i] = 0;
  }
  dt.counts[5] = 32;
  for (let i = 0; i < 32; i++) {
    dt.symbols[i] = i;
  }
}

buildBitsBase(LENGTH_BITS, LENGTH_BASE, 4, 3);
buildBitsBase(DIST_BITS, DIST_BASE, 2, 1);
// Length code 285 has no extra bits and a fixed length of 258 (the generic formula
// above would assign it extra bits; override it).
LENGTH_BITS[28] = 0;
LENGTH_BASE[28] = 258;
buildFixedTrees(FIXED_LTREE, FIXED_DTREE);

// ---- Bit reader -------------------------------------------------------------

function readBits(d: InflateState, num: number, base: number): number {
  if (num === 0) {
    return base;
  }
  while (d.bitcount < 24) {
    d.tag |= d.source[d.pos++]! << d.bitcount;
    d.bitcount += 8;
  }
  const val = d.tag & (0xffff >>> (16 - num));
  d.tag >>>= num;
  d.bitcount -= num;
  return val + base;
}

function decodeSymbol(d: InflateState, t: Tree): number {
  while (d.bitcount < 24) {
    d.tag |= d.source[d.pos++]! << d.bitcount;
    d.bitcount += 8;
  }
  let sum = 0;
  let cur = 0;
  let len = 0;
  let tag = d.tag;
  // Walk the canonical-Huffman code one bit at a time until the running code
  // value falls within the range of codes of the current length.
  do {
    cur = 2 * cur + (tag & 1);
    tag >>>= 1;
    len++;
    sum += t.counts[len]!;
    cur -= t.counts[len]!;
  } while (cur >= 0);
  d.tag = tag;
  d.bitcount -= len;
  return t.symbols[sum + cur]!;
}

// ---- Tree / block decoding --------------------------------------------------

function buildTree(t: Tree, lengths: Uint8Array, off: number, num: number): void {
  for (let i = 0; i < 16; i++) {
    t.counts[i] = 0;
  }
  for (let i = 0; i < num; i++) {
    const codeLen = lengths[off + i]!;
    t.counts[codeLen] = t.counts[codeLen]! + 1;
  }
  t.counts[0] = 0;
  const offsets = new Uint16Array(16);
  let sum = 0;
  for (let i = 0; i < 16; i++) {
    offsets[i] = sum;
    sum += t.counts[i]!;
  }
  for (let i = 0; i < num; i++) {
    const codeLen = lengths[off + i]!;
    if (codeLen !== 0) {
      const slot = offsets[codeLen]!;
      t.symbols[slot] = i;
      offsets[codeLen] = slot + 1;
    }
  }
}

function decodeTrees(d: InflateState, lt: Tree, dt: Tree): void {
  const codeTree = newTree(19);
  const lengths = new Uint8Array(288 + 32);

  const hlit = readBits(d, 5, 257);
  const hdist = readBits(d, 5, 1);
  const hclen = readBits(d, 4, 4);

  for (let i = 0; i < hclen; i++) {
    lengths[CLC_ORDER[i]!] = readBits(d, 3, 0);
  }
  buildTree(codeTree, lengths, 0, 19);

  let num = 0;
  while (num < hlit + hdist) {
    const sym = decodeSymbol(d, codeTree);
    if (sym === 16) {
      // Repeat previous code length 3..6 times.
      const prev = lengths[num - 1]!;
      for (let length = readBits(d, 2, 3); length > 0; length--) {
        lengths[num++] = prev;
      }
    } else if (sym === 17) {
      // Repeat zero 3..10 times.
      for (let length = readBits(d, 3, 3); length > 0; length--) {
        lengths[num++] = 0;
      }
    } else if (sym === 18) {
      // Repeat zero 11..138 times.
      for (let length = readBits(d, 7, 11); length > 0; length--) {
        lengths[num++] = 0;
      }
    } else {
      lengths[num++] = sym;
    }
  }

  buildTree(lt, lengths, 0, hlit);
  buildTree(dt, lengths, hlit, hdist);
}

function ensureCapacity(d: InflateState, extra: number): void {
  const needed = d.destLen + extra;
  if (needed <= d.dest.length) {
    return;
  }
  let newLen = d.dest.length * 2;
  while (newLen < needed) {
    newLen *= 2;
  }
  const bigger = new Uint8Array(newLen);
  bigger.set(d.dest);
  d.dest = bigger;
}

function inflateBlock(d: InflateState, lt: Tree, dt: Tree): void {
  for (;;) {
    let sym = decodeSymbol(d, lt);
    if (sym === 256) {
      return; // end of block
    }
    if (sym < 256) {
      ensureCapacity(d, 1);
      d.dest[d.destLen++] = sym;
    } else {
      sym -= 257;
      const length = readBits(d, LENGTH_BITS[sym]!, LENGTH_BASE[sym]!);
      const distSym = decodeSymbol(d, dt);
      const dist = readBits(d, DIST_BITS[distSym]!, DIST_BASE[distSym]!);
      const start = d.destLen - dist;
      ensureCapacity(d, length);
      // LZ77 back-reference: copy byte-by-byte (ranges may overlap, e.g. run-length).
      for (let i = 0; i < length; i++) {
        d.dest[d.destLen++] = d.dest[start + i]!;
      }
    }
  }
}

function inflateStoredBlock(d: InflateState): void {
  // Stored blocks are byte-aligned: push whole bytes back into the source so the
  // LEN/NLEN fields are read from byte boundaries.
  while (d.bitcount > 8) {
    d.pos--;
    d.bitcount -= 8;
  }
  const length = (d.source[d.pos + 1]! << 8) | d.source[d.pos]!;
  const invLength = (d.source[d.pos + 3]! << 8) | d.source[d.pos + 2]!;
  if (length !== (~invLength & 0xffff) >>> 0) {
    throw new Error('inflate: invalid stored block length');
  }
  d.pos += 4;
  ensureCapacity(d, length);
  for (let i = length; i > 0; i--) {
    d.dest[d.destLen++] = d.source[d.pos++]!;
  }
  d.tag = 0;
  d.bitcount = 0;
}

/**
 * Inflates a raw DEFLATE (RFC 1951) byte stream — no gzip/zlib container.
 *
 * @param source - raw DEFLATE bytes.
 * @param sizeHint - expected uncompressed size, if known (e.g. gzip ISIZE). Used to
 * pre-size the output buffer and avoid reallocation; correctness does not depend on
 * it (the buffer grows as needed).
 */
export function inflateRaw(source: Uint8Array, sizeHint?: number): Uint8Array<ArrayBuffer> {
  const initialSize = sizeHint !== undefined && sizeHint > 0 ? sizeHint : Math.max(source.length * 4, 1024);
  const d: InflateState = {
    source,
    pos: 0,
    tag: 0,
    bitcount: 0,
    dest: new Uint8Array(initialSize),
    destLen: 0,
  };

  for (;;) {
    const bfinal = readBits(d, 1, 0);
    const btype = readBits(d, 2, 0);
    if (btype === 0) {
      inflateStoredBlock(d);
    } else if (btype === 1) {
      inflateBlock(d, FIXED_LTREE, FIXED_DTREE);
    } else if (btype === 2) {
      const lt = newTree(288);
      const dt = newTree(32);
      decodeTrees(d, lt, dt);
      inflateBlock(d, lt, dt);
    } else {
      throw new Error('inflate: invalid block type');
    }
    if (bfinal !== 0) {
      break;
    }
  }

  return d.dest.subarray(0, d.destLen);
}

/**
 * Decompresses a gzip (RFC 1952) stream: validates the magic + method, skips the
 * optional header fields, inflates the DEFLATE body, and pre-sizes the output from
 * the trailer's ISIZE (uncompressed size mod 2^32).
 */
export function gunzip(source: Uint8Array): Uint8Array<ArrayBuffer> {
  if (source.length < 18 || source[0] !== 0x1f || source[1] !== 0x8b) {
    throw new Error('gunzip: not a gzip stream');
  }
  if (source[2] !== 8) {
    throw new Error(`gunzip: unsupported compression method ${String(source[2])}`);
  }
  const flg = source[3]!;
  let pos = 10; // fixed gzip header size
  if ((flg & 0x04) !== 0) {
    // FEXTRA: 2-byte length + payload
    pos += 2 + ((source[pos]! | (source[pos + 1]! << 8)) >>> 0);
  }
  if ((flg & 0x08) !== 0) {
    // FNAME: NUL-terminated
    while (source[pos] !== 0) {
      pos++;
    }
    pos++;
  }
  if ((flg & 0x10) !== 0) {
    // FCOMMENT: NUL-terminated
    while (source[pos] !== 0) {
      pos++;
    }
    pos++;
  }
  if ((flg & 0x02) !== 0) {
    // FHCRC: 2-byte header CRC
    pos += 2;
  }

  const n = source.length;
  const isize = (source[n - 4]! | (source[n - 3]! << 8) | (source[n - 2]! << 16) | (source[n - 1]! << 24)) >>> 0;
  return inflateRaw(source.subarray(pos), isize);
}

/**
 * Pure-JS equivalent of `DecompressionStream` for the formats the SDK emits.
 * Use as the fallback when `supportsDecompressionStream()` is false.
 *
 * - `gzip`        — RFC 1952 (magic + header + DEFLATE + CRC/ISIZE trailer).
 * - `deflate`     — RFC 1950 zlib (2-byte header + DEFLATE + Adler-32 trailer).
 * - `deflate-raw` — RFC 1951 raw DEFLATE.
 */
export function inflateDecompress(
  source: Uint8Array,
  format: 'gzip' | 'deflate' | 'deflate-raw',
): Uint8Array<ArrayBuffer> {
  if (format === 'gzip') {
    return gunzip(source);
  }
  if (format === 'deflate-raw') {
    return inflateRaw(source);
  }
  // 'deflate' (zlib): skip the 2-byte CMF/FLG header; inflate stops at the final
  // block, so the 4-byte Adler-32 trailer is simply never read.
  return inflateRaw(source.subarray(2));
}

#!/usr/bin/env node
/*
 * Generic wasm-to-base64 builder.
 *
 * Reads a .wasm file, optionally compresses it via the Web `CompressionStream`
 * (same API the runtime uses for decompression — guaranteed bit-compatible),
 * then base64-encodes the result and emits an ES module plus a matching
 * declaration file that exports it.
 *
 * Usage:
 *   node wasm-to-base64.build.mjs <input.wasm> [--out <path>] [--export <name>] [--no-compress]
 *
 * Defaults:
 *   --out      <input>.base64.js  (next to the input)
 *   --export   wasmBase64
 *   compress=true (toggle with --no-compress)
 */
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { dirname, basename, join, isAbsolute, resolve } from 'node:path';

////////////////////////////////////////////////////////////////////////////////

const DEFAULT_COMPRESSION_FORMAT = 'gzip';

const args = process.argv.slice(2);

if (args.includes('-h') || args.includes('--help')) {
  printUsage();
  process.exit(0);
}

const compress = !args.includes('--no-compress');
const flag = (name) => {
  const idx = args.indexOf(name);
  if (idx < 0) return undefined;
  const v = args[idx + 1];
  if (!v || v.startsWith('--')) {
    console.error(`error: ${name} requires a value`);
    process.exit(1);
  }
  return v;
};
const positional = args.filter((a, i) => {
  if (a.startsWith('--')) return false;
  const prev = args[i - 1];
  return prev !== '--out' && prev !== '--export';
});

const input = positional[0];
if (!input) {
  printUsage();
  process.exit(1);
}

const exportName = flag('--export') ?? 'wasmBase64';
const inputAbs = isAbsolute(input) ? input : resolve(process.cwd(), input);
const outAbs = flag('--out')
  ? resolve(process.cwd(), flag('--out'))
  : join(dirname(inputAbs), `${basename(inputAbs)}.base64.js`);
const outDtsAbs = declarationFilePath(outAbs);

////////////////////////////////////////////////////////////////////////////////

const wasmBytes = readFileSync(inputAbs);

let payload = wasmBytes;
const compressionFormat = compress ? DEFAULT_COMPRESSION_FORMAT : undefined;
if (compress) {
  const compressedStream = new Blob([wasmBytes]).stream().pipeThrough(new CompressionStream(compressionFormat));
  payload = new Uint8Array(await new Response(compressedStream).arrayBuffer());
}

const base64 = Buffer.from(payload).toString('base64');
const sha256 = createHash('sha256').update(base64).digest('hex');
const encodingTag = compress ? 'gzip+base64' : 'base64';

const gzippedFlagName = `${exportName}IsGzipped`;
const compressionFormatName = `${exportName}CompressionFormat`;
const compressionFormatLiteral = compressionFormat === undefined ? 'undefined' : JSON.stringify(compressionFormat);

writeFileSync(
  outAbs,
  `// Auto-generated — do not edit.
// Source:   ${input}
// Encoding: ${encodingTag}
// SHA-256:  ${sha256}
export const ${exportName} = "${base64}";
export const ${gzippedFlagName} = ${compress};
export const ${compressionFormatName} = ${compressionFormatLiteral};
`,
);

writeFileSync(
  outDtsAbs,
  `export const ${exportName}: string;
export const ${gzippedFlagName}: boolean;
export const ${compressionFormatName}: 'gzip' | 'deflate' | 'deflate-raw' | undefined;
`,
);

const wasmMB = (wasmBytes.length / 1024 / 1024).toFixed(2);
const payloadMB = (payload.length / 1024 / 1024).toFixed(2);
const b64MB = (base64.length / 1024 / 1024).toFixed(2);
console.log(`✔ ${outAbs}`);
console.log(`✔ ${outDtsAbs}`);
console.log(`  encoding: ${encodingTag}`);
console.log(`  exports:  ${exportName}, ${gzippedFlagName}, ${compressionFormatName}`);
console.log(`  wasm:     ${wasmMB}MB`);
if (compress) {
  const ratio = ((payload.length / wasmBytes.length) * 100).toFixed(1);
  console.log(`  gzip:     ${payloadMB}MB (${ratio}% of wasm)`);
}
console.log(`  base64:   ${b64MB}MB`);
console.log(`  sha256:   ${sha256}`);

////////////////////////////////////////////////////////////////////////////////

function printUsage() {
  console.error('Usage: wasm-to-base64.build.mjs <input.wasm> [--out <path>] [--export <name>] [--no-compress]');
}

function declarationFilePath(jsPath) {
  if (jsPath.endsWith('.js')) {
    return `${jsPath.slice(0, -'.js'.length)}.d.ts`;
  }
  return `${jsPath}.d.ts`;
}

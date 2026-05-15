import { build } from 'esbuild';
import { cpSync, mkdirSync, readdirSync, statSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';

const SRC = 'src/wasm';
const DEST = 'src/_cjs/wasm';

const copyOnly = process.argv.includes('--copy-only');

// Legacy mode: copy the entire wasm tree verbatim. Reproduces the original
// `cpSync('src/wasm','src/_cjs/wasm',{recursive:true})` behavior.
if (copyOnly) {
  cpSync(SRC, DEST, { recursive: true });
  process.exit(0);
}

// Default mode: transpile only the files that are reached at runtime from
// CJS-context code. Everything else (workers, dead code, .wasm binaries,
// .d.ts, .cjs, etc.) is copied verbatim.
//
// The 5 files below are the ones whose ESM `import`/`export` syntax would
// otherwise be evaluated under `_cjs/package.json {"type":"commonjs"}` and
// produce a SyntaxError at load time.
const TRANSPILE = [
  'tfhe/tfhe.v1.5.3.js',
  'tfhe/startWorkers.v1.5.3.js',
  'tfhe/tfhe_bg.v1.5.3.wasm.base64.js',
  'tkms/kms_lib.v0.13.10.js',
  'tkms/kms_lib_bg.v0.13.10.wasm.base64.js',
  'wasmBaseUrl.js',
];

function walk(dir, out = []) {
  for (const e of readdirSync(dir)) {
    const p = join(dir, e);
    statSync(p).isDirectory() ? walk(p, out) : out.push(p);
  }
  return out;
}

const allFiles = walk(SRC);
const transpileSet = new Set(TRANSPILE.map((p) => join(SRC, p)));

// 1. ESM → CJS for the listed entries. `bundle: false` does syntax-only
// transpilation per file: relative imports stay as relative requires, so the
// output mirrors the input directory layout.
await build({
  entryPoints: [...transpileSet],
  outdir: DEST,
  outbase: SRC,
  format: 'cjs',
  platform: 'node',
  bundle: false,
  sourcemap: 'linked',
  logLevel: 'info',
});

// 2. Copy everything else verbatim, recreating any parent dirs as needed.
for (const f of allFiles) {
  if (transpileSet.has(f)) continue;
  const dest = join(DEST, relative(SRC, f));
  mkdirSync(dirname(dest), { recursive: true });
  cpSync(f, dest);
}

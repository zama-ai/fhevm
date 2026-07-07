import { build } from 'esbuild';
import { cpSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import {
  WASM_SRC,
  assertTscJsOutputs,
  cleanWasmDest,
  compileWasmTypescriptToDest,
  copyWasmRuntimeFiles,
  createWasmBuildContext,
  generatedWasmLoaderRels,
  generatedWasmRels,
  isWasmBase64DataRel,
  logWasmBuildSummary,
  removeUnexpectedTscJsOutputs,
  writeGeneratedWasmArtifacts,
} from './wasm-build-common.mjs';

const context = createWasmBuildContext('cjs', 'build-cjs-wasm');

const copyOnly = process.argv.includes('--copy-only');

// Legacy mode: copy the entire wasm tree verbatim. Reproduces the original
// `cpSync('src/wasm','src/_cjs/wasm',{recursive:true})` behavior.
if (copyOnly) {
  cpSync(WASM_SRC, context.dest, { recursive: true });
  process.exit(0);
}

const transpileSet = new Set(
  context.allSourceFiles.filter((f) => {
    const rel = context.sourceRel(f);
    return rel.endsWith('.js') && !generatedWasmRels.has(rel);
  }),
);

// Keep standalone build runs deterministic too: without this, directories
// copied by a previous build may survive after the filters change.
cleanWasmDest(context);

////////////////////////////////////////////////////////////////////////////////
// 1. ESM → CJS for allowed runtime JS files.
//
//    Auto-generated base64 WASM payloads are transpiled without a sourcemap:
//    they're giant data-only files with no logic to step through, and the
//    sourcemap would otherwise roughly double their published size.
////////////////////////////////////////////////////////////////////////////////

const base64DataEntryPoints = [...transpileSet].filter((f) => isWasmBase64DataRel(context.sourceRel(f)));
const codeEntryPoints = [...transpileSet].filter((f) => !isWasmBase64DataRel(context.sourceRel(f)));

const sharedBuildOptions = {
  outdir: context.dest,
  outbase: WASM_SRC,
  format: 'cjs',
  platform: 'node',
  bundle: false,
  logLevel: 'info',
  // wasmBaseUrl.js intentionally uses import.meta.url with a CJS fallback;
  // suppress the warning since the empty-object case is already handled.
  logOverride: { 'empty-import-meta': 'silent' },
};

if (codeEntryPoints.length > 0) {
  await build({ ...sharedBuildOptions, entryPoints: codeEntryPoints, sourcemap: 'linked' });
}
if (base64DataEntryPoints.length > 0) {
  await build({ ...sharedBuildOptions, entryPoints: base64DataEntryPoints, sourcemap: false });
}

////////////////////////////////////////////////////////////////////////////////
// 2. Copy everything else verbatim. Skips the generated sources — we regenerate
//    them in step 3 to apply the profile filter.
////////////////////////////////////////////////////////////////////////////////

copyWasmRuntimeFiles(context, { skipSourcePaths: transpileSet });

compileWasmTypescriptToDest(context);
removeUnexpectedTscJsOutputs(context);
assertTscJsOutputs(context);

////////////////////////////////////////////////////////////////////////////////
// 3. Generate profile-filtered loaders/API declarations. Loaders are transpiled
//    straight into DEST; declarations are written verbatim. Source files in
//    src/wasm/ are NOT mutated — the filter is applied only to the published
//    artifact.
////////////////////////////////////////////////////////////////////////////////

const tmp = join(tmpdir(), `fhevm-loaders-${process.pid}-${Date.now()}`);
writeGeneratedWasmArtifacts(tmp, context.versions, (artifact) => artifact.kind === 'loader');
writeGeneratedWasmArtifacts(context.dest, context.versions, (artifact) => artifact.kind === 'apiDeclaration');

try {
  await build({
    entryPoints: generatedWasmLoaderRels.map((rel) => join(tmp, rel)),
    outdir: context.dest,
    outbase: tmp,
    format: 'cjs',
    platform: 'node',
    bundle: false,
    sourcemap: 'linked',
    logLevel: 'silent',
  });
} finally {
  rmSync(tmp, { recursive: true, force: true });
}

logWasmBuildSummary(context);

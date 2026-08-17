// Copies src/wasm → src/_esm/wasm, with four profile-aware adjustments:
//  1. Copy only shared runtime files and manifest-listed version directories
//     whose `tags` include the active profile.
//  2. Compile non-test TypeScript sources in src/wasm using the WASM-local
//     tsconfig.
//  3. Overwrite the loaders in the output with a freshly-generated copy that
//     references only profile-included versions (source loaders are untouched).
//  4. Overwrite the shared API declarations in the output with freshly-generated
//     copies whose version unions match the active profile.
//
// Replaces the original one-liner:
//   node -e "require('fs').cpSync('src/wasm','src/_esm/wasm',{recursive:true})"

import {
  assertTscJsOutputs,
  cleanWasmDest,
  compileWasmTypescriptToDest,
  copyWasmRuntimeFiles,
  createWasmBuildContext,
  logWasmBuildSummary,
  removeUnexpectedTscJsOutputs,
  writeGeneratedWasmArtifacts,
} from './wasm-build-common.mjs';

const context = createWasmBuildContext('esm', 'build-esm-wasm');

// Keep standalone build runs deterministic too: without this, directories
// copied by a previous build may survive after the filters change.
cleanWasmDest(context);

// 1. Copy only shared runtime files and manifest-listed version directories.
copyWasmRuntimeFiles(context);

// 2. Compile standard tsc output for non-test TypeScript sources in src/wasm.
compileWasmTypescriptToDest(context);
removeUnexpectedTscJsOutputs(context);
assertTscJsOutputs(context);

// 3. Write profile-filtered generated files directly (ESM target, no transpile).
writeGeneratedWasmArtifacts(context.dest, context.versions);

logWasmBuildSummary(context);

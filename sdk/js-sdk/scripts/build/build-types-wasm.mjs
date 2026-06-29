// Emits declarations for src/wasm/**/*.ts through the WASM-local tsconfig,
// copies hand-written .d.ts declarations from src/wasm/ → src/_types/wasm/,
// then regenerates the shared WASM API declarations for the active profile.
//
// Stand-alone .d.ts files in src/wasm/ (e.g. KmsLibApi.d.ts, TfheApi.d.ts,
// per-version tfhe.d.ts / kms_lib.d.ts, loadXxxLib.d.ts) are tsc inputs, not
// outputs, so they still need to be mirrored into src/_types/wasm/.
//
// The generated declarations in src/_types/core/.../init-p.d.ts import from
// those wasm/.d.ts files, so consumers of the published types need them
// mirrored into src/_types/wasm/.
//
// Profile filter mirrors build-cjs-wasm.mjs / build-esm-wasm.mjs: only shared
// declarations and manifest-listed version directories whose tags include the
// active profile are copied, so local/debug folders never land in _types.

import {
  assertTscDeclarationOutputs,
  cleanWasmDest,
  compileWasmTypescriptDeclarationsToDest,
  copyWasmDeclarationFiles,
  createWasmBuildContext,
  removeUnexpectedTscDeclarationOutputs,
  writeGeneratedWasmArtifacts,
} from './wasm-build-common.mjs';

// Keep standalone build runs deterministic too: without this, declarations
// copied by a previous build may survive after the filters change.
const context = createWasmBuildContext('types', 'build-types-wasm');
cleanWasmDest(context);

const copied = copyWasmDeclarationFiles(context);

compileWasmTypescriptDeclarationsToDest(context);
removeUnexpectedTscDeclarationOutputs(context);
assertTscDeclarationOutputs(context);

writeGeneratedWasmArtifacts(context.dest, context.versions, (artifact) => artifact.kind === 'apiDeclaration');

console.log(`[${context.scriptName}] profile=${context.profile}`);
console.log(`[${context.scriptName}]   copied ${copied} .d.ts files to ${context.dest}`);
console.log(`[${context.scriptName}]   generated shared WASM API declarations`);

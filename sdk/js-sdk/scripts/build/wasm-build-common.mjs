import { execFileSync } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join, relative, sep } from 'node:path';

import { BUILD_PROFILES, KMS_MANIFEST, TFHE_MANIFEST } from '../../versionsManifest.js';
import {
  generateKmsApiSource,
  generateKmsLoaderSource,
  resolveDefaultWasmVersions,
  generateTfheApiSource,
  generateTfheLoaderSource,
} from './codegen-loaders.mjs';
import {
  isBuildMetadataRel,
  isDeclarationRel,
  isTypeScriptSourceRel,
  readTsconfigSourceRels,
} from './wasm-tsconfig.mjs';

const require = createRequire(import.meta.url);
const TSC_BIN = require.resolve('typescript/bin/tsc');

export const WASM_SRC = 'src/wasm';

export const generatedWasmArtifacts = Object.freeze({
  tfheLoader: Object.freeze({
    rel: 'tfhe/loadTfheLib.js',
    kind: 'loader',
    source: ({ profile, tfheDefaultVersion, tfheVersions }) =>
      generateTfheLoaderSource(tfheVersions, profile, tfheDefaultVersion),
  }),
  tfheApi: Object.freeze({
    rel: 'tfhe/TfheApi.d.ts',
    kind: 'apiDeclaration',
    source: ({ tfheVersions }) => generateTfheApiSource(tfheVersions),
  }),
  kmsLoader: Object.freeze({
    rel: 'tkms/loadKmsLib.js',
    kind: 'loader',
    source: ({ kmsDefaultVersion, kmsVersions, profile }) =>
      generateKmsLoaderSource(kmsVersions, profile, kmsDefaultVersion),
  }),
  kmsApi: Object.freeze({
    rel: 'tkms/KmsLibApi.d.ts',
    kind: 'apiDeclaration',
    source: ({ kmsVersions }) => generateKmsApiSource(kmsVersions),
  }),
});

export const generatedWasmRels = new Set(Object.values(generatedWasmArtifacts).map((artifact) => artifact.rel));
export const generatedWasmLoaderRels = Object.values(generatedWasmArtifacts)
  .filter((artifact) => artifact.kind === 'loader')
  .map((artifact) => artifact.rel);
export const generatedWasmApiDeclarationRels = new Set(
  Object.values(generatedWasmArtifacts)
    .filter((artifact) => artifact.kind === 'apiDeclaration')
    .map((artifact) => artifact.rel),
);

export function createWasmBuildContext(target, scriptName) {
  const profile = process.env.BUILD_PROFILE ?? 'dev';
  if (!BUILD_PROFILES.includes(profile)) {
    console.error(`[${scriptName}] unknown BUILD_PROFILE='${profile}'. Expected one of: ${BUILD_PROFILES.join(', ')}`);
    process.exit(1);
  }

  const tfheVersions = TFHE_MANIFEST.filter((entry) => entry.tags.includes(profile)).map((entry) => entry.version);
  const kmsVersions = KMS_MANIFEST.filter((entry) => entry.tags.includes(profile)).map((entry) => entry.version);
  const defaultVersions = resolveDefaultWasmVersions(profile);
  const dest = target === 'types' ? 'src/_types/wasm' : `src/_${target}/wasm`;
  const tsconfig = target === 'cjs' ? 'src/wasm/tsconfig.cjs.json' : 'src/wasm/tsconfig.esm.json';
  const tsconfigSourceRels = readTsconfigSourceRels(tsconfig, WASM_SRC);
  const allowedVersionDirs = new Set([
    ...tfheVersions.map((version) => `tfhe/v${version}`),
    ...kmsVersions.map((version) => `tkms/v${version}`),
  ]);
  const versions = {
    kmsDefaultVersion: defaultVersions.tkms,
    kmsVersions,
    profile,
    tfheDefaultVersion: defaultVersions.tfhe,
    tfheVersions,
  };

  const context = {
    allSourceFiles: [],
    allTsSourceRels: [],
    allowedTsSourceRels: [],
    dest,
    kmsDefaultVersion: defaultVersions.tkms,
    kmsVersions,
    profile,
    scriptName,
    sourceRel: (p) => relative(WASM_SRC, p).split(sep).join('/'),
    target,
    tfheDefaultVersion: defaultVersions.tfhe,
    tfheVersions,
    tsconfig,
    tsconfigSourceRels,
    versions,
  };

  context.allSourceFiles = walk(WASM_SRC).filter((p) => isAllowedSourceRel(context.sourceRel(p), allowedVersionDirs));
  context.allTsSourceRels = [...tsconfigSourceRels].filter(isTypeScriptSourceRel);
  context.allowedTsSourceRels = context.allTsSourceRels.filter((rel) =>
    isAllowedTypeScriptSourceRel(rel, allowedVersionDirs),
  );

  return Object.freeze(context);
}

export function cleanWasmDest(context) {
  rmSync(context.dest, { recursive: true, force: true });
}

export function copyWasmRuntimeFiles(context, { skipSourcePaths = new Set() } = {}) {
  for (const f of context.allSourceFiles) {
    if (skipSourcePaths.has(f)) continue;

    const rel = context.sourceRel(f);
    if (isDeclarationRel(rel) && !context.tsconfigSourceRels.has(rel)) continue;
    if (isTypeScriptSourceRel(rel)) continue;
    if (generatedWasmRels.has(rel)) continue;

    copyFileToDest(context, f);
  }
}

export function copyWasmDeclarationFiles(context) {
  let copied = 0;

  for (const f of context.allSourceFiles) {
    const rel = context.sourceRel(f);
    if (!isDeclarationRel(rel)) continue;
    if (!context.tsconfigSourceRels.has(rel)) continue;
    if (generatedWasmApiDeclarationRels.has(rel)) continue;

    copyFileToDest(context, f);
    copied++;
  }

  return copied;
}

export function compileWasmTypescript(context, extraArgs = []) {
  execFileSync(process.execPath, [TSC_BIN, '--project', context.tsconfig, ...extraArgs], { stdio: 'inherit' });
}

export function compileWasmTypescriptToDest(context) {
  compileWasmTypescript(context, ['--outDir', context.dest]);
}

export function compileWasmTypescriptDeclarationsToDest(context) {
  compileWasmTypescript(context, [
    '--declarationDir',
    context.dest,
    '--emitDeclarationOnly',
    '--declaration',
    '--declarationMap',
  ]);
}

export function removeUnexpectedTscJsOutputs(context) {
  removeTscOutputs(
    context,
    context.allTsSourceRels.filter((rel) => !context.allowedTsSourceRels.includes(rel)),
    jsOutputRelForTsSource,
  );
}

export function removeUnexpectedTscDeclarationOutputs(context) {
  removeTscOutputs(
    context,
    context.allTsSourceRels.filter((rel) => !context.allowedTsSourceRels.includes(rel)),
    declarationOutputRelForTsSource,
  );
}

export function assertTscJsOutputs(context) {
  assertTscOutputs(context, context.allowedTsSourceRels, jsOutputRelForTsSource, 'tsc output');
}

export function assertTscDeclarationOutputs(context) {
  assertTscOutputs(context, context.allowedTsSourceRels, declarationOutputRelForTsSource, 'tsc declaration');
}

export function writeGeneratedWasmArtifacts(dest, versions, artifactPredicate = () => true) {
  for (const artifact of Object.values(generatedWasmArtifacts)) {
    if (!artifactPredicate(artifact)) continue;
    const artifactDest = join(dest, artifact.rel);
    mkdirSync(dirname(artifactDest), { recursive: true });
    writeFileSync(artifactDest, artifact.source(versions));
  }
}

export function logWasmBuildSummary(context) {
  console.log(`[${context.scriptName}] profile=${context.profile}`);
  console.log(`[${context.scriptName}]   TFHE versions: ${context.tfheVersions.join(', ') || '(none)'}`);
  console.log(`[${context.scriptName}]   KMS versions:  ${context.kmsVersions.join(', ') || '(none)'}`);
}

export function walk(dir, out = []) {
  for (const e of readdirSync(dir)) {
    const p = join(dir, e);
    statSync(p).isDirectory() ? walk(p, out) : out.push(p);
  }
  return out;
}

function copyFileToDest(context, source) {
  const dest = join(context.dest, relative(WASM_SRC, source));
  mkdirSync(dirname(dest), { recursive: true });
  cpSync(source, dest);
}

function isAllowedSourceRel(rel, allowedVersionDirs) {
  if (isBuildMetadataRel(rel)) return false;
  if (isInsideAllowedVersionDir(rel, allowedVersionDirs)) return true;
  return !isInsideManagedVersionDir(rel);
}

function isAllowedTypeScriptSourceRel(rel, allowedVersionDirs) {
  if (!isTypeScriptSourceRel(rel)) return false;
  if (isInsideAllowedVersionDir(rel, allowedVersionDirs)) return true;
  return !isInsideManagedVersionDir(rel);
}

function isInsideAllowedVersionDir(rel, allowedVersionDirs) {
  for (const dir of allowedVersionDirs) {
    if (rel === dir || rel.startsWith(`${dir}/`)) return true;
  }
  return false;
}

function isInsideManagedVersionDir(rel) {
  return /^tfhe\/v[^/]+(?:\/|$)/.test(rel) || /^tkms\/v[^/]+(?:\/|$)/.test(rel);
}

function jsOutputRelForTsSource(rel) {
  return `${rel.slice(0, -'.ts'.length)}.js`;
}

function declarationOutputRelForTsSource(rel) {
  return `${rel.slice(0, -'.ts'.length)}.d.ts`;
}

function removeTscOutputs(context, sourceRels, outputRelForSource) {
  for (const sourceRel of sourceRels) {
    const outputRel = outputRelForSource(sourceRel);
    rmSync(join(context.dest, outputRel), { force: true });
    rmSync(join(context.dest, `${outputRel}.map`), { force: true });
  }
}

function assertTscOutputs(context, sourceRels, outputRelForSource, label) {
  for (const sourceRel of sourceRels) {
    const outputRel = outputRelForSource(sourceRel);
    if (!existsSync(join(context.dest, outputRel))) {
      throw new Error(`[${context.scriptName}] Missing ${label} ${outputRel} for ${sourceRel}.`);
    }
  }
}

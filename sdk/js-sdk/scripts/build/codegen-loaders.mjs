// Regenerates src/wasm/<lib>/loadXxxLib.js and the shared WASM API declarations
// from versionsManifest.js.
//
// Two ways to use this script:
//
// 1. CLI (regenerates the source loaders/API declarations, committed to git):
//      node scripts/build/codegen-loaders.mjs              # writes the 'dev' (full) generated files
//      node scripts/build/codegen-loaders.mjs --check      # verifies committed generated files are fresh
//      BUILD_PROFILE=prod node scripts/build/codegen-loaders.mjs  # writes the 'prod' generated files
//    The default is 'dev' so the source tree always reflects "every supported
//    version." Only opt into 'prod' source for an experiment; restore with the
//    default before committing.
//
// 2. Programmatic (build scripts):
//      import {
//        generateTfheLoaderSource,
//        generateTfheApiSource,
//        generateKmsLoaderSource,
//        generateKmsApiSource,
//        resolveDefaultWasmVersions,
//      }
//        from './codegen-loaders.mjs';
//      const defaults = resolveDefaultWasmVersions(profile);
//      const source = generateTfheLoaderSource(versions, profile, defaults.tfhe);
//    This is how build-cjs-wasm.mjs / build-esm-wasm.mjs override generated
//    files emitted into src/_cjs/wasm/ and src/_esm/wasm/, without mutating
//    src/wasm/.

import { createHash } from 'node:crypto';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { BUILD_PROFILES, KMS_MANIFEST, TFHE_MANIFEST, WASM_DEFAULT_VERSIONS } from '../../versionsManifest.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const SRC_WASM = resolve(__dirname, '../../src/wasm');
const TEMPLATES_DIR = resolve(__dirname, '../wasm/loaders');

const BUILD_PROFILE_PLACEHOLDER = '__BUILD_PROFILE__';
const TFHE_API_TEMPLATE_PATH = resolve(TEMPLATES_DIR, 'TfheApi.template.d.ts');
const KMS_API_TEMPLATE_PATH = resolve(TEMPLATES_DIR, 'KmsLibApi.template.d.ts');

function fail(message) {
  throw new Error(message);
}

function renderTemplateText(template, label, replacements) {
  for (const [placeholder, replacement] of replacements) {
    if (!template.includes(placeholder)) {
      fail(`Missing ${placeholder} in ${label}.`);
    }

    template = template.replaceAll(placeholder, replacement);
  }

  return `${template.trimEnd()}\n`;
}

function renderTemplate(path, replacements) {
  return renderTemplateText(readFileSync(path, 'utf8'), path, replacements);
}

function renderVersionList(versions) {
  return versions.map((v) => `'${v}'`).join(', ');
}

function renderDisplayVersionList(versions) {
  return versions.map((v) => `v${v}`).join(', ');
}

function renderVersionUnion(versions) {
  return versions.length === 0 ? 'never' : versions.map((v) => `'${v}'`).join(' | ');
}

function renderDefaultVersion(versions, label, defaultVersion = versions[0]) {
  const version = defaultVersion;
  if (version === undefined) {
    fail(`Cannot generate ${label} default version from an empty version list.`);
  }
  if (!versions.includes(version)) {
    fail(
      `Cannot generate ${label} default version '${version}' because it is not included in the active version list: ${versions.join(', ')}`,
    );
  }
  return renderJsString(version);
}

function renderImportLoaders(versions, file) {
  return versions.map((v) => `  '${v}': () => import('./v${v}/${file}'),`).join('\n');
}

function renderJsString(value) {
  return `'${String(value).replaceAll('\\', '\\\\').replaceAll("'", "\\'")}'`;
}

function sha256File(path) {
  if (!existsSync(path)) {
    fail(`Missing file needed for generated asset metadata: ${path}`);
  }
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

function renderAsset(asset) {
  return `Object.freeze({
      filename: ${renderJsString(asset.filename)},
      localRelativePath: ${renderJsString(asset.localRelativePath)},
      sha256: ${renderJsString(asset.sha256)},
    })`;
}

function renderAssetMap(versions, getAssets) {
  return versions
    .map((version) => {
      const entries = Object.entries(getAssets(version))
        .map(([name, asset]) => `    ${name}: ${renderAsset(asset)},`)
        .join('\n');
      return `  '${version}': Object.freeze({\n${entries}\n  }),`;
    })
    .join('\n');
}

function getTfheAssets(version) {
  return {
    wasm: {
      filename: `tfhe_bg.v${version}.wasm`,
      localRelativePath: `./v${version}/tfhe_bg.wasm`,
      sha256: sha256File(resolve(SRC_WASM, `tfhe/v${version}/tfhe_bg.wasm`)),
    },
    worker: {
      filename: `tfhe-worker.v${version}.mjs`,
      localRelativePath: `./v${version}/tfhe-worker.mjs`,
      sha256: sha256File(resolve(SRC_WASM, `tfhe/v${version}/tfhe-worker.mjs`)),
    },
  };
}

function getKmsAssets(version) {
  return {
    wasm: {
      filename: `kms_lib_bg.v${version}.wasm`,
      localRelativePath: `./v${version}/kms_lib_bg.wasm`,
      sha256: sha256File(resolve(SRC_WASM, `tkms/v${version}/kms_lib_bg.wasm`)),
    },
  };
}

function extractApiImportAnchorVersion(importPath, label) {
  const match = /(?:^|\/)v([^/]+)\/[^/]+\.js$/.exec(importPath);
  if (match === null) {
    fail(`Unable to infer ${label} API anchor version from template import: ${importPath}`);
  }
  return match[1];
}

function extractApiVersionType(template, typeName, label) {
  const match = new RegExp(`export type ${typeName} = '([^']+)';`).exec(template);
  if (match === null) {
    fail(
      `Missing single-version ${typeName} anchor in ${label} template. Expected: export type ${typeName} = 'X.Y.Z';`,
    );
  }
  return {
    anchorVersion: match[1],
    source: match[0],
  };
}

function extractApiImportPath(template, label) {
  const match = template.match(/from '([^']+)'/);
  if (match === null) {
    fail(`Missing canonical export import in ${label} template.`);
  }
  return match[1];
}

function assertAnchorVersion(versions, anchorVersion, label) {
  if (!versions.includes(anchorVersion)) {
    fail(`${label} API anchor v${anchorVersion} is not included in the active version set: ${versions.join(', ')}`);
  }
}

function readApiTemplate(path, label, versionTypeName) {
  const template = readFileSync(path, 'utf8');
  const importPath = extractApiImportPath(template, label);
  const versionType = extractApiVersionType(template, versionTypeName, label);
  const importAnchorVersion = extractApiImportAnchorVersion(importPath, label);

  if (versionType.anchorVersion !== importAnchorVersion) {
    fail(
      `${label} API template anchor mismatch: ${versionTypeName} uses v${versionType.anchorVersion}, but canonical import uses v${importAnchorVersion}.`,
    );
  }

  return {
    anchorVersion: versionType.anchorVersion,
    importPath,
    template,
    versionTypeSource: versionType.source,
  };
}

function renderApiSource({ versions, label, templatePath, versionTypeName, outputImportFile }) {
  const { anchorVersion, importPath, template, versionTypeSource } = readApiTemplate(
    templatePath,
    label,
    versionTypeName,
  );
  assertAnchorVersion(versions, anchorVersion, label);

  return renderTemplateText(
    template,
    templatePath,
    new Map([
      [importPath, `./v${anchorVersion}/${outputImportFile}`],
      [versionTypeSource, `export type ${versionTypeName} = ${renderVersionUnion(versions)};`],
    ]),
  );
}

export function getTfheApiAnchorVersion() {
  return readApiTemplate(TFHE_API_TEMPLATE_PATH, 'TFHE', 'TfheVersion').anchorVersion;
}

export function getKmsApiAnchorVersion() {
  return readApiTemplate(KMS_API_TEMPLATE_PATH, 'KMS', 'TkmsVersion').anchorVersion;
}

export function resolveDefaultWasmVersions(profile) {
  const defaults = WASM_DEFAULT_VERSIONS[profile];
  if (defaults === undefined) {
    fail(`Missing WASM default versions for BUILD_PROFILE='${profile}'.`);
  }

  const { tfhe, tkms } = defaults;
  if (tfhe === undefined) {
    fail(`Missing TFHE default version for BUILD_PROFILE='${profile}'.`);
  }
  if (tkms === undefined) {
    fail(`Missing TKMS default version for BUILD_PROFILE='${profile}'.`);
  }

  return defaults;
}

////////////////////////////////////////////////////////////////////////////////
// Generator: TFHE loader source
////////////////////////////////////////////////////////////////////////////////

/**
 * @param {readonly string[]} versions
 * @param {string} profile
 * @param {string} defaultVersion
 * @returns {string} JS source for src/wasm/tfhe/loadTfheLib.js
 */
export function generateTfheLoaderSource(versions, profile, defaultVersion = versions[0]) {
  return renderTemplate(
    resolve(TEMPLATES_DIR, 'loadTfheLib.template.js'),
    new Map([
      [BUILD_PROFILE_PLACEHOLDER, profile],
      ['__TFHE_VERSIONS__', renderVersionList(versions)],
      ['__TFHE_DEFAULT_VERSION__', renderDefaultVersion(versions, 'TFHE', defaultVersion)],
      ['  /* __TFHE_LIB_LOADERS__ */', renderImportLoaders(versions, 'tfhe.js')],
      ['  /* __TFHE_WASM_BASE64_LOADERS__ */', renderImportLoaders(versions, 'tfhe_bg.wasm.base64.js')],
      ['  /* __TFHE_ASSETS__ */', renderAssetMap(versions, getTfheAssets)],
    ]),
  );
}

////////////////////////////////////////////////////////////////////////////////
// Generator: TFHE API declaration source
////////////////////////////////////////////////////////////////////////////////

/**
 * @param {readonly string[]} versions
 * @returns {string} TS declaration source for src/wasm/tfhe/TfheApi.d.ts
 */
export function generateTfheApiSource(versions) {
  return renderApiSource({
    versions,
    label: 'TFHE',
    templatePath: TFHE_API_TEMPLATE_PATH,
    versionTypeName: 'TfheVersion',
    outputImportFile: 'tfhe.js',
  });
}

////////////////////////////////////////////////////////////////////////////////
// Generator: KMS loader source
////////////////////////////////////////////////////////////////////////////////

/**
 * @param {readonly string[]} versions
 * @param {string} profile
 * @param {string} defaultVersion
 * @returns {string} JS source for src/wasm/tkms/loadKmsLib.js
 */
export function generateKmsLoaderSource(versions, profile, defaultVersion = versions[0]) {
  return renderTemplate(
    resolve(TEMPLATES_DIR, 'loadKmsLib.template.js'),
    new Map([
      [BUILD_PROFILE_PLACEHOLDER, profile],
      ['__KMS_VERSIONS__', renderVersionList(versions)],
      ['__TKMS_DEFAULT_VERSION__', renderDefaultVersion(versions, 'TKMS', defaultVersion)],
      ['  /* __KMS_LIB_LOADERS__ */', renderImportLoaders(versions, 'kms_lib.js')],
      ['  /* __KMS_WASM_BASE64_LOADERS__ */', renderImportLoaders(versions, 'kms_lib_bg.wasm.base64.js')],
      ['  /* __KMS_ASSETS__ */', renderAssetMap(versions, getKmsAssets)],
    ]),
  );
}

////////////////////////////////////////////////////////////////////////////////
// Generator: KMS API declaration source
////////////////////////////////////////////////////////////////////////////////

/**
 * @param {readonly string[]} versions
 * @returns {string} TS declaration source for src/wasm/tkms/KmsLibApi.d.ts
 */
export function generateKmsApiSource(versions) {
  return renderApiSource({
    versions,
    label: 'KMS',
    templatePath: KMS_API_TEMPLATE_PATH,
    versionTypeName: 'TkmsVersion',
    outputImportFile: 'kms_lib.js',
  });
}

////////////////////////////////////////////////////////////////////////////////
// CLI main
////////////////////////////////////////////////////////////////////////////////

const isMain = import.meta.url === `file://${process.argv[1]}`;
if (isMain) {
  const args = process.argv.slice(2);
  const check = args.includes('--check');
  const unknownArgs = args.filter((arg) => arg !== '--check');
  if (unknownArgs.length > 0) {
    console.error(`[codegen-loaders] unknown argument(s): ${unknownArgs.join(', ')}`);
    process.exit(1);
  }

  const profile = process.env.BUILD_PROFILE ?? 'dev';
  if (!BUILD_PROFILES.includes(profile)) {
    console.error(
      `[codegen-loaders] unknown BUILD_PROFILE='${profile}'. Expected one of: ${BUILD_PROFILES.join(', ')}`,
    );
    process.exit(1);
  }

  const tfheVersions = TFHE_MANIFEST.filter((entry) => entry.tags.includes(profile)).map((entry) => entry.version);
  const kmsVersions = KMS_MANIFEST.filter((entry) => entry.tags.includes(profile)).map((entry) => entry.version);
  const defaultVersions = resolveDefaultWasmVersions(profile);
  const tfheApiAnchorVersion = getTfheApiAnchorVersion();
  const kmsApiAnchorVersion = getKmsApiAnchorVersion();

  const generatedFiles = [
    {
      path: resolve(SRC_WASM, 'tfhe/loadTfheLib.js'),
      rel: 'src/wasm/tfhe/loadTfheLib.js',
      source: generateTfheLoaderSource(tfheVersions, profile, defaultVersions.tfhe),
    },
    {
      path: resolve(SRC_WASM, 'tfhe/TfheApi.d.ts'),
      rel: 'src/wasm/tfhe/TfheApi.d.ts',
      source: generateTfheApiSource(tfheVersions),
    },
    {
      path: resolve(SRC_WASM, 'tkms/loadKmsLib.js'),
      rel: 'src/wasm/tkms/loadKmsLib.js',
      source: generateKmsLoaderSource(kmsVersions, profile, defaultVersions.tkms),
    },
    {
      path: resolve(SRC_WASM, 'tkms/KmsLibApi.d.ts'),
      rel: 'src/wasm/tkms/KmsLibApi.d.ts',
      source: generateKmsApiSource(kmsVersions),
    },
  ];

  if (check) {
    const staleFiles = generatedFiles.filter(
      ({ path, source }) => !existsSync(path) || readFileSync(path, 'utf8') !== source,
    );
    if (staleFiles.length > 0) {
      console.error('[codegen-loaders] generated loader files are stale:');
      for (const file of staleFiles) {
        console.error(`[codegen-loaders]   ${file.rel}`);
      }
      console.error('[codegen-loaders] run `npm run codegen:loaders` to regenerate them.');
      process.exit(1);
    }
  } else {
    for (const file of generatedFiles) {
      writeFileSync(file.path, file.source);
    }
  }

  console.log(`[codegen-loaders] ${check ? 'checked' : 'generated'} profile=${profile}`);
  console.log(`[codegen-loaders]   TFHE versions:   ${renderDisplayVersionList(tfheVersions) || '(none)'}`);
  console.log(`[codegen-loaders]   TFHE API anchor: v${tfheApiAnchorVersion}`);
  console.log(`[codegen-loaders]   KMS versions:    ${renderDisplayVersionList(kmsVersions) || '(none)'}`);
  console.log(`[codegen-loaders]   KMS API anchor:  v${kmsApiAnchorVersion}`);
}

// Regenerates src/core/key/fheEncryptionKeyDigestsWorkerCode-p.ts: the code
// the FHE key / CRS digests run in an isomorphic worker
// (runCodeInIsomorphicWorker), bundled with vite from
// src/core/key/fheEncryptionKeyDigests-p.ts and the installed @noble/hashes.
//
// Usage (the generated file is committed to git):
//   node scripts/build/codegen-key-digests-worker.mjs         # writes the generated file
//   node scripts/build/codegen-key-digests-worker.mjs --check # verifies the committed file is fresh
//
// `--check` does not bundle: the bundler's exact output may change with a
// vite / esbuild upgrade without changing what the code computes. The
// generated file records its inputs (the @noble/hashes version, and a sha256
// of the bundled noble files and the entry); `--check` recomputes that hash
// from the installed files. It also records a sha256 of the generated code
// string, which `--check` recomputes from the committed string. The unit
// tests check that the committed code is self-contained, keeps noble's
// notice and returns noble's digests.
//
// The worker code is a self-contained script: no import, no `import()`, no
// `node:` specifier, no `import.meta` (zama-sdk bundles its own worker as a
// single IIFE). It keeps noble's MIT notice.

import { createHash } from 'node:crypto';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, isAbsolute, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { runInNewContext } from 'node:vm';
import * as prettier from 'prettier';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, '../..');
const ENTRY_PATH = resolve(ROOT, 'src/core/key/fheEncryptionKeyDigests-p.ts');
const OUTPUT_PATH = resolve(ROOT, 'src/core/key/fheEncryptionKeyDigestsWorkerCode-p.ts');
const PUBLISHED_PACKAGE_JSON_PATH = resolve(ROOT, 'src/package.json');

// The IIFE's global, local to the worker function body.
const BUNDLE_NAME = 'fheEncryptionKeyDigests';
const ENTRY_FUNCTION = 'computeFheEncryptionKeyDigests';

// Keep in sync with the 'is a self-contained script' test in fheEncryptionKeyDigests-p.test.ts.
const FORBIDDEN_PATTERNS = [
  { re: /\bimport\s*\(/, label: 'import()' },
  { re: /\bimport\s*[\s{*'"]/, label: 'import statement' },
  { re: /\bimport\.meta\b/, label: 'import.meta' },
  { re: /\brequire\s*\(/, label: 'require()' },
  { re: /['"`]node:/, label: 'node: specifier' },
];

// The committed code string: a single string literal, so evaluating it runs nothing.
const CODE_RE =
  /^export const FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE: string =\s*('(?:[^'\\\n]|\\.)*'|"(?:[^"\\\n]|\\.)*");$/m;

function fail(message) {
  throw new Error(message);
}

function readNoble() {
  const require = createRequire(ENTRY_PATH);
  // './package.json' is not exported: sha3.js sits at the package root.
  const packageDir = dirname(require.resolve('@noble/hashes/sha3.js'));
  const { version } = JSON.parse(readFileSync(resolve(packageDir, 'package.json'), 'utf8'));
  const license = readFileSync(resolve(packageDir, 'LICENSE'), 'utf8').trim();

  // The calling-thread fallback runs the published dependency: it must be the
  // version the worker code is bundled from.
  const published = JSON.parse(readFileSync(PUBLISHED_PACKAGE_JSON_PATH, 'utf8')).dependencies?.['@noble/hashes'];
  if (published !== version) {
    fail(`@noble/hashes is installed at ${version}, but src/package.json depends on ${String(published)}.`);
  }

  return { packageDir, version, license };
}

/**
 * sha256 of the inputs: the entry and the bundled noble files (named relative
 * to the noble package).
 *
 * @param {string} packageDir
 * @param {readonly string[]} nobleFiles
 */
function inputsHash(packageDir, nobleFiles) {
  const hash = createHash('sha256');
  const files = [
    [relative(dirname(OUTPUT_PATH), ENTRY_PATH), ENTRY_PATH],
    ...nobleFiles.map((file) => [`@noble/hashes/${file}`, resolve(packageDir, file)]),
  ];
  for (const [name, path] of files) {
    if (!existsSync(path)) {
      fail(`Missing input ${name}.`);
    }
    hash.update(`${name}\0`).update(readFileSync(path)).update('\0');
  }
  return hash.digest('hex');
}

/** @param {string} code */
function codeHash(code) {
  return createHash('sha256').update(code).digest('hex');
}

/** @returns {string | undefined} the first forbidden construct in `code` */
function forbiddenIn(code) {
  return FORBIDDEN_PATTERNS.find(({ re }) => re.test(code))?.label;
}

/** @returns {Promise<{ code: string, moduleIds: string[] }>} */
async function bundle() {
  // Not needed by `--check`.
  const { build } = await import('vite');
  const output = await build({
    configFile: false,
    logLevel: 'silent',
    publicDir: false,
    root: ROOT,
    build: {
      write: false,
      minify: 'esbuild',
      target: 'es2021',
      lib: { entry: ENTRY_PATH, formats: ['iife'], name: BUNDLE_NAME, fileName: BUNDLE_NAME },
    },
  });
  const chunks = (Array.isArray(output) ? output : [output]).flatMap((o) => o.output);
  if (chunks.length !== 1 || chunks[0].type !== 'chunk') {
    fail(`Expected a single bundled chunk, got ${chunks.length} output file(s).`);
  }
  return { code: chunks[0].code.trim(), moduleIds: chunks[0].moduleIds };
}

/**
 * @returns {Promise<string>} TS source for src/core/key/fheEncryptionKeyDigestsWorkerCode-p.ts
 */
async function generateKeyDigestsWorkerCodeSource() {
  const noble = readNoble();
  const { code: bundled, moduleIds } = await bundle();
  const forbidden = forbiddenIn(bundled);
  if (forbidden !== undefined) {
    fail(`The bundled worker code contains a forbidden ${forbidden}.`);
  }

  const nobleFiles = moduleIds
    .filter((id) => id !== ENTRY_PATH)
    .map((id) => {
      const file = relative(noble.packageDir, id);
      if (file.startsWith('..') || isAbsolute(file)) {
        fail(`Unexpected bundled module ${id}: only the entry and @noble/hashes may be bundled.`);
      }
      return file.split(sep).join('/');
    })
    .sort();

  const workerCode = [
    `/*! @noble/hashes ${noble.version}, bundled from https://github.com/paulmillr/noble-hashes`,
    '',
    noble.license,
    '*/',
    bundled,
    `return ${BUNDLE_NAME}.${ENTRY_FUNCTION}(data);`,
  ].join('\n');

  const entry = relative(dirname(OUTPUT_PATH), ENTRY_PATH);
  const source = `/**
 * Generated by scripts/build/codegen-key-digests-worker.mjs from ./${entry}
 * and @noble/hashes ${noble.version} (MIT, notice kept in the code): do not edit.
 * Regenerate with \`npm run codegen:key-digests\`.
 *
 * Inputs, checked by \`--check\` instead of rebundling:
 *   @noble/hashes ${noble.version}: ${nobleFiles.join(', ')}
 *   sha256: ${inputsHash(noble.packageDir, nobleFiles)}
 *
 * Code, checked by \`--check\` against a tampered or stale bundle (supply-chain check):
 *   sha256: ${codeHash(workerCode)}
 *
 * The body of a \`runCodeInIsomorphicWorker\` function: receives
 * \`{ publicKey, crs }\` bytes as \`data\`, returns their 32-byte digests.
 */
export const FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE: string = ${JSON.stringify(workerCode)};
`;

  const options = await prettier.resolveConfig(OUTPUT_PATH);
  return prettier.format(source, { ...options, filepath: OUTPUT_PATH });
}

/**
 * Checks the committed file against the installed inputs and its recorded
 * code hash, without bundling.
 *
 * @param {string} source the committed file
 * @returns {string | undefined} why the file is stale, if it is
 */
export function checkKeyDigestsWorkerCodeSource(source) {
  const noble = readNoble();

  const inputs = /^ \* {3}@noble\/hashes (\S+): (.+)\n \* {3}sha256: ([0-9a-f]{64})$/m.exec(source);
  if (inputs === null) {
    return 'its inputs are not recorded';
  }
  const [, version, files, hash] = inputs;
  if (version !== noble.version) {
    return `it is bundled from @noble/hashes ${version}, ${noble.version} is installed`;
  }
  if (hash !== inputsHash(noble.packageDir, files.split(', '))) {
    return 'its inputs changed';
  }

  const recorded = /^ \* Code, .*\n \* {3}sha256: ([0-9a-f]{64})$/m.exec(source);
  const code = CODE_RE.exec(source);
  if (recorded === null || code === null) {
    return 'its code hash is not recorded';
  }
  if (recorded[1] !== codeHash(runInNewContext(code[1]))) {
    return 'its code does not match its recorded hash';
  }
  return undefined;
}

////////////////////////////////////////////////////////////////////////////////
// CLI main
////////////////////////////////////////////////////////////////////////////////

async function main() {
  const args = process.argv.slice(2);
  const check = args.includes('--check');
  const unknownArgs = args.filter((arg) => arg !== '--check');
  if (unknownArgs.length > 0) {
    console.error(`[codegen-key-digests] unknown argument(s): ${unknownArgs.join(', ')}`);
    process.exit(1);
  }

  const rel = relative(ROOT, OUTPUT_PATH);

  if (check) {
    const stale = existsSync(OUTPUT_PATH)
      ? checkKeyDigestsWorkerCodeSource(readFileSync(OUTPUT_PATH, 'utf8'))
      : 'it is missing';
    if (stale !== undefined) {
      console.error(`[codegen-key-digests] ${rel} is stale: ${stale}.`);
      console.error('[codegen-key-digests] run `npm run codegen:key-digests` to regenerate it.');
      process.exit(1);
    }
  } else {
    writeFileSync(OUTPUT_PATH, await generateKeyDigestsWorkerCodeSource());
  }

  console.log(`[codegen-key-digests] ${check ? 'checked' : 'generated'} ${rel}`);
}

// Run as a script, not when imported by the unit tests.
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}

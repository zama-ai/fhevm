#!/usr/bin/env node
//
// Generates lib/cryptography/FhevmECDSA.sol from OpenZeppelin's ECDSA.sol, or checks that it is in sync.
//
// FhevmECDSA is OpenZeppelin's ECDSA library, renamed so that it does not clash with ECDSA when a project imports
// both. The OpenZeppelin source is pinned in publish/fhevm-ecdsa.json: `ref` is a tag or a commit hash, and `sha256`
// is the hash of ECDSA.sol at that ref. The only changes made to it are:
//   - `library ECDSA {` becomes `library FhevmECDSA {`;
//   - a @dev note saying that the library is a renamed fork is added to the library documentation.
// Anything else in FhevmECDSA.sol must come from OpenZeppelin unchanged.
//
// The codegen (`npm run codegen`) also writes FhevmECDSA.sol, from codegen/src/templates/FhevmECDSA.sol-template:
// `generate` writes both files with the same content, and `check` fails if they differ.
//
// Usage (from library-solidity/):
//   node publish/fhevm-ecdsa.mjs check                  checks, offline, that FhevmECDSA.sol is the pinned
//                                                       source (`sha256`) with the FhevmECDSA changes, and
//                                                       that the codegen template is identical to it
//   node publish/fhevm-ecdsa.mjs check --online         also downloads the pinned ref and checks that its
//                                                       hash is `sha256`
//   node publish/fhevm-ecdsa.mjs generate               regenerates FhevmECDSA.sol and the codegen template
//                                                       from the pinned ref
//   node publish/fhevm-ecdsa.mjs generate --ref <ref>   regenerates it from <ref> and pins <ref> in
//                                                       publish/fhevm-ecdsa.json
//   --source <file>                                     uses a local copy of ECDSA.sol instead of downloading it
//
// `generate` records the `sha256` of the source it used. Node built-ins only. The source is downloaded from
// raw.githubusercontent.com unless --source is given.

import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { parseArgs } from 'node:util';
import { fileURLToPath } from 'node:url';

const packageDir = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const configFile = join(packageDir, 'publish', 'fhevm-ecdsa.json');
const targetFile = join(packageDir, 'lib', 'cryptography', 'FhevmECDSA.sol');
// The codegen copies this template to FhevmECDSA.sol (it has no `//$$` template comments to remove).
const templateFile = join(packageDir, 'codegen', 'src', 'templates', 'FhevmECDSA.sol-template');

const LIBRARY_LINE = '\nlibrary ECDSA {\n';
const FORKED_LIBRARY_LINE = '\nlibrary FhevmECDSA {\n';
// The fork note goes after this line of the library documentation, before the closing ` */`.
const NOTE_ANCHOR = ' * of the private keys of a given address.\n';
const FORK_NOTE =
  ' *\n' +
  " * @dev This library is forked from OpenZeppelin's ECDSA and renamed to FhevmECDSA\n" +
  ' * to avoid naming conflicts with the original when both are used in the same project.\n';

function fail(message) {
  console.error(`fhevm-ecdsa: ${message}`);
  process.exit(1);
}

// Returns the number of occurrences of `needle` in `text`.
function count(text, needle) {
  return text.split(needle).length - 1;
}

function sha256(text) {
  return createHash('sha256').update(text).digest('hex');
}

// Replaces the single occurrence of `from` (described by `name`) in `text` (described by `where`) by `to`. Fails,
// instead of guessing, if `from` does not occur exactly once: the transformation then needs a review.
function replaceOnce(text, from, to, name, where) {
  const occurrences = count(text, from);
  if (occurrences !== 1) {
    fail(`expected exactly one ${name} in ${where}, found ${occurrences}`);
  }
  return text.replace(from, () => to);
}

// Applies the FhevmECDSA changes to OpenZeppelin's ECDSA.sol.
function toFhevmECDSA(source) {
  const where = 'the OpenZeppelin source';
  const renamed = replaceOnce(source, LIBRARY_LINE, FORKED_LIBRARY_LINE, `"${LIBRARY_LINE.trim()}" line`, where);
  return replaceOnce(renamed, NOTE_ANCHOR, NOTE_ANCHOR + FORK_NOTE, `"${NOTE_ANCHOR.trim()}" line`, where);
}

// Reverts the FhevmECDSA changes, giving back OpenZeppelin's ECDSA.sol if FhevmECDSA.sol has no other change.
function fromFhevmECDSA(fhevmSource, target) {
  const reverted = replaceOnce(
    fhevmSource,
    FORKED_LIBRARY_LINE,
    LIBRARY_LINE,
    `"${FORKED_LIBRARY_LINE.trim()}" line`,
    target,
  );
  return replaceOnce(
    reverted,
    NOTE_ANCHOR + FORK_NOTE,
    NOTE_ANCHOR,
    'fork note (after the library documentation)',
    target,
  );
}

async function readSource(config, ref, sourceFile) {
  if (sourceFile !== undefined) {
    return readFileSync(sourceFile, 'utf8');
  }
  const url = `https://raw.githubusercontent.com/${config.repository}/${ref}/${config.path}`;
  let response;
  try {
    response = await fetch(url);
  } catch (error) {
    fail(`cannot download ${url}: ${error.message}`);
  }
  if (!response.ok) {
    fail(`cannot download ${url}: HTTP ${response.status} (is "${ref}" a tag or commit of ${config.repository}?)`);
  }
  return response.text();
}

// Describes the first line where `actual` and `expected` differ.
function firstDifference(actual, expected) {
  const actualLines = actual.split('\n');
  const expectedLines = expected.split('\n');
  for (let i = 0; i < Math.max(actualLines.length, expectedLines.length); i++) {
    if (actualLines[i] !== expectedLines[i]) {
      return (
        `first difference at line ${i + 1}:\n` +
        `  file:     ${JSON.stringify(actualLines[i] ?? '<end of file>')}\n` +
        `  expected: ${JSON.stringify(expectedLines[i] ?? '<end of file>')}`
      );
    }
  }
  return 'the files differ';
}

const USAGE =
  'usage: node publish/fhevm-ecdsa.mjs check [--online] [--source <ECDSA.sol>]\n' +
  '       node publish/fhevm-ecdsa.mjs generate [--ref <tag or commit>] [--source <ECDSA.sol>]';

let parsed;
try {
  parsed = parseArgs({
    allowPositionals: true,
    options: { online: { type: 'boolean' }, ref: { type: 'string' }, source: { type: 'string' } },
  });
} catch (error) {
  fail(`${error.message}\n${USAGE}`);
}
const { positionals, values } = parsed;
const command = positionals[0];
if (
  positionals.length !== 1 ||
  (command !== 'check' && command !== 'generate') ||
  (command === 'check' && values.ref !== undefined) ||
  (command === 'generate' && values.online !== undefined)
) {
  fail(USAGE);
}

const config = JSON.parse(readFileSync(configFile, 'utf8'));
const target = relative(packageDir, targetFile);
const template = relative(packageDir, templateFile);
const configName = relative(packageDir, configFile);
const regenerateHint = 'Run `node publish/fhevm-ecdsa.mjs generate` to regenerate it.';

if (command === 'check') {
  const actual = readFileSync(targetFile, 'utf8');
  if (typeof config.sha256 !== 'string') {
    fail(`${configName} has no "sha256". ${regenerateHint}`);
  }

  // Offline: FhevmECDSA.sol, without the FhevmECDSA changes, must be the source whose hash is pinned.
  const pinned = `${config.repository}@${config.ref} (sha256 ${config.sha256})`;
  if (sha256(fromFhevmECDSA(actual, target)) !== config.sha256) {
    fail(`${target} is not in sync with ${pinned}: it has changes other than the FhevmECDSA ones.\n${regenerateHint}`);
  }

  // The codegen template must be identical, or `npm run codegen` would overwrite FhevmECDSA.sol with it.
  const templateSource = readFileSync(templateFile, 'utf8');
  if (templateSource !== actual) {
    fail(`${template} differs from ${target}, ${firstDifference(templateSource, actual)}\n${regenerateHint}`);
  }

  // Online, or with --source: the downloaded or given source must be the pinned one.
  if (values.online || values.source !== undefined) {
    const source = await readSource(config, config.ref, values.source);
    const origin = values.source ?? `${config.repository}@${config.ref}`;
    if (sha256(source) !== config.sha256) {
      const expected = toFhevmECDSA(source);
      fail(
        `${origin} does not have the pinned sha256 ${config.sha256}` +
          (actual === expected ? '.' : `, ${firstDifference(actual, expected)}`) +
          `\n${regenerateHint}`,
      );
    }
    console.log(`fhevm-ecdsa: ok (${target} is in sync with ${origin}, sha256 ${config.sha256})`);
  } else {
    console.log(`fhevm-ecdsa: ok (${target} is in sync with ${pinned})`);
  }
} else {
  const ref = values.ref ?? config.ref;
  const source = await readSource(config, ref, values.source);
  const generated = toFhevmECDSA(source);
  writeFileSync(targetFile, generated);
  writeFileSync(templateFile, generated);
  const newConfig = { ...config, ref, sha256: sha256(source) };
  if (JSON.stringify(newConfig) !== JSON.stringify(config)) {
    writeFileSync(configFile, `${JSON.stringify(newConfig, null, 2)}\n`);
    console.log(`fhevm-ecdsa: pinned ${config.repository}@${ref} (sha256 ${newConfig.sha256}) in ${configName}`);
  }
  console.log(
    `fhevm-ecdsa: generated ${target} and ${template} from ${values.source ?? `${config.repository}@${ref}`}`,
  );
}

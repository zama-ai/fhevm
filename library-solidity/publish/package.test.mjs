// Checks that the package published by CI (package.json.publish.template) ships exactly
// what a standard `npm pack` of library-solidity ships, with a cleaned-up package.json.
//
// Builds two independent tarballs:
// - standard: `npm pack` of library-solidity/ (package.json + .npmignore);
// - template: the staging done by .github/workflows/library-solidity-publish.yml
//   (copy lib/, config/, README.md, LICENSE, package.json from the template), then `npm pack`.
//   If PUBLISH_TARBALL is set, the tarball at that path is checked instead: CI passes the
//   tarball it is about to publish.
//
// Usage (from library-solidity/): node --test 'publish/*.test.mjs'
//                                 PUBLISH_TARBALL=<path.tgz> node --test 'publish/*.test.mjs'
//
// Node built-ins only, never node_modules. Lifecycle scripts are disabled while packing.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { cpSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, relative, resolve } from 'node:path';
import { after, before, test } from 'node:test';
import { fileURLToPath } from 'node:url';

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '..');

// Entries removed from the published package.json.
// `type` only tells Node how to load the Hardhat tooling (ESM); the published package ships Solidity files only.
const CLEANED_UP = ['scripts', 'devDependencies', 'optionalDependencies', 'engines', 'main', 'type'];
// Files both tarballs must contain.
const REQUIRED_FILES = ['package.json', 'README.md', 'LICENSE'];
// Entries that only exist in the published package.json.
const PUBLISH_ONLY = ['files', 'keywords'];
// The only entries accepted in the published package.json. Anything else (scripts,
// *Dependencies other than `dependencies`, bin, exports, ...) fails the test.
const ALLOWED = [
  'name',
  'description',
  'version',
  'files',
  'repository',
  'keywords',
  'author',
  'license',
  'bugs',
  'homepage',
  'dependencies',
];

let work;
let standard;
let template;

// Extracts `tarball` in `<work>/<name>`. Returns the extracted `package/` directory.
function extract(tarball, name) {
  const extracted = join(work, name);
  mkdirSync(extracted);
  execFileSync('tar', ['-xzf', tarball, '-C', extracted]);
  return join(extracted, 'package');
}

// Runs `npm pack` in `cwd` and extracts the tarball in `<work>/<name>`. Returns the extracted
// `package/` directory.
function packAndExtract(cwd, name) {
  const dest = join(work, `${name}-tgz`);
  mkdirSync(dest);
  const out = execFileSync('npm', ['pack', '--ignore-scripts', '--json', '--pack-destination', dest], {
    cwd,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  return extract(join(dest, JSON.parse(out)[0].filename), name);
}

// Mirrors the "Stage package" step of library-solidity-publish.yml.
function stageFromTemplate() {
  const stage = join(work, 'stage');
  mkdirSync(stage);
  for (const source of ['lib', 'config', 'README.md', 'LICENSE']) {
    cpSync(join(ROOT, source), join(stage, source), { recursive: true });
  }
  const { version } = JSON.parse(readFileSync(join(ROOT, 'package.json'), 'utf8'));
  const manifest = JSON.parse(readFileSync(join(ROOT, 'package.json.publish.template'), 'utf8'));
  manifest.version = version;
  writeFileSync(join(stage, 'package.json'), JSON.stringify(manifest, null, 2) + '\n');
  return stage;
}

// Relative paths of all entries under `dir`, sorted. Fails on anything but regular files.
function listFiles(dir) {
  const files = [];
  const walk = (current) => {
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      const path = join(current, entry.name);
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile()) files.push(relative(dir, path));
      else assert.fail(`not a regular file: ${relative(dir, path)}`);
    }
  };
  walk(dir);
  return files.sort();
}

function readManifest(dir) {
  return JSON.parse(readFileSync(join(dir, 'package.json'), 'utf8'));
}

function omit(object, keys) {
  return Object.fromEntries(Object.entries(object).filter(([key]) => !keys.includes(key)));
}

before(() => {
  work = mkdtempSync(join(tmpdir(), 'fhevm-solidity-pack-'));
  standard = packAndExtract(ROOT, 'standard');
  const tarball = process.env.PUBLISH_TARBALL;
  template = tarball ? extract(resolve(tarball), 'template') : packAndExtract(stageFromTemplate(), 'template');
});

after(() => {
  if (work) rmSync(work, { recursive: true, force: true });
});

test('both tarballs contain the same files', () => {
  const standardFiles = listFiles(standard);
  const templateFiles = listFiles(template);
  assert.ok(
    standardFiles.some((file) => file.endsWith('.sol')),
    'no Solidity file in the standard tarball',
  );
  assert.deepEqual(
    templateFiles,
    standardFiles,
    `only in standard: ${standardFiles.filter((f) => !templateFiles.includes(f)).join(', ') || '-'}; ` +
      `only in template: ${templateFiles.filter((f) => !standardFiles.includes(f)).join(', ') || '-'}`,
  );
});

test('every file except package.json is byte-identical', () => {
  // Files present in only one tarball are reported by the previous test.
  const templateFiles = listFiles(template);
  for (const file of listFiles(standard)) {
    if (file === 'package.json' || !templateFiles.includes(file)) continue;
    const a = join(standard, file);
    const b = join(template, file);
    assert.ok(readFileSync(a).equals(readFileSync(b)), `content differs: ${file}`);
    assert.equal(lstatSync(b).mode, lstatSync(a).mode, `mode differs: ${file}`);
  }
});

test('both tarballs contain package.json, README.md and LICENSE', () => {
  for (const dir of [standard, template]) {
    const files = listFiles(dir);
    for (const file of REQUIRED_FILES) {
      assert.ok(files.includes(file), `${file} is missing from ${relative(work, dir)}`);
    }
  }
});

test('no bridge contract is published', () => {
  for (const dir of [standard, template]) {
    const bridge = listFiles(dir).filter((file) => file.startsWith('lib/bridge/'));
    assert.deepEqual(bridge, [], `bridge contracts in ${relative(work, dir)}`);
  }
});

// True if Foundry (`forge`) is installed: the Soldeer test is skipped otherwise.
function hasForge() {
  try {
    execFileSync('forge', ['--version'], { stdio: 'ignore' });
    return true;
  } catch {
    return false;
  }
}

// Mirrors the "Publish to Soldeer" step of library-solidity-publish.yml, in dry-run mode: copies the
// git-tracked files (a clean checkout) to `<work>/soldeer/library-solidity`, runs
// `forge soldeer push --dry-run` there and returns the paths in the zip it creates, sorted.
function soldeerDryRunFiles() {
  const checkout = join(work, 'soldeer', 'library-solidity');
  const tracked = execFileSync('git', ['ls-files', '-z'], { cwd: ROOT, encoding: 'utf8' }).split('\0').filter(Boolean);
  for (const file of tracked) {
    mkdirSync(dirname(join(checkout, file)), { recursive: true });
    cpSync(join(ROOT, file), join(checkout, file));
  }
  execFileSync('forge', ['soldeer', 'push', '@fhevm-solidity~0.0.0', '.', '--skip-warnings', '--dry-run'], {
    cwd: checkout,
    stdio: 'ignore',
  });
  return execFileSync('unzip', ['-Z1', join(checkout, 'library-solidity.zip')], { encoding: 'utf8' })
    .split('\n')
    .filter((entry) => entry && !entry.endsWith('/'))
    .sort();
}

// Skipped locally without Foundry, never on CI (GitHub Actions sets CI=true): the CI jobs that run
// this test install Foundry, and a missing forge must fail there instead of passing silently.
test(
  'the Soldeer package contains the npm package files, except package.json',
  { skip: hasForge() || process.env.CI ? false : 'forge not found (install Foundry)' },
  () => {
    const soldeerFiles = soldeerDryRunFiles();
    const npmFiles = listFiles(standard).filter((file) => file !== 'package.json');
    assert.deepEqual(
      soldeerFiles,
      npmFiles,
      `only in Soldeer: ${soldeerFiles.filter((f) => !npmFiles.includes(f)).join(', ') || '-'}; ` +
        `only in npm: ${npmFiles.filter((f) => !soldeerFiles.includes(f)).join(', ') || '-'} ` +
        '(check .soldeerignore)',
    );
  },
);

test('published package.json only contains accepted entries', () => {
  const unexpected = Object.keys(readManifest(template)).filter((key) => !ALLOWED.includes(key));
  assert.deepEqual(unexpected, [], `entries not accepted in the published package.json: ${unexpected.join(', ')}`);
});

test('package.json files are equivalent, except the cleaned-up entries', () => {
  const standardManifest = readManifest(standard);
  const templateManifest = readManifest(template);

  for (const key of CLEANED_UP) {
    assert.ok(!(key in templateManifest), `"${key}" must not be published`);
  }
  for (const key of PUBLISH_ONLY) {
    assert.ok(key in templateManifest, `"${key}" is missing from the published package.json`);
  }
  // Every other entry (name, version, dependencies, license, ...) must be identical.
  assert.deepEqual(omit(templateManifest, PUBLISH_ONLY), omit(standardManifest, CLEANED_UP));
});

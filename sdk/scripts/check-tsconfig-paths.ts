#!/usr/bin/env node
//
// Every path a tsconfig names by hand must exist. A stale one is invisible: TypeScript ignores an
// `include` entry that matches nothing as long as another entry matches, and an `exclude` for a deleted
// file is simply inert — so a project can quietly check less than it claims, for months.
//
// This gate found eight such entries the first time it was run, across two generations: a `test/ts`
// exclude naming a file that only exists in the other generation, three `include` entries for utils that
// had moved into @fhevm/sdk-common-dev, and four excludes for files deleted in a refactor.
//
// Usage: ./scripts/check-tsconfig-paths.ts [--verbose]
//   --verbose   list every path inspected, not just the failures
//
// Run from anywhere: the workspace root is found via `npm prefix`, falling back to this file's parent.
//
// What is checked: `include`, `exclude`, `files`, `references[].path`, and a relative `extends`. A pattern
// containing `*` or `?` is skipped — a glob that matches nothing is legitimate (a directory that is empty
// today), and only a literal path is a claim about a file that should be there.
//
// SCOPE: the root's own tsconfigs, every declared workspace member, and scripts/ — this workspace's own
// tooling. NOT the whole tree. `sdk/js-sdk` is a member of the OUTER repo workspace, so its tsconfigs are
// not this gate's to enforce, exactly as check-dep-versions.ts scopes itself. Deriving the roots from
// `workspaces` rather than walking everything is also what makes a new member covered with no edit here.
//
// INTENTIONAL is the one allowlist: build-output names that may or may not exist at any moment, which is
// the point of excluding them.

import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

/** Build-output and toolchain names that legitimately may not exist. Excluding them is the point. */
const INTENTIONAL = new Set([
  'node_modules', // specifying `exclude` replaces tsc's default list, so it must be named again
  'artifacts',
  'cache',
  'typechain',
  'typechain-types',
  'out',
  'dependencies',
  'tarballs',
]);

/** This workspace's own tooling: not a member, but its tsconfig is ours. */
const OWNED_DIRS = ['scripts'];

const SKIP_DIRS = new Set(['node_modules', '_cjs', '_esm', '_types', '.next', 'out', 'cache', 'dependencies']);

type TsConfig = {
  readonly extends?: string | readonly string[];
  readonly include?: readonly string[];
  readonly exclude?: readonly string[];
  readonly files?: readonly string[];
  readonly references?: readonly { readonly path?: string }[];
};

/** The leading comment block doubles as the help text, so there is only one copy of it. */
function usage(): void {
  const source = readFileSync(fileURLToPath(import.meta.url), 'utf8');
  for (const line of source.split('\n').slice(1)) {
    if (!line.startsWith('//')) break;
    console.log(line.replace(/^\/\/ ?/, ''));
  }
}

function parseArgs(argv: readonly string[]): { verbose: boolean } {
  let verbose = false;
  for (const arg of argv) {
    if (arg === '--verbose') verbose = true;
    else if (arg === '-h' || arg === '--help') {
      usage();
      process.exit(0);
    } else {
      console.error(`Error: unknown argument '${arg}'. Try --help.`);
      process.exit(1);
    }
  }
  return { verbose };
}

function workspaceRoot(): string {
  try {
    const prefix = execFileSync('npm', ['prefix'], { encoding: 'utf8' }).trim();
    if (prefix && existsSync(join(prefix, 'package.json'))) {
      const manifest = JSON.parse(readFileSync(join(prefix, 'package.json'), 'utf8')) as {
        workspaces?: unknown;
      };
      if (manifest.workspaces !== undefined) return prefix;
    }
  } catch {
    // npm is not on PATH, or we are outside a package — fall through to the path-based guess.
  }
  return dirname(dirname(fileURLToPath(import.meta.url)));
}

/**
 * JSON with comments and trailing commas, which is what a tsconfig actually is.
 *
 * Written out rather than pulled from a dependency: sdk/scripts has no manifest of its own beyond a
 * module-type marker, and a gate that runs in `check` should not need an install to work.
 */
function parseJsonc(text: string): unknown {
  let out = '';
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c === '"') {
      let j = i + 1;
      while (j < text.length) {
        if (text[j] === '\\') {
          j += 2;
          continue;
        }
        if (text[j] === '"') break;
        j++;
      }
      out += text.slice(i, j + 1);
      i = j;
    } else if (c === '/' && text[i + 1] === '/') {
      while (i < text.length && text[i] !== '\n') i++;
      out += '\n';
    } else if (c === '/' && text[i + 1] === '*') {
      const end = text.indexOf('*/', i + 2);
      i = end === -1 ? text.length : end + 1;
    } else {
      out += c;
    }
  }
  return JSON.parse(out.replace(/,(\s*[}\]])/g, '$1'));
}

function tsconfigsUnder(dir: string, found: string[] = [], recurse = true): string[] {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (recurse && !SKIP_DIRS.has(entry.name)) tsconfigsUnder(join(dir, entry.name), found);
    } else if (/^tsconfig.*\.json$/.test(entry.name)) {
      found.push(join(dir, entry.name));
    }
  }
  return found;
}

/**
 * The tsconfigs this workspace owns: its own, its members', and its tooling's.
 *
 * The root is read NON-recursively — walking it would sweep up sibling packages that belong to the outer
 * repo workspace, which this gate does not govern.
 */
function ownedTsconfigs(members: readonly string[]): string[] {
  const found = tsconfigsUnder('.', [], false);
  for (const dir of [...members, ...OWNED_DIRS]) {
    if (existsSync(dir)) tsconfigsUnder(dir, found);
  }
  return [...new Set(found)].sort();
}

/** A `references[].path` may name a directory, a file, or a directory holding tsconfig.json. */
function referenceExists(target: string): boolean {
  return existsSync(target) || existsSync(`${target}.json`) || existsSync(join(target, 'tsconfig.json'));
}

function main(): void {
  const { verbose } = parseArgs(process.argv.slice(2));
  const rootDir = workspaceRoot();
  process.chdir(rootDir);

  console.log('🔎 tsconfig paths: every path a tsconfig names by hand must exist');

  const root = JSON.parse(readFileSync('package.json', 'utf8')) as { workspaces?: readonly string[] };
  const configs = ownedTsconfigs(root.workspaces ?? []);
  const failures: string[] = [];
  let inspected = 0;
  let allowed = 0;

  for (const config of configs) {
    const base = dirname(config);
    const label = relative(rootDir, resolve(config));
    let parsed: TsConfig;
    try {
      parsed = parseJsonc(readFileSync(config, 'utf8')) as TsConfig;
    } catch (error) {
      failures.push(`${label} — could not be parsed: ${error instanceof Error ? error.message : String(error)}`);
      continue;
    }

    const claims: { key: string; path: string; exists: boolean }[] = [];
    for (const key of ['include', 'exclude', 'files'] as const) {
      for (const entry of parsed[key] ?? []) {
        if (entry.includes('*') || entry.includes('?')) continue;
        claims.push({ key, path: entry, exists: existsSync(resolve(base, entry)) });
      }
    }
    for (const reference of parsed.references ?? []) {
      if (reference.path === undefined) continue;
      claims.push({ key: 'references', path: reference.path, exists: referenceExists(resolve(base, reference.path)) });
    }
    const extendsList = typeof parsed.extends === 'string' ? [parsed.extends] : (parsed.extends ?? []);
    for (const entry of extendsList) {
      if (!entry.startsWith('.')) continue;
      claims.push({ key: 'extends', path: entry, exists: existsSync(resolve(base, entry)) });
    }

    for (const claim of claims) {
      inspected += 1;
      if (claim.exists) {
        if (verbose) console.log(`   ✓  ${label} [${claim.key}] ${claim.path}`);
        continue;
      }
      const name = claim.path.replace(/^\.\//, '');
      if (INTENTIONAL.has(name)) {
        allowed += 1;
        if (verbose) console.log(`   ·  ${label} [${claim.key}] ${claim.path} — intentional`);
        continue;
      }
      failures.push(`${label} [${claim.key}] ${claim.path} — no such path`);
    }
  }

  if (inspected === 0) {
    console.error('   ❌ found no tsconfig to inspect — this would pass vacuously.');
    process.exit(1);
  }

  if (failures.length > 0) {
    for (const failure of failures) console.log(`   ❌ ${failure}`);
    console.log('');
    console.log(`${failures.length} tsconfig path(s) name something that is not there.`);
    console.log('Fix the path, delete the entry, or — if it is a build-output name that may legitimately be');
    console.log('absent — add it to INTENTIONAL in scripts/check-tsconfig-paths.ts.');
    process.exit(1);
  }

  const suffix = allowed === 0 ? '' : ` (${allowed} allowlisted)`;
  console.log(`   ✅ ${inspected} path(s) across ${configs.length} tsconfig(s) all exist${suffix}`);
}

main();

#!/usr/bin/env node
//
// The workspace root is the single source of truth for the web3 library versions. This gate holds every
// PUBLISHED manifest to it: a payload's `dependencies` or `peerDependencies` entry for one of them must
// name the exact version the root pins.
//
// Why it exists: the root hoists one copy, so every member compiles and tests against the root pin no
// matter what it declares. A payload declaring a different range therefore publishes a claim nobody
// verified — `hardhat/v2/pkg` shipped `ethers ^6.16.0` while the whole workspace built against 6.17.0,
// and nothing failed. A consumer installing 6.16.0 would have been the first to find out.
//
// What counts as a match: the declared range's FLOOR must equal the root pin, so `^6.17.0` matches a pin
// of `6.17.0` and `^6.16.0` does not. A caret is allowed — a published peer range should let consumers
// take patches — but its floor is what the workspace actually tested. Any other range syntax fails
// loudly rather than being guessed at, because a compound range has no single floor to compare.
//
// Scope: PUBLISHED manifests only, meaning `private` is not true. The scan roots come from the root
// manifest's `workspaces`, so a payload nested under a member (v12/pkg, which is deliberately not itself
// a member) is covered, while sdk/js-sdk is not — it belongs to the outer repo workspace and this gate
// does not govern it.
//
// Usage: ./scripts/check-dep-versions.ts [--verbose]
//   --verbose   list every declaration inspected, not just the failures
//
// Run from anywhere: the workspace root is found via `npm prefix`, falling back to this file's parent.
//
// SHARED SCRIPT, run by node with its types stripped — see scripts/tsconfig.json. It is data-driven: the
// libraries come from LIBRARIES, the pins from the root devDependencies, and the scan roots from
// `workspaces`. Adding a member needs no edit here.
//
// Exits non-zero on a mismatch, on a root pin that is not exact, or if the scan found no published
// manifest at all — that last one would otherwise pass vacuously.

import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const LIBRARIES = ['ethers', 'viem'] as const;
const FIELDS = ['dependencies', 'peerDependencies'] as const;

type Library = (typeof LIBRARIES)[number];

type Manifest = {
  readonly private?: boolean;
  readonly workspaces?: readonly string[];
  readonly devDependencies?: Readonly<Record<string, string>>;
  readonly dependencies?: Readonly<Record<string, string>>;
  readonly peerDependencies?: Readonly<Record<string, string>>;
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
    // `workspaces`, not merely a package.json: scripts/ has its own module-type marker, so `npm prefix`
    // answers with scripts/ when run from there, and that is not a root to scan.
    if (prefix && existsSync(join(prefix, 'package.json')) && readJson(join(prefix, 'package.json')).workspaces) {
      return prefix;
    }
  } catch {
    // npm is not on PATH, or we are outside a package — fall through to the path-based guess.
  }
  return dirname(dirname(fileURLToPath(import.meta.url)));
}

function readJson(file: string): Manifest {
  return JSON.parse(readFileSync(file, 'utf8')) as Manifest;
}

/** Every package.json under `dir`, skipping installed packages. */
function manifestsUnder(dir: string, found: string[] = []): string[] {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (entry.name === 'node_modules') continue;
    const full = join(dir, entry.name);
    if (entry.isDirectory()) manifestsUnder(full, found);
    else if (entry.name === 'package.json') found.push(full);
  }
  return found;
}

/**
 * The exact version a range's floor names, or undefined when the syntax has no single floor.
 * `^6.17.0` and `~6.17.0` and `6.17.0` all floor at `6.17.0`; `>=6 <7` has no answer.
 */
function floorOf(range: string): string | undefined {
  const match = /^[\^~]?(\d+\.\d+\.\d+.*)$/.exec(range);
  return match?.[1];
}

/** The root's pins. Exact, because a range here would make the source of truth itself ambiguous. */
function readPins(root: Manifest): ReadonlyMap<Library, string> {
  const pins = new Map<Library, string>();
  for (const library of LIBRARIES) {
    const declared = root.devDependencies?.[library];
    if (declared === undefined) {
      console.error(`   ❌ the root manifest has no devDependencies.${library} — nothing to compare against.`);
      process.exit(1);
    }
    if (!/^\d+\.\d+\.\d+$/.test(declared)) {
      console.error(`   ❌ root devDependencies.${library} is "${declared}" — the source of truth must be exact.`);
      process.exit(1);
    }
    pins.set(library, declared);
  }
  return pins;
}

function main(): void {
  const { verbose } = parseArgs(process.argv.slice(2));
  const rootDir = workspaceRoot();
  process.chdir(rootDir);

  console.log('🔎 dep versions: every published manifest must pin what the root pins');

  const root = readJson('package.json');
  const pins = readPins(root);

  const files = new Set<string>();
  for (const member of root.workspaces ?? []) {
    if (existsSync(member)) for (const file of manifestsUnder(member)) files.add(file);
  }

  let published = 0;
  let checked = 0;
  const failures: string[] = [];

  for (const file of [...files].sort()) {
    const manifest = readJson(file);
    if (manifest.private === true) continue;
    published += 1;

    for (const field of FIELDS) {
      for (const library of LIBRARIES) {
        const declared = manifest[field]?.[library];
        if (declared === undefined) continue;
        checked += 1;

        const where = `${relative(rootDir, file)} ${field}.${library}`;
        const floor = floorOf(declared);
        if (floor === undefined) {
          failures.push(`${where} = "${declared}" — unsupported range syntax, no single floor to compare`);
        } else if (floor !== pins.get(library)) {
          failures.push(`${where} = "${declared}" — floor ${floor} is not the root pin ${pins.get(library)}`);
        } else if (verbose) {
          console.log(`   ✓  ${where} = "${declared}"`);
        }
      }
    }
  }

  if (published === 0) {
    console.error('   ❌ found no published manifest under the declared workspaces — this would pass vacuously.');
    process.exit(1);
  }

  if (failures.length > 0) {
    for (const failure of failures) console.log(`   ❌ ${failure}`);
    const pinList = LIBRARIES.map((l) => `${l}=${pins.get(l)}`).join(', ');
    console.log('');
    console.log(`${failures.length} declaration(s) disagree with the root pins (${pinList}).`);
    console.log('Change the payload to match the root, not the other way round: the root pin is what every');
    console.log('member actually compiles and tests against, because the workspace hoists one copy.');
    process.exit(1);
  }

  console.log(`   ✅ ${checked} declaration(s) across ${published} published manifest(s) agree with the root pins`);
}

main();

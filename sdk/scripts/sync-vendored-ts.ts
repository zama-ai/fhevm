#!/usr/bin/env node
//
// Copies TypeScript shared verbatim between workspace members, from one source of truth to every
// destination that lists it. Sibling of check-vendored-sources.sh, which covers a different case: that
// one compares Solidity against an EXTERNAL repo at a declared commit and normalises through
// `forge fmt`. This one is in-repo TypeScript, compared exactly.
//
// A destination is normally byte-identical to the source: no header is prepended, because each file
// names its own provenance on line 1, and no import is rewritten, because these files import only bare
// specifiers or siblings.
//
// The ONE exception is a `rewrites` list on a destination, for when a destination already depends on the
// package the source only describes. `hardhat/v2/pkg` declares `@fhevm/host-contracts-cleartext` as a
// real dependency and imports it elsewhere, so it takes the interfaces from there rather than a copy of
// ethereumLibTypes.ts it does not need. A rewrite is:
//   - an EXACT string swap, never a regex, so it cannot match more than intended;
//   - asserted to have applied — a `from` that is absent is a hard failure, because a renamed import
//     would otherwise silently leave a destination unrewritten but still compiling;
//   - applied identically in both modes, so `--check` compares against exactly what a write would produce.
// Reach for a rewrite only to drop a dependency a destination already has. Anything else is a
// destination that should own its own file.
//
// Usage: ./scripts/sync-vendored-ts.ts [--check] [--verbose]
//   --check     compare instead of writing, and exit non-zero on any difference (for CI and lint)
//   --verbose   list every file inspected, not just the failures
//
// Run from anywhere: the workspace root is found via `npm prefix`, falling back to this file's parent.
//
// SHARED SCRIPT, run by node with its types stripped — see scripts/tsconfig.json. It is data-driven: the
// source directory, the destinations, the per-destination file list and any rewrites all come from
// vendored/manifest.json. Keep it that way — a destination is a manifest entry, never a line of code here.
//
// Destinations take a SUBSET of the source on purpose, so a file present at the source but not listed by
// a destination is not a failure. What IS a failure: a file a destination lists but the source does not
// have, a destination directory that does not exist, a rewrite that did not apply, and a run that
// inspected nothing at all. That last guard matters — an empty file list makes a file-by-file compare
// report no differences, which looks exactly like success.
//
// Every destination must be excluded from eslint and prettier. This is not optional hygiene: eslint's
// `prefer-nullish-coalescing` autofix silently rewrote a statement in one of these files, and no test or
// compiler would ever have flagged it.
//
// One kind of destination is a PUBLISHED package source tree: each generation's pkg/ts/types/ receives
// ethereumLibTypes.ts, and its public.ts re-exports from that copy rather than declaring the interfaces.
// That is what makes this gate the single check for those types — there is no second definition to diff.

import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const MANIFEST = 'vendored/manifest.json';

type Rewrite = {
  readonly file: string;
  readonly from: string;
  readonly to: string;
};

type Destination = {
  readonly to: string;
  readonly files: readonly string[];
  readonly rewrites?: readonly Rewrite[];
};

type VendoredManifest = {
  readonly source: string;
  readonly destinations?: readonly Destination[];
};

/** What a destination should hold, or why it could not be produced. Never both. */
type Expected = { readonly content: string; readonly error?: undefined } | { readonly error: string };

/** The leading comment block doubles as the help text, so there is only one copy of it. */
function usage(): void {
  const source = readFileSync(fileURLToPath(import.meta.url), 'utf8');
  for (const line of source.split('\n').slice(1)) {
    if (!line.startsWith('//')) break;
    console.log(line.replace(/^\/\/ ?/, ''));
  }
}

function parseArgs(argv: readonly string[]): { check: boolean; verbose: boolean } {
  let check = false;
  let verbose = false;
  for (const arg of argv) {
    if (arg === '--check') check = true;
    else if (arg === '--verbose') verbose = true;
    else if (arg === '-h' || arg === '--help') {
      usage();
      process.exit(0);
    } else {
      console.error(`Error: unknown argument '${arg}'. Try --help.`);
      process.exit(1);
    }
  }
  return { check, verbose };
}

function workspaceRoot(): string {
  try {
    const prefix = execFileSync('npm', ['prefix'], { encoding: 'utf8' }).trim();
    if (prefix && existsSync(join(prefix, MANIFEST))) return prefix;
  } catch {
    // npm is not on PATH, or we are outside a package — fall through to the path-based guess.
  }
  return dirname(dirname(fileURLToPath(import.meta.url)));
}

/**
 * The content a destination should hold: the source, plus any rewrites it declares for this file.
 * Returns `{ content }` on success or `{ error }` when a rewrite did not do what it claims.
 */
function expectedContent(sourcePath: string, destination: Destination, file: string): Expected {
  let content = readFileSync(sourcePath, 'utf8');
  for (const rewrite of destination.rewrites ?? []) {
    if (rewrite.file !== file) continue;
    if (!content.includes(rewrite.from)) {
      return { error: `rewrite did not apply: ${JSON.stringify(rewrite.from)} is not in the source` };
    }
    content = content.split(rewrite.from).join(rewrite.to);
    if (content.includes(rewrite.from)) {
      return { error: `rewrite left ${JSON.stringify(rewrite.from)} behind` };
    }
  }
  return { content };
}

/** Where two texts first disagree, for an error that names the line rather than just the file. */
function firstDifference(actual: string, expected: string): { line: number; here: string; want: string } {
  const here = actual.split('\n');
  const want = expected.split('\n');
  const at = here.findIndex((line, i) => line !== want[i]);
  const index = at === -1 ? Math.min(here.length, want.length) : at;
  return {
    line: index + 1,
    here: here[index] ?? '(missing)',
    want: want[index] ?? '(missing)',
  };
}

function main(): void {
  const { check, verbose } = parseArgs(process.argv.slice(2));
  process.chdir(workspaceRoot());

  if (!existsSync(MANIFEST)) {
    console.error(`Error: ${MANIFEST} not found (looked from ${process.cwd()}).`);
    process.exit(1);
  }
  const manifest = JSON.parse(readFileSync(MANIFEST, 'utf8')) as VendoredManifest;

  const sourceDir = manifest.source;
  if (!existsSync(sourceDir)) {
    console.error(`Error: source directory "${sourceDir}" from the manifest does not exist.`);
    process.exit(1);
  }

  console.log(
    check
      ? `🔎 vendored ts: every destination must match ${sourceDir}`
      : `📄 vendored ts: writing ${sourceDir} to every destination that lists a file`,
  );

  let inspected = 0;
  let drift = 0;
  let written = 0;

  for (const destination of manifest.destinations ?? []) {
    for (const file of destination.files) {
      inspected += 1;
      const sourcePath = join(sourceDir, file);
      const destPath = join(destination.to, file);

      if (!existsSync(sourcePath)) {
        console.log(`   ❌ ${destPath} — "${file}" is listed here but absent from ${sourceDir}`);
        drift += 1;
        continue;
      }
      if (!existsSync(destination.to)) {
        console.log(`   ❌ ${destPath} — destination directory does not exist`);
        drift += 1;
        continue;
      }

      const result = expectedContent(sourcePath, destination, file);
      if (result.error !== undefined) {
        console.log(`   ❌ ${destPath} — ${result.error}`);
        drift += 1;
        continue;
      }
      const expected = result.content;

      const actual = existsSync(destPath) ? readFileSync(destPath, 'utf8') : undefined;
      if (actual === expected) {
        if (verbose) {
          const rewritten = (destination.rewrites ?? []).some((r) => r.file === file);
          console.log(`   ✓  ${destPath}${rewritten ? ' (rewritten)' : ''}`);
        }
        continue;
      }

      if (check) {
        if (actual === undefined) {
          console.log(`   ❌ ${destPath} — missing`);
        } else {
          const { line, here, want } = firstDifference(actual, expected);
          console.log(`   ❌ ${destPath} — differs from ${sourcePath}`);
          console.log(`        first difference at line ${line}:`);
          console.log(`          here:     ${JSON.stringify(here)}`);
          console.log(`          expected: ${JSON.stringify(want)}`);
        }
        drift += 1;
      } else {
        writeFileSync(destPath, expected);
        written += 1;
        console.log(`   ↻  ${destPath}`);
      }
    }
  }

  if (inspected === 0) {
    console.error('   ❌ the manifest listed no files — the run would have passed vacuously.');
    process.exit(1);
  }

  if (drift !== 0) {
    console.log('');
    console.log(`${drift} of ${inspected} vendored files drifted from ${sourceDir}.`);
    console.log('Run ./scripts/sync-vendored-ts.ts to rewrite them. Do not edit a destination in place:');
    console.log(`the source of truth is ${sourceDir}, and a destination is a generated artefact.`);
    process.exit(1);
  }

  console.log(
    check
      ? `   ✅ ${inspected} vendored files match ${sourceDir}`
      : `   ✅ ${inspected} vendored files in sync (${written} written)`,
  );
}

main();

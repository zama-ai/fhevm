#!/usr/bin/env node
//
// Every publishable payload in the workspace must be packable. The root packs with
// `npm run pack:tarball --workspaces --if-present`, and `--if-present` SKIPS a member that does not
// define the script — silently, with a zero exit. So a new publishable member joins the workspace, its
// payload is validated by publint and attw, and it is simply never packed.
//
// That is not hypothetical: `hardhat/v2` sat in exactly that state. `pkg/package.json` was
// `@fhevm/hardhat-plugin@0.4.2`, non-private, with a `files` array, and `npm run pack:tarballs` produced
// only the two cleartext tarballs. Nothing failed, because nothing asked.
//
// Usage: ./scripts/check-pack-scripts.ts [--verbose]
//   --verbose   list every workspace inspected, not just the failures
//
// Run from anywhere: the workspace root is found via `npm prefix`, falling back to this file's parent.
//
// The rule: a workspace whose own manifest is publishable, or which holds a publishable `pkg/`, must
// define `pack:tarball`. "Publishable" means `private` is not true — the same test `check-dep-versions.ts`
// uses, so the two gates cannot disagree about what ships.

import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const PACK_SCRIPT = 'pack:tarball';

type Manifest = {
  readonly name?: string;
  readonly version?: string;
  readonly private?: boolean;
  readonly workspaces?: readonly string[];
  readonly scripts?: Readonly<Record<string, string>>;
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

function readManifest(file: string): Manifest {
  return JSON.parse(readFileSync(file, 'utf8')) as Manifest;
}

function workspaceRoot(): string {
  try {
    const prefix = execFileSync('npm', ['prefix'], { encoding: 'utf8' }).trim();
    if (prefix && existsSync(join(prefix, 'package.json')) && readManifest(join(prefix, 'package.json')).workspaces) {
      return prefix;
    }
  } catch {
    // npm is not on PATH, or we are outside a package — fall through to the path-based guess.
  }
  return dirname(dirname(fileURLToPath(import.meta.url)));
}

function isPublishable(manifest: Manifest): boolean {
  return manifest.private !== true;
}

function main(): void {
  const { verbose } = parseArgs(process.argv.slice(2));
  const rootDir = workspaceRoot();
  process.chdir(rootDir);

  console.log(`🔎 pack scripts: every publishable payload must define ${PACK_SCRIPT}`);

  const root = readManifest('package.json');
  const failures: string[] = [];
  let inspected = 0;

  // A member is skipped only if it is itself a nested payload of another member — `v13/pkg` is a
  // workspace in its own right AND the payload `v13` packs, so requiring a script of both would demand
  // two tarballs of one artifact.
  const members = [...(root.workspaces ?? [])].sort();
  const payloadsOfMembers = new Set(members.filter((m) => members.some((o) => o !== m && m === `${o}/pkg`)));

  for (const member of members) {
    if (!existsSync(join(member, 'package.json'))) {
      failures.push(`${member} is declared in workspaces but has no package.json`);
      continue;
    }
    if (payloadsOfMembers.has(member)) {
      if (verbose) console.log(`   ·  ${member} — payload of another member, packed by it`);
      continue;
    }

    const manifest = readManifest(join(member, 'package.json'));
    const payloadPath = join(member, 'pkg', 'package.json');
    const payload = existsSync(payloadPath) ? readManifest(payloadPath) : undefined;

    const ships =
      (isPublishable(manifest) ? manifest : undefined) ?? (payload !== undefined && isPublishable(payload) ? payload : undefined);
    if (ships === undefined) {
      if (verbose) console.log(`   ·  ${member} — private, nothing to pack`);
      continue;
    }

    inspected += 1;
    const what = `${ships.name ?? '(unnamed)'}@${ships.version ?? '(unversioned)'}`;
    if (manifest.scripts?.[PACK_SCRIPT] === undefined) {
      failures.push(
        `${member} publishes ${what} but defines no ${PACK_SCRIPT} — ` +
          `\`--workspaces --if-present\` skips it silently`,
      );
    } else if (verbose) {
      console.log(`   ✓  ${member} → ${what}`);
    }
  }

  if (inspected === 0) {
    console.error('   ❌ found no publishable payload — this would pass vacuously.');
    process.exit(1);
  }

  if (failures.length > 0) {
    for (const failure of failures) console.log(`   ❌ ${failure}`);
    console.log('');
    console.log(`${failures.length} publishable payload(s) would never be packed.`);
    console.log(`Add "${PACK_SCRIPT}": "\\"$(npm prefix)/scripts/pack-tarball.ts\\"" to the member, or mark it private.`);
    process.exit(1);
  }

  console.log(`   ✅ ${inspected} publishable payload(s) all define ${PACK_SCRIPT}`);
  if (relative(rootDir, process.cwd()) !== '') console.log(`   (checked from ${rootDir})`);
}

main();

#!/usr/bin/env node
//
// Packs one workspace member's published payload into the shared sdk/tarballs directory.
//
// Run from a member: npm run pack:tarball [-- --out-dir <dir>] [--clean]
//
//   --package-dir <dir>  the payload to pack. Defaults to <member>/pkg.
//   --out-dir <dir>      where the .tgz is written; created if missing. Defaults to sdk/tarballs.
//   --clean              delete existing *.tgz in that directory first
//
// Prints the tarball's absolute path on stdout.
//
// One file for every member rather than a copy per package: which member is calling comes from
// `npm_package_json`, the manifest npm is running the script for.
//
// A destination is a named flag only — a bare path is rejected. This runs inside `npm run` chains where a
// stray argument is easy to introduce and, taken as a destination, would silently write the tarball
// somewhere nothing looks for it.

import { existsSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { createPackageTarball } from '@fhevm/sdk-common-dev';

const USAGE = 'usage: npm run pack:tarball [-- --package-dir <dir>] [--out-dir <dir>] [--clean]';

const VALUE_FLAGS = ['--package-dir', '--out-dir'] as const;
type ValueFlag = (typeof VALUE_FLAGS)[number];

function isValueFlag(token: string): token is ValueFlag {
  return (VALUE_FLAGS as readonly string[]).includes(token);
}

/** The member this was invoked for: npm's own answer when there is one, the cwd otherwise. */
const memberRoot = process.env.npm_package_json === undefined ? process.cwd() : dirname(process.env.npm_package_json);

const values = new Map<ValueFlag, string>();
let clean = false;

const argv = process.argv.slice(2);
for (let index = 0; index < argv.length; index++) {
  const token = argv[index];
  if (token === undefined) continue;

  const inline = VALUE_FLAGS.find((name) => token.startsWith(`${name}=`));

  if (isValueFlag(token)) {
    const value = argv[index + 1];
    // Rejecting a value that starts with `-` so `--out-dir --clean` fails instead of packing into a
    // directory literally named "--clean".
    if (value === undefined || value.startsWith('-')) {
      throw new Error(`${token} requires a value\n${USAGE}`);
    }
    values.set(token, value);
    index++;
  } else if (inline !== undefined) {
    const value = token.slice(inline.length + 1);
    if (value === '') {
      throw new Error(`${inline} requires a value\n${USAGE}`);
    }
    values.set(inline, value);
  } else if (token === '--clean') {
    clean = true;
  } else {
    throw new Error(`unexpected argument ${JSON.stringify(token)}\n${USAGE}`);
  }
}

// ./pkg is the PUBLISHED payload manifest — packing the member root would ship the harness instead.
const packageDir = resolve(memberRoot, values.get('--package-dir') ?? 'pkg');
if (!existsSync(join(packageDir, 'package.json'))) {
  throw new Error(
    `no package.json at ${packageDir}\n` +
      `   That path is the published payload, not the harness. Pass --package-dir if this member keeps ` +
      `it somewhere other than ./pkg.\n${USAGE}`,
  );
}

const outDir = values.get('--out-dir');
console.log(createPackageTarball({ packageDir, ...(outDir !== undefined ? { outDir } : {}), clean }));

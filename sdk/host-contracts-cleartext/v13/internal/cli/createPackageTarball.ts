// Run: node internal/cli/createPackageTarball.ts [--out-dir <dir>] [--clean]
//
//   --out-dir <dir>   where the .tgz is written; created if missing. Defaults to ./tarball, a sibling
//                     of ./pkg, which is where the tarball-consumer fixture looks for it.
//   --clean           delete existing *.tgz in that directory first
//
// Prints the tarball's absolute path on stdout.
//
// The destination is a named flag only — a bare path is rejected. This runs inside `npm run` chains where
// a stray argument is easy to introduce and, taken as a destination, would silently write the tarball
// somewhere nothing looks for it.

import { createPackageTarball } from '../createPackageTarball.ts';

const USAGE = 'usage: node internal/cli/createPackageTarball.ts [--out-dir <dir>] [--clean]';

const argv = process.argv.slice(2);
let outDir: string | undefined;
let clean = false;

for (let index = 0; index < argv.length; index++) {
  const token = argv[index];
  if (token === '--out-dir') {
    const value = argv[index + 1];
    // Rejecting a value that starts with `-` so `--out-dir --clean` fails instead of packing into a
    // directory literally named "--clean".
    if (value === undefined || value.startsWith('-')) {
      throw new Error(`--out-dir requires a value\n${USAGE}`);
    }
    outDir = value;
    index++;
  } else if (token?.startsWith('--out-dir=') === true) {
    outDir = token.slice('--out-dir='.length);
    if (outDir === '') {
      throw new Error(`--out-dir requires a value\n${USAGE}`);
    }
  } else if (token === '--clean') {
    clean = true;
  } else {
    throw new Error(`unexpected argument ${JSON.stringify(token)}\n${USAGE}`);
  }
}

console.log(createPackageTarball({ ...(outDir !== undefined ? { outDir } : {}), clean }));

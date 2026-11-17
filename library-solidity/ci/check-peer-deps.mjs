#!/usr/bin/env node
//
// Fails if a required peer dependency of a declared package is not declared in library-solidity/package.json.
//
// npm (7+) installs missing peer dependencies automatically, so an undeclared peer dependency (for example
// @nomicfoundation/hardhat-ethers, required by @nomicfoundation/hardhat-typechain) would be installed silently, in a
// version nobody chose. This check makes that an error: every required peer dependency of every package listed in
// dependencies, devDependencies or optionalDependencies must itself be listed in one of them.
//
// Optional peer dependencies (peerDependenciesMeta.<name>.optional) are not required. Version ranges are not
// checked here: npm already refuses to install conflicting peer versions.
//
// Node built-ins only: it runs before or without any dependency being usable.

import { existsSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const packageDir = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const manifest = JSON.parse(readFileSync(join(packageDir, 'package.json'), 'utf8'));

const declared = new Map();
for (const field of ['dependencies', 'devDependencies', 'optionalDependencies']) {
  for (const [name, range] of Object.entries(manifest[field] ?? {})) {
    declared.set(name, `${field}: ${range}`);
  }
}

// Finds the installed package.json of `name` the way Node resolves packages: in node_modules folders from
// library-solidity up to the filesystem root (npm hoists most packages to the repository root).
function findInstalledManifest(name) {
  let dir = packageDir;
  for (;;) {
    const file = join(dir, 'node_modules', name, 'package.json');
    if (existsSync(file)) {
      return JSON.parse(readFileSync(file, 'utf8'));
    }
    const parent = dirname(dir);
    if (parent === dir) {
      return undefined;
    }
    dir = parent;
  }
}

const errors = [];
const notInstalled = [];
for (const name of [...declared.keys()].sort()) {
  const installed = findInstalledManifest(name);
  if (installed === undefined) {
    // An optional dependency for another platform (e.g. solidity-comments-linux-x64-gnu on macOS) is never installed.
    if (!(name in (manifest.optionalDependencies ?? {}))) {
      notInstalled.push(name);
    }
    continue;
  }
  const meta = installed.peerDependenciesMeta ?? {};
  for (const [peer, range] of Object.entries(installed.peerDependencies ?? {})) {
    if (meta[peer]?.optional === true) {
      continue;
    }
    if (!declared.has(peer)) {
      errors.push(`${name}@${installed.version} requires peer ${peer}@"${range}", which is not declared`);
    }
  }
}

if (notInstalled.length > 0) {
  console.error(`check-peer-deps: declared but not installed (run npm install first): ${notInstalled.join(', ')}`);
  process.exit(1);
}
if (errors.length > 0) {
  console.error('check-peer-deps: undeclared peer dependencies in library-solidity/package.json:');
  for (const error of errors) {
    console.error(`  - ${error}`);
  }
  console.error(
    'Declare each one explicitly, from the repository root: npm install --save-dev --workspace=library-solidity <peer>@<range>',
  );
  process.exit(1);
}
console.log(`check-peer-deps: ok (${declared.size} declared packages, every required peer dependency is declared)`);

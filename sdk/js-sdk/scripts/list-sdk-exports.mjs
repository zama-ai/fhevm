#!/usr/bin/env node
// Lists all publicly exported names from each @fhevm/sdk entry point defined in src/package.json

import { readFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));

const SRC_DIR = new URL(import.meta.url).searchParams.get('src')
  ?? join(SCRIPT_DIR, '..', 'src');

const pkg = JSON.parse(readFileSync(join(SRC_DIR, 'package.json'), 'utf8'));

function extractExports(content) {
  const names = [];

  // export { Foo, type Bar, Baz as Qux } from '...'
  // export type { Foo } from '...'
  const braceRe = /export\s+(?:type\s+)?\{([^}]+)\}/g;
  for (const m of content.matchAll(braceRe)) {
    for (const item of m[1].split(',')) {
      const name = item.trim().replace(/^type\s+/, '').split(/\s+as\s+/).pop().trim();
      if (name) names.push(name);
    }
  }

  // export function/class/const/type/interface/enum Foo
  const directRe = /^export\s+(?:async\s+)?(?:function\*?|class|const|let|var|type|interface|enum)\s+(\w+)/gm;
  for (const m of content.matchAll(directRe)) names.push(m[1]);

  return [...new Set(names)].sort();
}

let anyOutput = false;

for (const [entryPoint, conditions] of Object.entries(pkg.exports)) {
  if (entryPoint === './package.json') continue;
  if (typeof conditions !== 'object' || !conditions.types) continue;

  // ./_types/core/foo/index.d.ts  →  core/foo/index.ts
  const srcRel = conditions.types.replace('./_types/', '').replace(/\.d\.ts$/, '.ts');
  const srcPath = join(SRC_DIR, srcRel);

  let content;
  try {
    content = readFileSync(srcPath, 'utf8');
  } catch {
    console.error(`  (could not read ${srcPath})`);
    continue;
  }

  const names = extractExports(content);
  if (names.length === 0) continue;

  anyOutput = true;
  const label = entryPoint === '.' ? '@fhevm/sdk' : `@fhevm/sdk/${entryPoint.replace(/^\.\//, '')}`;
  console.log(`\n${label}`);
  for (const name of names) {
    const isType = /^[A-Z]/.test(name) || content.includes(`export type { ${name}`) || content.includes(`export type {${name}`);
    const tag = isType && !content.match(new RegExp(`export\\s+(?:async\\s+)?(?:function|const|let|var)\\s+${name}`)) ? 'type' : 'fn  ';
    console.log(`  ${tag}  ${name}`);
  }
}

if (!anyOutput) console.log('(no exports found — run "npm run build:types" first or check source paths)');

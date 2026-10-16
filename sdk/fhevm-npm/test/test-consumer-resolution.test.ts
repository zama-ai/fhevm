import assert from 'node:assert/strict';
import { join } from 'node:path';
import test from 'node:test';

import type { PackageJson } from '../base/npm.ts';
import {
  classifyDeclarationTrace,
  classifyResolvedEntryPoint,
  declaredTypeEntryPoints,
  declaresRootEntryPoint,
  probedModuleKinds,
} from '../base/test-consumer.ts';

const dual = {
  name: '@scope/dual',
  main: './_cjs/index.js',
  types: './_types/index.d.ts',
  exports: {
    '.': {
      types: './_types/index.d.ts',
      import: './_esm/index.js',
      require: './_cjs/index.js',
    },
  },
} as unknown as PackageJson;

const esmOnly = {
  name: '@scope/esm',
  type: 'module',
  exports: { '.': { types: './_types/index.d.ts', import: './_esm/index.js' } },
} as unknown as PackageJson;

test('only the module kinds the consumer uses AND the installed package exposes are probed', () => {
  assert.deepEqual(probedModuleKinds('cjs', dual), ['cjs']);
  assert.deepEqual(probedModuleKinds('esm', dual), ['esm']);
  assert.deepEqual(probedModuleKinds('dual', dual), ['cjs', 'esm']);
  // An ESM-only payload is never probed through require(), whatever the consumer is.
  assert.deepEqual(probedModuleKinds('cjs', esmOnly), []);
  assert.deepEqual(probedModuleKinds('dual', esmOnly), ['esm']);
});

test('a resolved file is accepted only when it is inside the installed package and declared by it', () => {
  const root = join('/tmp', 'consumer', 'node_modules', '@scope', 'dual');

  const declared = classifyResolvedEntryPoint(root, join(root, '_cjs', 'index.js'), dual);
  assert.deepEqual(declared, { relPath: '_cjs/index.js', inside: true, declared: true });

  // Inside the package but not an entry point its own package.json names.
  const undeclaredPath = classifyResolvedEntryPoint(root, join(root, 'src', 'index.ts'), dual);
  assert.equal(undeclaredPath.inside, true);
  assert.equal(undeclaredPath.declared, false);

  // Escaped the installed package entirely — what a symlinked or hoisted install looks like.
  const escaped = classifyResolvedEntryPoint(root, join('/tmp', 'workspace', 'plugin', 'src', 'index.ts'), dual);
  assert.equal(escaped.inside, false);
  assert.equal(escaped.declared, false);
});

test('a subpath-only package is not probed for a root entry point', () => {
  // The vendored dev packages are shaped like this: reachable only as '@scope/pkg/thing'.
  assert.equal(declaresRootEntryPoint({ exports: { './*': './src/*' } } as unknown as PackageJson), false);
  assert.equal(
    declaresRootEntryPoint({
      exports: { './*': './src/*', './package.json': './package.json' },
    } as unknown as PackageJson),
    false,
  );
  // A root export, in each of the shapes Node accepts.
  assert.equal(declaresRootEntryPoint(dual), true);
  assert.equal(declaresRootEntryPoint({ exports: './index.js' } as unknown as PackageJson), true);
  assert.equal(declaresRootEntryPoint({ exports: ['./index.js'] } as unknown as PackageJson), true);
  assert.equal(declaresRootEntryPoint({ exports: { import: './i.js' } } as unknown as PackageJson), true);
  // No exports map at all: the legacy fields are the root.
  assert.equal(declaresRootEntryPoint({ main: './index.js' } as unknown as PackageJson), true);
  assert.equal(declaresRootEntryPoint({ name: '@scope/bare' } as unknown as PackageJson), false);
});

test('type entry points are collected from types, typings and every exports condition', () => {
  assert.deepEqual(declaredTypeEntryPoints(dual), ['_types/index.d.ts']);
  assert.deepEqual(
    declaredTypeEntryPoints({
      typings: './legacy.d.ts',
      exports: { '.': { require: { types: './_types-cjs/index.d.ts' } }, './ts': { types: './ts/index.d.ts' } },
    } as unknown as PackageJson),
    ['_types-cjs/index.d.ts', 'legacy.d.ts', 'ts/index.d.ts'],
  );
  assert.deepEqual(declaredTypeEntryPoints({ name: '@scope/none' } as unknown as PackageJson), []);
});

test('a trace that reaches the installed declarations passes', () => {
  const root = join('/tmp', 'consumer', 'node_modules', '@scope', 'dual');
  const trace = [
    "Resolving module '@scope/dual' from '/tmp/consumer/probe.ts'.",
    `File '${join(root, '_types', 'index.d.ts')}' exist - use it as a name resolution result.`,
  ].join('\n');

  const verdict = classifyDeclarationTrace(trace, root, ['_types/index.d.ts'], join('/tmp', 'workspace', 'plugin'));
  assert.equal(verdict.reachedDeclared, true);
  assert.equal(verdict.escapedTo, undefined);
});

test('a trace that resolves the payload sources in the workspace is an escape', () => {
  const root = join('/tmp', 'consumer', 'node_modules', '@scope', 'dual');
  const source = join('/tmp', 'workspace', 'plugin');
  const trace = [
    "Resolving module '@scope/dual' from '/tmp/consumer/probe.ts'.",
    `File '${join(source, 'src', 'index.ts')}' exist - use it as a name resolution result.`,
  ].join('\n');

  const verdict = classifyDeclarationTrace(trace, root, ['_types/index.d.ts'], source);
  assert.ok(verdict.escapedTo !== undefined, 'resolving a .ts under the payload directory must be reported');
  assert.equal(verdict.reachedDeclared, false);
});

test('merely mentioning the payload directory is not an escape', () => {
  const root = join('/tmp', 'consumer', 'node_modules', '@scope', 'dual');
  const source = join('/tmp', 'workspace', 'plugin');
  // The trace names the payload while walking a file: link, but resolves the INSTALLED declarations.
  const trace = [
    `Loading module as file / folder, candidate module location '${source}', target file types 'TypeScript'.`,
    `File '${join(root, '_types', 'index.d.ts')}' exist - use it as a name resolution result.`,
  ].join('\n');

  const verdict = classifyDeclarationTrace(trace, root, ['_types/index.d.ts'], source);
  assert.equal(verdict.escapedTo, undefined);
  assert.equal(verdict.reachedDeclared, true);
});

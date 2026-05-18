import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import ts from 'typescript';

const file = process.argv[2];
const functionName = process.argv[3] || '__wbg_get_imports';

if (!file) {
  throw new Error('Usage: node list-wbg-calls.js <file> [functionName]');
}

function findRepoRoot(start) {
  let current = resolve(start);

  while (true) {
    if (existsSync(resolve(current, '.git'))) {
      return current;
    }

    const parent = dirname(current);

    if (parent === current) {
      return undefined;
    }

    current = parent;
  }
}

function resolveInputPath(input) {
  const cwdPath = resolve(process.cwd(), input);

  if (existsSync(cwdPath)) {
    return cwdPath;
  }

  const scriptDir = dirname(fileURLToPath(import.meta.url));
  const repoRoot = findRepoRoot(scriptDir);

  if (repoRoot) {
    const repoPath = resolve(repoRoot, input);

    if (existsSync(repoPath)) {
      return repoPath;
    }
  }

  return cwdPath;
}

const resolvedFile = resolveInputPath(file);
const sourceText = readFileSync(resolvedFile, 'utf8');

const sourceFile = ts.createSourceFile(resolvedFile, sourceText, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);

const ignoredGlobalCalls = new Set([
  'AggregateError',
  'Array',
  'ArrayBuffer',
  'BigInt',
  'BigInt64Array',
  'BigUint64Array',
  'Boolean',
  'DataView',
  'Date',
  'Error',
  'EvalError',
  'Float32Array',
  'Float64Array',
  'Int8Array',
  'Int16Array',
  'Int32Array',
  'Map',
  'Number',
  'Object',
  'Promise',
  'RangeError',
  'ReferenceError',
  'RegExp',
  'Set',
  'String',
  'Symbol',
  'SyntaxError',
  'TypeError',
  'URIError',
  'Uint8Array',
  'Uint8ClampedArray',
  'Uint16Array',
  'Uint32Array',
  'WeakMap',
  'WeakSet',
]);

function text(node) {
  return node.getText(sourceFile);
}

function findFunction(node, name) {
  if (ts.isFunctionDeclaration(node) && node.name && node.name.text === name) {
    return node;
  }

  return ts.forEachChild(node, (child) => findFunction(child, name));
}

function collectCalls(node, out = new Set()) {
  function visit(current) {
    if (ts.isCallExpression(current) && ts.isIdentifier(current.expression)) {
      const call = text(current.expression);

      if (!ignoredGlobalCalls.has(call)) {
        out.add(call);
      }
    }

    ts.forEachChild(current, visit);
  }

  visit(node);
  return out;
}

const fn = findFunction(sourceFile, functionName);

if (!fn) {
  throw new Error(`Could not find ${functionName}`);
}

const calls = [...collectCalls(fn)].sort();

for (const call of calls) {
  console.log(call);
}

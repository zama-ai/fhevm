import { createRequire } from 'node:module';
import { dirname, isAbsolute, relative, resolve, sep } from 'node:path';

const require = createRequire(import.meta.url);
const ts = require('typescript');

const formatHost = {
  getCanonicalFileName: (fileName) => fileName,
  getCurrentDirectory: ts.sys.getCurrentDirectory,
  getNewLine: () => ts.sys.newLine,
};

function formatDiagnostics(diagnostics) {
  return ts.formatDiagnosticsWithColorAndContext(diagnostics, formatHost);
}

function isInsideDir(file, dir) {
  const rel = relative(dir, file);
  return rel !== '' && !rel.startsWith('..') && !isAbsolute(rel);
}

export function isTypeScriptSourceRel(rel) {
  return rel.endsWith('.ts') && !rel.endsWith('.d.ts');
}

export function isDeclarationRel(rel) {
  return rel.endsWith('.d.ts');
}

export function isBuildMetadataRel(rel) {
  const parts = rel.split('/');
  return parts.some((part) => part.startsWith('.')) || /^tsconfig(?:\..*)?\.json$/.test(rel);
}

export function readTsconfigSourceRels(configPath, sourceRoot) {
  const resolvedConfigPath = resolve(configPath);
  const resolvedSourceRoot = resolve(sourceRoot);
  const readResult = ts.readConfigFile(resolvedConfigPath, ts.sys.readFile);

  if (readResult.error !== undefined) {
    throw new Error(formatDiagnostics([readResult.error]));
  }

  const parsed = ts.parseJsonConfigFileContent(
    readResult.config,
    ts.sys,
    dirname(resolvedConfigPath),
    undefined,
    resolvedConfigPath,
  );

  if (parsed.errors.length > 0) {
    throw new Error(formatDiagnostics(parsed.errors));
  }

  return new Set(
    parsed.fileNames
      .map((file) => resolve(file))
      .filter((file) => isInsideDir(file, resolvedSourceRoot))
      .map((file) => relative(resolvedSourceRoot, file).split(sep).join('/')),
  );
}

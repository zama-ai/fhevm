import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

/** Root of the `@fhevm/test-engine` package (this file is compiled to `dist/src/paths.js`). */
export const PACKAGE_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

/** Where `tsc` writes the compiled JavaScript. */
export const DIST_DIR = path.join(PACKAGE_ROOT, 'dist');

export const DEFAULT_SCENARIOS_DIR = path.join(PACKAGE_ROOT, 'scenarios');
export const DEFAULT_REPORTS_DIR = path.join(PACKAGE_ROOT, 'reports');
export const MANIFEST_SCHEMA_PATH = path.join(PACKAGE_ROOT, 'schema', 'scenario.v1.schema.json');

/** Compiled entry point of the scenario worker process. */
export const WORKER_ENTRY = path.join(DIST_DIR, 'src', 'runner', 'worker.js');

/** Support code loaded into every scenario before the scenario's own support code. */
export const ENGINE_SUPPORT_MODULE = path.join(DIST_DIR, 'src', 'cucumber', 'support', 'engine-support.js');

export function readEngineVersion(): string {
  const pkg = JSON.parse(readFileSync(path.join(PACKAGE_ROOT, 'package.json'), 'utf8')) as { version: string };
  return pkg.version;
}

/** True when `candidate` is `parent` itself or lives below it. */
export function isWithin(parent: string, candidate: string): boolean {
  const relative = path.relative(parent, candidate);
  return relative === '' || (!relative.startsWith('..') && !path.isAbsolute(relative));
}

/**
 * Maps a TypeScript source file to the JavaScript file `tsc` emits for it, e.g.
 * `<root>/scenarios/smoke/steps.ts` → `<root>/dist/scenarios/smoke/steps.js`.
 * Returns undefined when the source lives outside the compiled source root.
 */
export function compiledPathFor(sourcePath: string, sourceRoot: string, distDir: string): string | undefined {
  if (!isWithin(sourceRoot, sourcePath)) return undefined;
  return path.join(distDir, path.relative(sourceRoot, sourcePath)).replace(/\.ts$/, '.js');
}

// Guards the one thing a Hardhat v2 plugin cannot get wrong: that a `module: CommonJS` +
// `moduleResolution: node10` consumer resolves this package to its SHIPPED DECLARATIONS.
//
// node10 ignores the `exports` map entirely. It reads `main` and `types` off package.json and nothing
// else, so those two fields are the whole contract — and a package that looks correct under `exports`
// can still be invisible to the resolver Hardhat v2 users actually run.
//
// ## Why this asserts the resolved PATH rather than "it compiles"
//
// Without correct `types`, a consumer often still typechecks: node10 falls back to directory resolution
// and can land on `src/index.ts`, pulling the package's TypeScript SOURCES into the consumer's build. No
// error is produced, so a pass/fail check would not notice. The file it lands on is the assertion.
//
// ## Run it after a build
//
// It reads the real build output. `npm run build` produces it; on a clean tree this fails with a message
// saying so rather than silently passing on absent files.

import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test, { after } from 'node:test';
import { CJS_ENTRY_ABS_PATH, PKG_DIR_ABS_PATH, TYPES_ENTRY_ABS_PATH } from '../internal/constants.ts';

/** The repo's own compiler. The throwaway consumer installs nothing. */
const TSC = createRequire(import.meta.url).resolve('typescript/bin/tsc');

const PACKAGE_NAME = '@fhevm/hardhat-plugin';

const consumers: string[] = [];
after(() => {
  for (const dir of consumers) rmSync(dir, { recursive: true, force: true });
});

/**
 * A throwaway CommonJS + node10 project with this package linked in, typechecked with `--traceResolution`.
 *
 * The trace is the point: it names the file the resolver chose, which is the only way to tell a correct
 * `types` entry from a fallback onto the sources.
 */
function resolveAsNode10Consumer(): string {
  const dir = mkdtempSync(join(tmpdir(), 'hardhat-v2-node10-'));
  consumers.push(dir);

  // Linked rather than installed: `node_modules/<name>` pointing at pkg/ is exactly what a workspace
  // consumer sees, and it keeps the test independent of npm's network behaviour.
  const modules = join(dir, 'node_modules', '@fhevm');
  mkdirSync(modules, { recursive: true });
  execFileSync('ln', ['-s', PKG_DIR_ABS_PATH, join(modules, 'hardhat-plugin')]);

  writeFileSync(
    join(dir, 'tsconfig.json'),
    JSON.stringify(
      {
        compilerOptions: {
          // Exactly what a Hardhat v2 plugin consumer runs, deprecation warning included.
          module: 'CommonJS',
          moduleResolution: 'node10',
          ignoreDeprecations: '6.0',
          noEmit: true,
          skipLibCheck: true,
          esModuleInterop: true,
          types: [],
        },
        files: ['index.ts'],
      },
      null,
      2,
    ),
  );
  writeFileSync(join(dir, 'index.ts'), `import ${JSON.stringify(PACKAGE_NAME)};\n`);

  // `--traceResolution` prints a line per resolution step and already exceeds execFileSync's 1 MB default,
  // which throws ENOBUFS. Sized well past that rather than to today's measurement: the trace grows with
  // every dependency, and the two failure modes are a crash or — if the error were swallowed — assertions
  // run against a truncated trace, which is the worse one.
  return execFileSync(process.execPath, [TSC, '--project', dir, '--traceResolution'], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
    maxBuffer: 64 * 1024 * 1024,
  });
}

void test('the build produced the entries package.json advertises', () => {
  const built = existsSync(CJS_ENTRY_ABS_PATH) && existsSync(TYPES_ENTRY_ABS_PATH);
  assert.ok(
    built,
    `no build output — run \`npm run build\` first.\n  expected ${CJS_ENTRY_ABS_PATH}\n  expected ${TYPES_ENTRY_ABS_PATH}`,
  );

  // The CJS stub is what makes Node treat `_cjs/*.js` as CommonJS even though the harness above declares
  // `type: module`. Without it, a consumer on a modern Node gets ERR_REQUIRE_ESM at load time.
  const stub = join(PKG_DIR_ABS_PATH, '_cjs', 'package.json');
  assert.ok(existsSync(stub), `missing ${stub} — build:cjs writes it after tsc`);
  assert.equal(
    (JSON.parse(readFileSync(stub, 'utf8')) as { type?: string }).type,
    'commonjs',
    'the _cjs stub must pin the module format',
  );
});

void test('a node10 consumer resolves the shipped declarations, not the sources', () => {
  if (!existsSync(TYPES_ENTRY_ABS_PATH)) {
    assert.fail('no build output — run `npm run build` first');
  }
  const trace = resolveAsNode10Consumer();

  const resolvedToTypes = trace.includes(join('_types', 'index.d.ts'));
  const resolvedToSource = /File '.*pkg[/\\]src[/\\]index\.ts' exist/.test(trace);

  assert.ok(
    resolvedToTypes,
    'a node10 consumer did not land on _types/index.d.ts. Check `types` and `typings` in pkg/package.json —\n' +
      'node10 never reads the `exports` map, so those two fields are the entire contract.',
  );
  // The interesting half. Landing on the sources still typechecks, which is why "it compiled" proves
  // nothing: it means the consumer is compiling OUR TypeScript with THEIR settings.
  assert.ok(
    !resolvedToSource,
    'a node10 consumer resolved pkg/src/index.ts — the TypeScript sources — instead of the declarations',
  );
});

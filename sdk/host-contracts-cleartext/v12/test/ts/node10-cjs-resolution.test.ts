// Guards the package's resolution for a Hardhat v2 plugin: `module: CommonJS` + `moduleResolution: node10`.
//
// node10 ignores the `exports` map entirely and falls back to directory resolution on `ts/`, so the only
// thing that points it at the shipped declarations is `ts/package.json` (the `main`/`module`/`types` stub).
//
// Why this test asserts the resolved PATH and not merely "it compiles": without that stub the consumer
// still typechecks — node10 happily resolves `ts/index.ts` and pulls the package's ESM *sources* into the
// consumer's build. No error is produced, so a pass/fail check would not notice. The file it lands on is
// The whole assertion.
//
// Built against the real packed artifact in the shared tarballs directory, not against pkg/ — an
// `exports`/`files` change that omits the stub has to be caught in the thing that actually gets published.
import { TARBALL_DIR_ABS_PATH } from '@fhevm/sdk-common';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { afterAll, expect, test } from 'vitest';

const TEST_ROOT = dirname(fileURLToPath(import.meta.url));
const PACKAGE_ROOT = join(TEST_ROOT, '..', '..');
const PACKAGE_SPECIFIER = '@fhevm/host-contracts-cleartext/ts';

/** The repo's own compiler; the throwaway consumers install nothing. */
const TSC = createRequire(import.meta.url).resolve('typescript/bin/tsc');

const createdConsumers: string[] = [];

afterAll(() => {
  for (const dir of createdConsumers) {
    rmSync(dir, { recursive: true, force: true });
  }
});

////////////////////////////////////////////////////////////////////////////////

function readJson<T>(path: string): T | null {
  try {
    return JSON.parse(readFileSync(path, 'utf8')) as T;
  } catch {
    return null;
  }
}

////////////////////////////////////////////////////////////////////////////////

function _tarballPath(): string {
  if (!existsSync(TARBALL_DIR_ABS_PATH)) {
    throw new Error(`Missing ${TARBALL_DIR_ABS_PATH}. Run npm run build (or npm run pack:tarball) first.`);
  }
  const tarballs = readdirSync(TARBALL_DIR_ABS_PATH).filter((entry) => entry.endsWith('.tgz'));

  // Pinned by VERSION, not by name prefix: the directory is shared, and every generation publishes under
  // The same npm name, so a prefix filter matches a sibling's tarball too. The version pins the exact
  // artifact this test exercises, which also makes a stale one a miss rather than a coin flip.
  const version = readJson<{ version?: string }>(join(PACKAGE_ROOT, 'pkg', 'package.json'))?.version;
  if (version === undefined) {
    throw new Error(`Could not read version from ${join(PACKAGE_ROOT, 'pkg', 'package.json')}.`);
  }
  const expected = `fhevm-host-contracts-cleartext-${version}.tgz`;
  if (!tarballs.includes(expected)) {
    const found = tarballs.length === 0 ? '(none)' : tarballs.join(', ');
    throw new Error(`Missing ${expected} in ${TARBALL_DIR_ABS_PATH}, found: ${found}. Run npm run pack:tarball.`);
  }
  return join(TARBALL_DIR_ABS_PATH, expected);
}

/**
 * A throwaway project configured the way a Hardhat v2 plugin is, with the packed tarball extracted into
 * its node_modules exactly as npm would place it.
 */
function _createNode10Consumer(): string {
  const dir = mkdtempSync(join(tmpdir(), 'fhevm-node10-consumer-'));
  createdConsumers.push(dir);

  const packageDir = join(dir, 'node_modules', '@fhevm', 'host-contracts-cleartext');
  mkdirSync(packageDir, { recursive: true });
  mkdirSync(join(dir, 'src'), { recursive: true });
  execFileSync('tar', ['-xzf', _tarballPath(), '--strip-components', '1', '-C', packageDir], { stdio: 'pipe' });

  // "type": "commonjs" is the point of the exercise: Hardhat v2 requires its plugins to be require()-able.
  writeFileSync(
    join(dir, 'package.json'),
    `${JSON.stringify({ name: 'node10-consumer', private: true, type: 'commonjs' }, null, 2)}\n`,
  );

  // Mirrors packages/hardhat-plugin/tsconfig.base.json. verbatimModuleSyntax must be false here — with
  // module: CommonJS, TypeScript rejects ESM import syntax when it is on.
  writeFileSync(
    join(dir, 'tsconfig.json'),
    `${JSON.stringify(
      {
        compilerOptions: {
          strict: true,
          noEmit: true,
          skipLibCheck: true,
          target: 'ES2021',
          esModuleInterop: true,
          module: 'CommonJS',
          moduleResolution: 'node10',
          verbatimModuleSyntax: false,
          ignoreDeprecations: '6.0',
        },
        include: ['src'],
      },
      null,
      2,
    )}\n`,
  );

  writeFileSync(
    join(dir, 'src', 'consumer.ts'),
    `import { deploy, precomputeAddresses } from '${PACKAGE_SPECIFIER}';\n` +
      'export const entryPoints: readonly unknown[] = [deploy, precomputeAddresses];\n',
  );

  return dir;
}

////////////////////////////////////////////////////////////////////////////////

function _runTsc(cwd: string): { readonly output: string; readonly ok: boolean } {
  try {
    const output = execFileSync(process.execPath, [TSC, '--project', 'tsconfig.json', '--traceResolution'], {
      cwd,
      encoding: 'utf8',
      stdio: 'pipe',
    });
    return { output, ok: true };
  } catch (error) {
    const failure = error as { stdout?: string; stderr?: string };
    return { output: `${failure.stdout ?? ''}${failure.stderr ?? ''}`, ok: false };
  }
}

/** The path `--traceResolution` reports for the package specifier, or undefined if it never resolved. */
function _resolvedTarget(traceOutput: string): string | undefined {
  const line = traceOutput
    .split('\n')
    .find(
      (entry) => entry.includes(`Module name '${PACKAGE_SPECIFIER}'`) && entry.includes('successfully resolved to'),
    );
  return line === undefined ? undefined : /resolved to '([^']+)'/.exec(line)?.[1];
}

////////////////////////////////////////////////////////////////////////////////

test('node10 + CommonJS resolves the shipped declarations, not the package sources', () => {
  const dir = _createNode10Consumer();

  // The mechanism, asserted directly so a missing stub reports itself rather than showing up as a
  // confusing resolution result below.
  const stub = join(dir, 'node_modules', '@fhevm', 'host-contracts-cleartext', 'ts', 'package.json');
  expect(existsSync(stub), `the tarball must ship ts/package.json — node10 has nothing else to go on`).toBe(true);

  const { output, ok } = _runTsc(dir);
  const resolved = _resolvedTarget(output);

  expect(resolved, output).toBeDefined();
  expect(resolved).toMatch(/ts[/\\]_types[/\\]index\.d\.ts$/);
  // The regression this test exists for: ts/index.ts compiles fine and is silently wrong.
  expect(resolved).not.toMatch(/ts[/\\]index\.ts$/);
  expect(ok, output).toBe(true);
});

////////////////////////////////////////////////////////////////////////////////

test('the CommonJS build is require()-able from a CommonJS consumer', () => {
  const dir = _createNode10Consumer();

  // Node does honour the exports map, so this goes through "require" -> ts/_cjs/index.js. It passes even
  // when the types are misconfigured, which is exactly why the type assertions above are separate.
  const output = execFileSync(
    process.execPath,
    ['-e', `const m = require('${PACKAGE_SPECIFIER}'); console.log(typeof m.deploy, typeof m.precomputeAddresses);`],
    { cwd: dir, encoding: 'utf8', stdio: 'pipe' },
  );

  expect(output.trim()).toBe('function function');
});

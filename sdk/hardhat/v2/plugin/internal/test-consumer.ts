#!/usr/bin/env node
import { execFileSync, spawnSync } from 'node:child_process';
import { createRequire } from 'node:module';
import { cpSync, lstatSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, isAbsolute, join, relative, resolve, sep } from 'node:path';
import { pathToFileURL } from 'node:url';

const pluginDirectory = dirname(dirname(import.meta.filename));
const workspaceRoot = resolve(pluginDirectory, '../../..');
const templateDirectory = resolve(pluginDirectory, '../fhevm-hardhat-template');
const hostOwnerDirectory = resolve(workspaceRoot, 'host-contracts-cleartext/v13');
const candidates = {
  '@fhevm/hardhat-plugin': resolve(pluginDirectory, 'pkg'),
  '@fhevm/host-contracts-cleartext': resolve(hostOwnerDirectory, 'pkg'),
} as const;
const excludedTemplateRoots = new Set([
  '.git',
  'artifacts',
  'cache',
  'coverage',
  'dist',
  'fhevmTemp',
  'node_modules',
  'package-lock.json',
  'types',
]);

type PackageJson = {
  devDependencies?: Record<string, string>;
};

function main(): void {
  buildCandidates();

  const temporaryRoot = mkdtempSync(join(tmpdir(), 'fhevm-hardhat-consumer-'));
  const consumerDirectory = join(temporaryRoot, 'consumer');
  try {
    copyConsumer(consumerDirectory);
    patchCandidateDependencies(consumerDirectory);

    runNpm(consumerDirectory, ['install', '--install-links', '--no-audit', '--no-fund']);
    verifyPhysicalCandidates(consumerDirectory);
    verifyRuntimeResolution(consumerDirectory);
    verifyTypeResolution(consumerDirectory);
    runNpm(consumerDirectory, ['test']);

    console.log(`✅ test:consumer passed in isolated installation ${consumerDirectory}`);
  } finally {
    rmSync(temporaryRoot, { recursive: true, force: true });
  }
}

function buildCandidates(): void {
  for (const output of [
    resolve(pluginDirectory, 'pkg/_cjs'),
    resolve(pluginDirectory, 'pkg/_types'),
    resolve(hostOwnerDirectory, 'pkg/ts/_cjs'),
    resolve(hostOwnerDirectory, 'pkg/ts/_esm'),
    resolve(hostOwnerDirectory, 'pkg/ts/_types'),
  ]) {
    rmSync(output, { recursive: true, force: true });
  }
  runNpm(pluginDirectory, ['run', 'build:cjs']);
  runNpm(pluginDirectory, ['run', 'build:types']);
  runNpm(hostOwnerDirectory, ['run', 'build:cjs']);
  runNpm(hostOwnerDirectory, ['run', 'build:esm']);
  runNpm(hostOwnerDirectory, ['run', 'build:types']);
}

function copyConsumer(destination: string): void {
  cpSync(templateDirectory, destination, {
    recursive: true,
    filter: (source) => {
      const rel = relative(templateDirectory, source);
      const topLevel = rel.split(sep)[0];
      return rel === '' || topLevel === undefined || !excludedTemplateRoots.has(topLevel);
    },
  });
}

function patchCandidateDependencies(consumerDirectory: string): void {
  const file = join(consumerDirectory, 'package.json');
  const manifest = JSON.parse(readFileSync(file, 'utf8')) as PackageJson;
  manifest.devDependencies ??= {};
  for (const [name, directory] of Object.entries(candidates)) {
    manifest.devDependencies[name] = pathToFileURL(directory).href;
  }
  writeFileSync(file, `${JSON.stringify(manifest, null, 2)}\n`);
}

function verifyPhysicalCandidates(consumerDirectory: string): void {
  for (const name of Object.keys(candidates)) {
    const installed = join(consumerDirectory, 'node_modules', ...name.split('/'));
    if (lstatSync(installed).isSymbolicLink()) throw new Error(`${name} was installed as a symlink: ${installed}`);
    assertInside(consumerDirectory, realpathSync(installed), `${name} real installation`);
  }
}

function verifyRuntimeResolution(consumerDirectory: string): void {
  const requireFromConsumer = createRequire(join(consumerDirectory, 'consumer.cjs'));
  const runtime = requireFromConsumer.resolve('@fhevm/hardhat-plugin');
  const expected = join('node_modules', '@fhevm', 'hardhat-plugin', '_cjs', 'index.js');
  if (!normalize(runtime).endsWith(normalize(expected))) {
    throw new Error(`@fhevm/hardhat-plugin runtime resolved to ${runtime}, expected ${expected}`);
  }
  assertInside(consumerDirectory, realpathSync(runtime), 'plugin runtime');
}

function verifyTypeResolution(consumerDirectory: string): void {
  writeFileSync(join(consumerDirectory, 'consumer-resolution.ts'), `import '@fhevm/hardhat-plugin';\n`);
  writeFileSync(
    join(consumerDirectory, 'tsconfig.consumer.json'),
    `${JSON.stringify(
      {
        compilerOptions: {
          module: 'commonjs',
          moduleResolution: 'node10',
          noEmit: true,
          skipLibCheck: true,
          target: 'es2022',
          types: ['node'],
        },
        files: ['./consumer-resolution.ts'],
      },
      null,
      2,
    )}\n`,
  );
  const result = spawnSync('npm', ['exec', '--', 'tsc', '--project', 'tsconfig.consumer.json', '--traceResolution'], {
    cwd: consumerDirectory,
    encoding: 'utf8',
    maxBuffer: 16 * 1024 * 1024,
  });
  if (result.error !== undefined) throw result.error;
  const trace = `${result.stdout}${result.stderr}`;
  const expected = normalize(join('node_modules', '@fhevm', 'hardhat-plugin', '_types', 'index.d.ts'));
  if (!normalize(trace).includes(expected)) {
    throw new Error(`TypeScript did not resolve @fhevm/hardhat-plugin to ${expected}`);
  }
  const sourceFallback = normalize(join(candidates['@fhevm/hardhat-plugin'], 'src', 'index.ts'));
  if (normalize(trace).includes(sourceFallback)) {
    throw new Error(`TypeScript escaped the installed package and resolved source at ${sourceFallback}`);
  }
  if (result.status !== 0) {
    const diagnostics = trace
      .split('\n')
      .filter((line) => /error TS\d+:/.test(line))
      .join('\n');
    throw new Error(
      `TypeScript consumer compilation failed:\n${diagnostics.length > 0 || `exit code ${result.status}`}`,
    );
  }
}

function runNpm(directory: string, args: readonly string[]): void {
  console.log(`npm ${args.join(' ')} (${directory})`);
  execFileSync('npm', [...args], { cwd: directory, stdio: 'inherit' });
}

function assertInside(root: string, candidate: string, label: string): void {
  const rel = relative(realpathSync(root), candidate);
  if (rel === '..' || rel.startsWith(`..${sep}`) || isAbsolute(rel)) {
    throw new Error(`${label} escaped the isolated consumer: ${candidate}`);
  }
}

function normalize(value: string): string {
  return value.split(sep).join('/');
}

main();

import { execFileSync } from 'node:child_process';
import {
  cpSync,
  existsSync,
  lstatSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  realpathSync,
  renameSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, isAbsolute, join, relative, resolve, sep } from 'node:path';
import { pathToFileURL } from 'node:url';

import type { NpmManifest } from '../manifest.ts';
import { type ModuleKind, consumerModuleKinds } from './module-kind.ts';
import { type LoadedPackage, dependencyDeclarations, loadPackages, readPackageJson } from './npm.ts';

export type TestConsumerFixture = {
  readonly owner: LoadedPackage;
  readonly published: LoadedPackage;
  readonly fixture: LoadedPackage;
  readonly moduleKind: ModuleKind;
};

export type PrepareTestConsumerOptions = {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly packageSelector?: string;
  readonly output?: string;
  readonly testFile?: string;
  readonly force: boolean;
  readonly buildPackage: boolean;
  readonly run: boolean;
  readonly list: boolean;
  readonly ci: boolean;
};

export type RegenerateTestConsumerPackageLocksOptions = {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly packageSelector?: string;
};

export type NpmRunner = (directory: string, args: readonly string[]) => void;

const excludedFixtureRoots = new Set(['dist', 'node_modules']);
const outputMarker = '.fhevm-npm-test-consumer.json';
const managedRootMarker = '.fhevm-npm-test-consumer-root.json';
const managedOutputRoot = join(tmpdir(), 'fhevm-npm-test-consumer');

export function prepareTestConsumer(options: PrepareTestConsumerOptions): void {
  const fixtures = findTestConsumerFixtures(options.workspaceRoot, options.manifest);
  if (options.list) {
    printFixtures(fixtures);
    return;
  }
  if (options.packageSelector === undefined) {
    throw new Error("test-consumer requires a package selector; use '--list' to see available consumers");
  }

  const selected = selectTestConsumerFixtures(fixtures, options.packageSelector);
  if (options.testFile !== undefined && selected.length !== 1) {
    throw new Error("'--test-file' requires a selector that resolves to exactly one CJS or ESM consumer fixture");
  }
  if (options.testFile !== undefined && selected[0]?.fixture.packageJson.scripts?.['test:file']?.trim() === '') {
    throw new Error(`Consumer fixture '${selected[0].fixture.key}' has an empty 'test:file' script`);
  }
  if (options.testFile !== undefined && selected[0]?.fixture.packageJson.scripts?.['test:file'] === undefined) {
    throw new Error(`Consumer fixture '${selected[0]?.fixture.key}' does not define a 'test:file' script`);
  }
  const first = selected[0];
  if (first === undefined) throw new Error('unreachable');
  const planned = selected.map((fixture) => {
    const output =
      options.output === undefined || selected.length === 1 ? options.output : join(options.output, fixture.moduleKind);
    const testFile =
      options.testFile === undefined
        ? undefined
        : resolveConsumerTestFile(options.testFile, options.workspaceRoot, fixture.fixture.directory);
    const testFileLabel =
      testFile === undefined
        ? undefined
        : workspaceRelativeDisplayPath(options.workspaceRoot, join(fixture.fixture.directory, testFile));
    return { fixture, testFile, testFileLabel, ...resolveDestination(output, fixture.owner.key, fixture.moduleKind) };
  });
  for (const { destination, managed } of planned) {
    if (managed) removeExistingManagedDestination(destination);
  }
  if (options.buildPackage) {
    console.log('\n🔨 Building candidates once before the serial consumer runs.\n');
    runNpm(first.owner.directory, ['run', 'build']);
  }

  for (const [index, { fixture, testFile, testFileLabel, destination, managed }] of planned.entries()) {
    const serialIndex = index + 1;
    const startedAt = Date.now();
    let succeeded = false;
    try {
      printConsumerStartBanner(fixture, destination, testFileLabel, options.ci, serialIndex, planned.length);
      prepareDestination(destination, options.force, managed);
      copyFixture(fixture.fixture.directory, destination);
      writeFileSync(
        join(destination, outputMarker),
        `${JSON.stringify(
          { owner: fixture.owner.key, fixture: fixture.fixture.key, moduleKind: fixture.moduleKind },
          null,
          2,
        )}\n`,
      );
      patchLocalDependencies(destination, fixture.fixture.directory, fixture.published, options.ci);
      runNpm(destination, consumerInstallArguments(options.ci));
      verifyPhysicalInstallation(destination, fixture.published);

      if (options.run) {
        runNpm(destination, testFile === undefined ? ['test'] : ['run', 'test:file', '--', testFile]);
        console.log(
          `✅ ${fixture.moduleKind.toUpperCase()} consumer test passed for ${fixture.published.packageJson.name}`,
        );
      } else {
        console.log(
          `✅ Installed ${fixture.published.packageJson.name} for ${fixture.moduleKind.toUpperCase()} inspection`,
        );
        console.log(`Consumer directory: ${destination}`);
        console.log(
          `Run manually: cd ${shellQuote(destination)} && ${
            testFile === undefined ? 'npm test' : `npm run test:file -- ${shellQuote(testFile)}`
          }`,
        );
      }
      succeeded = true;
    } finally {
      let cleanupSucceeded = true;
      try {
        if (options.run && managed) {
          removeOwnedDestination(destination);
          console.log(`Cleaned consumer directory: ${destination}`);
        }
      } catch (error) {
        cleanupSucceeded = false;
        throw error;
      } finally {
        printConsumerEndBanner(
          fixture,
          destination,
          serialIndex,
          planned.length,
          succeeded && cleanupSucceeded,
          Date.now() - startedAt,
        );
      }
    }
  }
}

function printConsumerStartBanner(
  fixture: TestConsumerFixture,
  destination: string,
  testFile: string | undefined,
  ci: boolean,
  serialIndex: number,
  serialTotal: number,
): void {
  const separator = '================================================================================';
  const testName = fixture.fixture.packageJson.name ?? fixture.fixture.key;
  console.log(`
🟦${separator}
🚦 FHEVM TEST CONSUMER START — SERIAL RUN ${String(serialIndex)}/${String(serialTotal)} — ${fixture.moduleKind.toUpperCase()}
🧪 TEST: ${testName}
📂 RUNNING IN: ${destination}
📋 FIXTURE SOURCE: ${fixture.fixture.directory}
${testFile === undefined ? '' : `🎯 TEST FILE: ${testFile}\n`}🔒 INSTALL MODE: ${ci ? 'COMMITTED LOCK (npm ci)' : 'FRESH LOCK (npm install)'}
🟦${separator}
`);
}

function printConsumerEndBanner(
  fixture: TestConsumerFixture,
  destination: string,
  serialIndex: number,
  serialTotal: number,
  succeeded: boolean,
  elapsedMilliseconds: number,
): void {
  const separator = '================================================================================';
  const color = succeeded ? '🟩' : '🟥';
  const result = succeeded ? '✅ PASSED' : '❌ FAILED';
  const testName = fixture.fixture.packageJson.name ?? fixture.fixture.key;
  console.log(`
${color}${separator}
🏁 FHEVM TEST CONSUMER END — SERIAL RUN ${String(serialIndex)}/${String(serialTotal)} — ${fixture.moduleKind.toUpperCase()} — ${result}
🧪 TEST: ${testName}
📂 RAN IN: ${destination}
⏱️ ELAPSED: ${(elapsedMilliseconds / 1000).toFixed(1)}s
${color}${separator}
`);
}

export function regenerateTestConsumerPackageLocks(
  options: RegenerateTestConsumerPackageLocksOptions,
  runner: NpmRunner = runNpm,
): void {
  const fixtures = findTestConsumerFixtures(options.workspaceRoot, options.manifest);
  const selected =
    options.packageSelector === undefined ? fixtures : selectTestConsumerFixtures(fixtures, options.packageSelector);
  if (selected.length === 0) throw new Error('No checked-in test-consumer fixtures are available.');

  for (const { fixture, moduleKind } of selected) {
    regenerateFixturePackageLock(fixture.directory, runner);
    console.log(`✅ Regenerated ${fixture.key}/package-lock.json (${moduleKind.toUpperCase()})`);
  }
}

export function regenerateFixturePackageLock(fixtureDirectory: string, runner: NpmRunner = runNpm): void {
  const stagingDirectory = mkdtempSync(join(dirname(fixtureDirectory), '.fhevm-npm-lock-'));
  try {
    copyFixture(fixtureDirectory, stagingDirectory);
    rmSync(join(stagingDirectory, 'package-lock.json'), { force: true });
    runner(stagingDirectory, [
      'install',
      '--install-links',
      '--package-lock-only',
      '--ignore-scripts',
      '--no-audit',
      '--no-fund',
    ]);
    validateGeneratedPackageLock(stagingDirectory);
    runner(stagingDirectory, ['ci', '--install-links', '--ignore-scripts', '--no-audit', '--no-fund']);
    runner(stagingDirectory, ['ls', '--all', '--install-links']);

    const generatedLock = join(stagingDirectory, 'package-lock.json');
    const destinationLock = join(fixtureDirectory, 'package-lock.json');
    const pendingLock = join(fixtureDirectory, `.package-lock.json.${String(process.pid)}.tmp`);
    try {
      writeFileSync(pendingLock, readFileSync(generatedLock));
      renameSync(pendingLock, destinationLock);
    } finally {
      rmSync(pendingLock, { force: true });
    }
  } finally {
    rmSync(stagingDirectory, { recursive: true, force: true });
  }
}

function validateGeneratedPackageLock(stagingDirectory: string): void {
  const packageJson = readPackageJson(join(stagingDirectory, 'package.json'));
  const packageLockPath = join(stagingDirectory, 'package-lock.json');
  if (!existsSync(packageLockPath)) throw new Error(`npm did not generate ${packageLockPath}`);
  const packageLock = JSON.parse(readFileSync(packageLockPath, 'utf8')) as {
    lockfileVersion?: unknown;
    packages?: Record<
      string,
      {
        dependencies?: Record<string, string>;
        devDependencies?: Record<string, string>;
        optionalDependencies?: Record<string, string>;
        peerDependencies?: Record<string, string>;
        resolved?: string;
        link?: boolean;
      }
    >;
  };
  if (packageLock.lockfileVersion !== 3) {
    throw new Error(`${packageLockPath} must use lockfileVersion 3; found ${String(packageLock.lockfileVersion)}`);
  }
  const root = packageLock.packages?.[''];
  if (root === undefined) throw new Error(`${packageLockPath} has no root package entry`);

  for (const declaration of dependencyDeclarations(packageJson)) {
    if (!declaration.spec.startsWith('file:')) continue;
    if (root[declaration.field]?.[declaration.name] !== declaration.spec) {
      throw new Error(
        `${packageLockPath} does not preserve '${declaration.name}' as "${declaration.spec}" in '${declaration.field}'`,
      );
    }
    const installed = packageLock.packages?.[`node_modules/${declaration.name}`];
    if (installed === undefined)
      throw new Error(`${packageLockPath} does not lock local package '${declaration.name}'`);
    if (installed.link === true) {
      throw new Error(
        `${packageLockPath} locks local package '${declaration.name}' as a link; --install-links is required`,
      );
    }
    if (installed.resolved !== declaration.spec) {
      throw new Error(
        `${packageLockPath} resolves local package '${declaration.name}' as "${String(installed.resolved)}" instead of "${declaration.spec}"`,
      );
    }
  }
}

export function findTestConsumerFixtures(workspaceRoot: string, manifest: NpmManifest): readonly TestConsumerFixture[] {
  const packages = loadPackages(workspaceRoot, manifest);
  const byKey = new Map(packages.map((pkg) => [pkg.key, pkg]));
  const fixtures: TestConsumerFixture[] = [];

  for (const owner of packages) {
    if (owner.inventory.kind !== 'dev' || owner.packageJson.scripts?.['test:consumer']?.trim() === '') continue;
    if (owner.packageJson.scripts?.['test:consumer'] === undefined || owner.inventory.publishedRelPath === undefined) {
      continue;
    }
    const published = byKey.get(owner.inventory.publishedRelPath);
    if (published === undefined) continue;
    for (const moduleKind of consumerModuleKinds(published.packageJson)) {
      const fixture = byKey.get(`${owner.key}/test-consumer/${moduleKind}`);
      if (fixture === undefined || !existsSync(join(fixture.directory, 'package.json'))) continue;
      fixtures.push({ owner, published, fixture, moduleKind });
    }
  }

  return fixtures.sort((left, right) => left.owner.key.localeCompare(right.owner.key));
}

export function selectTestConsumerFixtures(
  fixtures: readonly TestConsumerFixture[],
  selector: string,
): readonly TestConsumerFixture[] {
  const normalized = normalizeSelector(selector);
  const matches = fixtures.filter((fixture) => fixtureSelectors(fixture).has(normalized));
  if (matches.length === 0) {
    throw new Error(`No test consumer matches '${selector}'. Use 'test-consumer --list' to see available consumers.`);
  }
  const owners = new Set(matches.map(({ owner }) => owner.key));
  if (owners.size > 1) {
    throw new Error(
      `Test consumer selector '${selector}' is ambiguous; use an owner path: ${[...owners].sort().join(', ')}`,
    );
  }
  return matches.sort((left, right) => left.moduleKind.localeCompare(right.moduleKind));
}

export function resolveConsumerTestFile(
  testFile: string,
  workspaceRoot: string,
  fixtureDirectory: string,
): string {
  const fixtureRoot = realpathSync(fixtureDirectory);
  const candidates = isAbsolute(testFile)
    ? [resolve(testFile)]
    : [resolve(workspaceRoot, testFile), resolve(fixtureDirectory, testFile)];
  const existing = [...new Set(candidates.filter((candidate) => existsSync(candidate)))];
  if (existing.length === 0) {
    throw new Error(`Consumer test file does not exist: ${testFile}`);
  }
  if (existing.length > 1) {
    throw new Error(`Consumer test file path is ambiguous: ${testFile}`);
  }

  const candidate = existing[0];
  if (candidate === undefined) throw new Error('unreachable');
  const fileStats = lstatSync(candidate);
  if (fileStats.isSymbolicLink() || !fileStats.isFile()) {
    throw new Error(`Consumer test file must be a regular, non-symlinked file: ${candidate}`);
  }
  const resolvedFile = realpathSync(candidate);
  const relativeFile = relative(fixtureRoot, resolvedFile);
  if (relativeFile === '..' || relativeFile.startsWith(`..${sep}`) || isAbsolute(relativeFile)) {
    throw new Error(`Consumer test file must be inside ${fixtureDirectory}: ${candidate}`);
  }
  const topLevel = relativeFile.split(sep)[0];
  if (topLevel !== undefined && excludedFixtureRoots.has(topLevel)) {
    throw new Error(`Consumer test file is excluded from the copied fixture: ${candidate}`);
  }
  return relativeFile.split(sep).join('/');
}

function fixtureSelectors(fixture: TestConsumerFixture): ReadonlySet<string> {
  return new Set(
    [
      fixture.owner.key,
      fixture.owner.packageJson.name,
      fixture.published.key,
      fixture.published.packageJson.name,
      fixture.fixture.key,
      fixture.fixture.packageJson.name,
    ]
      .filter((value): value is string => value !== undefined)
      .flatMap((value) => [normalizeSelector(value), normalizeSelector(value.replace(/^\.\//, ''))]),
  );
}

function normalizeSelector(selector: string): string {
  return selector.replace(/\/$/, '');
}

function printFixtures(fixtures: readonly TestConsumerFixture[]): void {
  if (fixtures.length === 0) {
    console.log('No checked-in test-consumer fixtures are available.');
    return;
  }
  for (const { owner, published, fixture, moduleKind } of fixtures) {
    console.log(
      `${owner.key} (${owner.packageJson.name}) -> ${fixture.key} [${published.packageJson.name}, ${moduleKind.toUpperCase()}]`,
    );
  }
}

function resolveDestination(
  output: string | undefined,
  ownerKey: string,
  moduleKind: ModuleKind,
): { readonly destination: string; readonly managed: boolean } {
  if (output !== undefined) return { destination: resolve(output), managed: false };
  prepareManagedRoot();
  const ownerDirectory = ownerKey.replace(/^\.\//, '').replaceAll('/', '-');
  return { destination: join(managedOutputRoot, ownerDirectory, moduleKind), managed: true };
}

function prepareManagedRoot(): void {
  if (existsSync(managedOutputRoot)) {
    if (lstatSync(managedOutputRoot).isSymbolicLink()) {
      throw new Error(`Refusing symlinked test-consumer root: ${managedOutputRoot}`);
    }
    if (!existsSync(join(managedOutputRoot, managedRootMarker))) {
      throw new Error(`Refusing unmarked test-consumer root: ${managedOutputRoot}`);
    }
    return;
  }
  mkdirSync(managedOutputRoot, { recursive: true });
  writeFileSync(join(managedOutputRoot, managedRootMarker), `${JSON.stringify({ managedBy: 'fhevm-npm' }, null, 2)}\n`);
}

function prepareDestination(destination: string, force: boolean, managed: boolean): void {
  if (!existsSync(destination)) {
    mkdirSync(destination, { recursive: true });
    if (managed) assertManagedDestination(destination);
    writeFileSync(join(destination, outputMarker), `${JSON.stringify({ managedBy: 'fhevm-npm' }, null, 2)}\n`);
    return;
  }
  if (managed) assertManagedDestination(destination);
  if (readdirSync(destination).length === 0) {
    writeFileSync(join(destination, outputMarker), `${JSON.stringify({ managedBy: 'fhevm-npm' }, null, 2)}\n`);
    return;
  }
  if (!managed && !force)
    throw new Error(`Output directory already exists: ${destination}. Pass '--force' to replace it.`);
  assertSafeOutput(destination);
  if (!existsSync(join(destination, outputMarker))) {
    throw new Error(`Refusing to replace an unmarked directory: ${destination}`);
  }
  rmSync(destination, { recursive: true, force: true });
  mkdirSync(destination, { recursive: true });
  writeFileSync(join(destination, outputMarker), `${JSON.stringify({ managedBy: 'fhevm-npm' }, null, 2)}\n`);
}

function assertSafeOutput(destination: string): void {
  const parsed = resolve(destination);
  if (parsed === dirname(parsed)) throw new Error(`Refusing to replace filesystem root: ${destination}`);
}

function removeOwnedDestination(destination: string): void {
  assertSafeOutput(destination);
  assertManagedDestination(destination);
  if (!existsSync(join(destination, outputMarker))) {
    throw new Error(`Refusing to remove an unmarked directory: ${destination}`);
  }
  rmSync(destination, { recursive: true, force: true });
}

function removeExistingManagedDestination(destination: string): void {
  if (!existsSync(destination)) return;
  removeOwnedDestination(destination);
  console.log(`Removed previous consumer directory: ${destination}`);
}

function assertManagedDestination(destination: string): void {
  if (lstatSync(destination).isSymbolicLink()) {
    throw new Error(`Refusing symlinked test-consumer directory: ${destination}`);
  }
  const rel = relative(realpathSync(managedOutputRoot), realpathSync(destination));
  if (rel === '..' || rel.startsWith(`..${sep}`) || isAbsolute(rel)) {
    throw new Error(`Test-consumer directory resolves outside ${managedOutputRoot}: ${destination}`);
  }
}

function copyFixture(source: string, destination: string): void {
  cpSync(source, destination, {
    recursive: true,
    filter: (candidate) => {
      const topLevel = relative(source, candidate).split(sep)[0];
      return topLevel === undefined || topLevel === '' || !excludedFixtureRoots.has(topLevel);
    },
  });
}

export function consumerInstallArguments(ci: boolean): readonly string[] {
  return [ci ? 'ci' : 'install', '--install-links', '--no-audit', '--no-fund'];
}

function patchLocalDependencies(
  destination: string,
  fixtureDirectory: string,
  published: LoadedPackage,
  useCommittedLock: boolean,
): void {
  const packageName = published.packageJson.name;
  if (packageName === undefined) throw new Error(`Published package '${published.key}' has no name`);
  const packageJsonPath = join(destination, 'package.json');
  const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8')) as {
    dependencies?: Record<string, string>;
    devDependencies?: Record<string, string>;
    optionalDependencies?: Record<string, string>;
    peerDependencies?: Record<string, string>;
  };
  const rewritten = new Map<string, string>();
  for (const field of ['dependencies', 'devDependencies', 'optionalDependencies', 'peerDependencies'] as const) {
    for (const [dependencyName, spec] of Object.entries(packageJson[field] ?? {})) {
      const absoluteSpec =
        dependencyName === packageName
          ? pathToFileURL(published.directory).href
          : absoluteLocalFileSpec(spec, fixtureDirectory);
      if (absoluteSpec === undefined) continue;
      packageJson[field]![dependencyName] = absoluteSpec;
      rewritten.set(dependencyName, absoluteSpec);
    }
  }
  if (!rewritten.has(packageName)) {
    throw new Error(`${selectedManifestLabel(packageJsonPath)} does not declare '${packageName}'`);
  }
  writeFileSync(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`);
  if (useCommittedLock) {
    patchLocalDependencyLock(destination, rewritten);
  } else {
    rmSync(join(destination, 'package-lock.json'), { force: true });
  }
}

function patchLocalDependencyLock(destination: string, rewritten: ReadonlyMap<string, string>): void {
  const packageLockPath = join(destination, 'package-lock.json');
  if (!existsSync(packageLockPath)) {
    throw new Error(`Consumer fixture has no package-lock.json: ${packageLockPath}`);
  }
  const packageLock = JSON.parse(readFileSync(packageLockPath, 'utf8')) as {
    packages?: Record<
      string,
      {
        dependencies?: Record<string, string>;
        devDependencies?: Record<string, string>;
        optionalDependencies?: Record<string, string>;
        peerDependencies?: Record<string, string>;
        resolved?: string;
      }
    >;
  };
  const root = packageLock.packages?.[''];
  if (root === undefined) throw new Error(`Consumer lockfile ${packageLockPath} has no root package`);
  for (const [packageName, absoluteSpec] of rewritten) {
    const fields = ['dependencies', 'devDependencies', 'optionalDependencies', 'peerDependencies'] as const;
    const field = fields.find((candidate) => root[candidate]?.[packageName] !== undefined);
    const lockedDependency = packageLock.packages?.[`node_modules/${packageName}`];
    if (field === undefined || lockedDependency === undefined) {
      throw new Error(`Consumer lockfile ${packageLockPath} does not lock '${packageName}'`);
    }
    root[field]![packageName] = absoluteSpec;
    lockedDependency.resolved = absoluteSpec;
  }
  writeFileSync(packageLockPath, `${JSON.stringify(packageLock, null, 2)}\n`);
}

function absoluteLocalFileSpec(spec: string, fixtureDirectory: string): string | undefined {
  if (!spec.startsWith('file:')) return undefined;
  return pathToFileURL(resolve(fixtureDirectory, spec.slice('file:'.length))).href;
}

function selectedManifestLabel(packageJsonPath: string): string {
  return `Consumer manifest ${packageJsonPath}`;
}

function verifyPhysicalInstallation(destination: string, published: LoadedPackage): void {
  const packageName = published.packageJson.name;
  if (packageName === undefined) throw new Error(`Published package '${published.key}' has no name`);
  const installed = join(destination, 'node_modules', ...packageName.split('/'));
  if (lstatSync(installed).isSymbolicLink()) throw new Error(`${packageName} was installed as a symlink: ${installed}`);
  const rel = relative(realpathSync(destination), realpathSync(installed));
  if (rel === '..' || rel.startsWith(`..${sep}`) || isAbsolute(rel)) {
    throw new Error(`${packageName} resolved outside the prepared consumer: ${realpathSync(installed)}`);
  }
}

function runNpm(directory: string, args: readonly string[]): void {
  console.log(`npm ${args.join(' ')} (${directory})`);
  execFileSync('npm', [...args], { cwd: directory, stdio: 'inherit' });
}

function shellQuote(value: string): string {
  return `'${value.replaceAll("'", `'\\''`)}'`;
}

function workspaceRelativeDisplayPath(workspaceRoot: string, absolutePath: string): string {
  const relativePath = relative(workspaceRoot, absolutePath).split(sep).join('/');
  return relativePath.startsWith('.') ? relativePath : `./${relativePath}`;
}

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
import {
  type DependencyDeclaration,
  type LoadedPackage,
  dependencyDeclarations,
  loadPackages,
  readPackageJson,
} from './npm.ts';
import { type Verbosity, hasDetailedOutput, hasProgress, npmVerbosityArguments } from './verbosity.ts';

export type TestConsumerFixture = {
  readonly owner: LoadedPackage;
  readonly published: LoadedPackage;
  readonly fixture: LoadedPackage;
  readonly moduleKind: ModuleKind;
};

export type LinkedDependency = {
  readonly declaration: DependencyDeclaration;
  readonly declaredBy: LoadedPackage;
  readonly direct: boolean;
  readonly package: LoadedPackage;
};

export type TestConsumerTarget = {
  readonly kind: 'fixture' | 'project';
  readonly source: LoadedPackage;
  readonly owner?: LoadedPackage;
  readonly published?: LoadedPackage;
  readonly moduleKind: ModuleKind | 'dual';
  readonly linkedDependencies: readonly LinkedDependency[];
};

export type PrepareTestConsumerOptions = {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly packageSelector?: string;
  readonly output?: string;
  readonly testFile?: string;
  readonly force: boolean;
  readonly buildLinkedDependencies: boolean;
  readonly run: boolean;
  readonly list: boolean;
  readonly ci: boolean;
  readonly verbosity: Verbosity;
};

export type RegenerateTestConsumerPackageLocksOptions = {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly packageSelector?: string;
  readonly verbosity: Verbosity;
};

export type NpmRunner = (directory: string, args: readonly string[], verbosity?: Verbosity) => void;
export type MakeBuildRunner = (workspaceRoot: string, packageKey: string, verbosity?: Verbosity) => void;

const excludedFixtureRoots = new Set(['dist', 'node_modules']);
const outputMarker = '.fhevm-npm-test-consumer.json';
const managedRootMarker = '.fhevm-npm-test-consumer-root.json';
const managedOutputRoot = join(tmpdir(), 'fhevm-npm-test-consumer');

export function prepareTestConsumer(options: PrepareTestConsumerOptions): void {
  const packages = loadPackages(options.workspaceRoot, options.manifest);
  const targets = findTestConsumerTargets(options.workspaceRoot, packages);
  if (options.list) {
    printTargets(targets);
    return;
  }
  if (options.packageSelector === undefined) {
    throw new Error("test-consumer requires a package selector; use '--list' to see available consumers");
  }

  const selected = selectTestConsumerTargets(targets, options.packageSelector);
  for (const target of selected) {
    const testScript = target.source.packageJson.scripts?.test;
    if (testScript === undefined || testScript.trim() === '') {
      throw new Error(`Consumer '${target.source.key}' must define a non-empty 'test' script`);
    }
    if (options.ci && !existsSync(join(target.source.directory, 'package-lock.json'))) {
      throw new Error(`Consumer '${target.source.key}' requires a committed package-lock.json for '--ci'`);
    }
  }
  if (options.testFile !== undefined && selected.length !== 1) {
    throw new Error("'--test-file' requires a selector that resolves to exactly one consumer project");
  }
  if (options.testFile !== undefined && selected[0]?.source.packageJson.scripts?.['test:file']?.trim() === '') {
    throw new Error(`Consumer '${selected[0].source.key}' has an empty 'test:file' script`);
  }
  if (options.testFile !== undefined && selected[0]?.source.packageJson.scripts?.['test:file'] === undefined) {
    throw new Error(`Consumer '${selected[0]?.source.key}' does not define a 'test:file' script`);
  }
  const planned = selected.map((target) => {
    const output =
      options.output === undefined || selected.length === 1 ? options.output : join(options.output, target.moduleKind);
    const testFile =
      options.testFile === undefined
        ? undefined
        : resolveConsumerTestFile(options.testFile, options.workspaceRoot, target.source.directory);
    const testFileLabel =
      testFile === undefined
        ? undefined
        : workspaceRelativeDisplayPath(options.workspaceRoot, join(target.source.directory, testFile));
    const destinationOwner = target.owner?.key ?? target.source.key;
    return { target, testFile, testFileLabel, ...resolveDestination(output, destinationOwner, target.moduleKind) };
  });
  for (const { destination, managed } of planned) {
    if (managed) removeExistingManagedDestination(destination);
  }
  if (options.buildLinkedDependencies) {
    const owners = linkedDependencyBuildOrder(
      packages,
      selected.flatMap((target) => target.linkedDependencies),
    );
    buildLinkedDependenciesWithMake(options.workspaceRoot, owners, options.verbosity);
  }

  for (const [index, { target, testFile, testFileLabel, destination, managed }] of planned.entries()) {
    const serialIndex = index + 1;
    const startedAt = Date.now();
    let succeeded = false;
    try {
      printConsumerStartBanner(target, destination, testFileLabel, options.ci, serialIndex, planned.length);
      prepareDestination(destination, options.force, managed);
      copyFixture(target.source.directory, destination);
      writeFileSync(
        join(destination, outputMarker),
        `${JSON.stringify(
          {
            consumer: target.source.key,
            kind: target.kind,
            moduleKind: target.moduleKind,
            linkedDependencies: target.linkedDependencies.map((dependency) => dependency.package.key),
          },
          null,
          2,
        )}\n`,
      );
      patchLocalDependencies(destination, target.source.directory, target.linkedDependencies, options.ci);
      runNpm(destination, consumerInstallArguments(options.ci), options.verbosity);
      for (const dependency of target.linkedDependencies) verifyPhysicalInstallation(destination, dependency.package);

      if (options.run) {
        runNpm(
          destination,
          testFile === undefined ? ['test'] : ['run', 'test:file', '--', testFile],
          options.verbosity,
        );
        console.log(`✅ ${target.moduleKind.toUpperCase()} consumer test passed: ${consumerName(target)}`);
      } else {
        console.log(`✅ Installed linked dependencies for ${target.moduleKind.toUpperCase()} inspection`);
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
          target,
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
  target: TestConsumerTarget,
  destination: string,
  testFile: string | undefined,
  ci: boolean,
  serialIndex: number,
  serialTotal: number,
): void {
  const separator = '================================================================================';
  const testName = consumerName(target);
  const dependencies = target.linkedDependencies
    .map((dependency) => {
      const origin = dependency.direct ? 'direct' : `transitive via ${dependency.declaredBy.packageJson.name}`;
      return `  - ${dependency.declaration.name} -> ${dependency.package.key} (${origin})`;
    })
    .join('\n');
  console.log(`
🟦${separator}
🚦 FHEVM TEST CONSUMER START — SERIAL RUN ${String(serialIndex)}/${String(serialTotal)} — ${target.moduleKind.toUpperCase()}
🧪 TEST: ${testName}
📂 RUNNING IN: ${destination}
📋 CONSUMER SOURCE: ${target.source.directory}
🔗 LINKED DEPENDENCIES:
${dependencies === '' ? '  - none' : dependencies}
${testFile === undefined ? '' : `🎯 TEST FILE: ${testFile}\n`}🔒 INSTALL MODE: ${ci ? 'COMMITTED LOCK (npm ci)' : 'FRESH LOCK (npm install)'}
🟦${separator}
`);
}

function printConsumerEndBanner(
  target: TestConsumerTarget,
  destination: string,
  serialIndex: number,
  serialTotal: number,
  succeeded: boolean,
  elapsedMilliseconds: number,
): void {
  const separator = '================================================================================';
  const color = succeeded ? '🟩' : '🟥';
  const result = succeeded ? '✅ PASSED' : '❌ FAILED';
  const testName = consumerName(target);
  console.log(`
${color}${separator}
🏁 FHEVM TEST CONSUMER END — SERIAL RUN ${String(serialIndex)}/${String(serialTotal)} — ${target.moduleKind.toUpperCase()} — ${result}
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
  const packages = loadPackages(options.workspaceRoot, options.manifest);
  const targets = findTestConsumerTargets(options.workspaceRoot, packages);
  const selected =
    options.packageSelector === undefined
      ? targets.filter((target) => target.kind === 'fixture')
      : selectTestConsumerTargets(targets, options.packageSelector);
  if (selected.length === 0) throw new Error('No checked-in test consumers are available.');

  for (const target of selected) {
    regenerateFixturePackageLock(target.source.directory, runner, target.linkedDependencies, options.verbosity);
    console.log(`✅ Regenerated ${target.source.key}/package-lock.json (${target.moduleKind.toUpperCase()})`);
  }
}

export function regenerateFixturePackageLock(
  fixtureDirectory: string,
  runner: NpmRunner = runNpm,
  linkedDependencies: readonly LinkedDependency[] = [],
  verbosity: Verbosity = 0,
): void {
  const stagingDirectory = mkdtempSync(join(dirname(fixtureDirectory), '.fhevm-npm-lock-'));
  try {
    copyFixture(fixtureDirectory, stagingDirectory);
    rmSync(join(stagingDirectory, 'package-lock.json'), { force: true });
    const injected = injectTransitiveDependencies(stagingDirectory, linkedDependencies);
    runner(
      stagingDirectory,
      ['install', '--install-links', '--package-lock-only', '--ignore-scripts', '--no-audit', '--no-fund'],
      verbosity,
    );
    validateGeneratedPackageLock(stagingDirectory);
    runner(stagingDirectory, ['ci', '--install-links', '--ignore-scripts', '--no-audit', '--no-fund'], verbosity);
    runner(stagingDirectory, ['ls', '--all', '--install-links'], verbosity);
    removeInjectedRootLockDeclarations(stagingDirectory, injected);

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

type InjectedRootDependency = {
  readonly declaration: DependencyDeclaration;
  readonly originalSpec?: string;
};

function injectTransitiveDependencies(
  stagingDirectory: string,
  linkedDependencies: readonly LinkedDependency[],
): readonly InjectedRootDependency[] {
  const packageJsonPath = join(stagingDirectory, 'package.json');
  const packageJson = readPackageJson(packageJsonPath);
  const injected: InjectedRootDependency[] = [];
  for (const dependency of linkedDependencies) {
    if (dependency.direct) continue;
    const existing = dependencyDeclarations(packageJson).filter(
      (declaration) => declaration.name === dependency.declaration.name,
    );
    if (existing.length > 1) {
      throw new Error(
        `Consumer manifest ${packageJsonPath} declares '${dependency.declaration.name}' in multiple dependency fields`,
      );
    }
    const rootDeclaration =
      existing[0] ?? ({ field: 'devDependencies', name: dependency.declaration.name, spec: '' } as const);
    packageJson[rootDeclaration.field] ??= {};
    packageJson[rootDeclaration.field]![dependency.declaration.name] = relativeFileSpec(
      stagingDirectory,
      dependency.package.directory,
    );
    injected.push({ declaration: rootDeclaration, originalSpec: existing[0]?.spec });
  }
  writeFileSync(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`);
  return injected;
}

function removeInjectedRootLockDeclarations(
  stagingDirectory: string,
  injected: readonly InjectedRootDependency[],
): void {
  if (injected.length === 0) return;
  const packageLockPath = join(stagingDirectory, 'package-lock.json');
  const packageLock = JSON.parse(readFileSync(packageLockPath, 'utf8')) as {
    packages?: Record<
      string,
      {
        dependencies?: Record<string, string>;
        devDependencies?: Record<string, string>;
        optionalDependencies?: Record<string, string>;
        peerDependencies?: Record<string, string>;
      }
    >;
  };
  const root = packageLock.packages?.[''];
  if (root === undefined) throw new Error(`${packageLockPath} has no root package entry`);
  for (const { declaration, originalSpec } of injected) {
    if (originalSpec === undefined) {
      delete root[declaration.field]?.[declaration.name];
    } else {
      root[declaration.field] ??= {};
      root[declaration.field]![declaration.name] = originalSpec;
    }
  }
  writeFileSync(packageLockPath, `${JSON.stringify(packageLock, null, 2)}\n`);
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
  return findFixtures(packages);
}

function findFixtures(packages: readonly LoadedPackage[]): readonly TestConsumerFixture[] {
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

export function findTestConsumerTargets(
  workspaceRoot: string,
  packages: readonly LoadedPackage[],
): readonly TestConsumerTarget[] {
  const fixtures = findFixtures(packages).map((fixture): TestConsumerTarget => ({
    kind: 'fixture',
    source: fixture.fixture,
    owner: fixture.owner,
    published: fixture.published,
    moduleKind: fixture.moduleKind,
    linkedDependencies: resolveLinkedDependencies(workspaceRoot, fixture.fixture, packages),
  }));
  const projects = packages
    .filter((pkg) => !/\/test-consumer\/(?:cjs|esm)$/.test(pkg.key))
    .filter((pkg) => pkg.packageJson.scripts?.test?.trim() !== '')
    .filter((pkg) => pkg.packageJson.scripts?.test !== undefined)
    .filter((pkg) =>
      dependencyDeclarations(pkg.packageJson).some((declaration) => declaration.spec.startsWith('file:')),
    )
    .map((source): TestConsumerTarget => ({
      kind: 'project',
      source,
      moduleKind: source.inventory.type,
      linkedDependencies: resolveLinkedDependencies(workspaceRoot, source, packages),
    }));
  return [...fixtures, ...projects].sort((left, right) => left.source.key.localeCompare(right.source.key));
}

function resolveLinkedDependencies(
  workspaceRoot: string,
  consumer: LoadedPackage,
  packages: readonly LoadedPackage[],
): readonly LinkedDependency[] {
  assertInsideDirectory(workspaceRoot, consumer.directory, `Consumer '${consumer.key}'`);
  const packagesByDirectory = new Map(packages.map((pkg) => [realpathSync(pkg.directory), pkg]));
  const publishedByNameAndVersion = new Map<string, LoadedPackage[]>();
  const publishedByName = new Map<string, LoadedPackage[]>();
  for (const pkg of packages) {
    if (
      pkg.inventory.kind !== 'published' ||
      pkg.packageJson.name === undefined ||
      pkg.packageJson.version === undefined
    ) {
      continue;
    }
    const key = `${pkg.packageJson.name}@${pkg.packageJson.version}`;
    const matches = publishedByNameAndVersion.get(key) ?? [];
    matches.push(pkg);
    publishedByNameAndVersion.set(key, matches);
    const namedMatches = publishedByName.get(pkg.packageJson.name) ?? [];
    namedMatches.push(pkg);
    publishedByName.set(pkg.packageJson.name, namedMatches);
  }
  const selectedByName = new Map<string, LinkedDependency>();
  const queue: LinkedDependency[] = [];

  const add = (dependency: LinkedDependency): void => {
    const packageName = dependency.package.packageJson.name;
    if (packageName === undefined) throw new Error(`Linked package '${dependency.package.key}' has no name`);
    const previous = selectedByName.get(packageName);
    if (previous !== undefined) {
      if (previous.package.key !== dependency.package.key) {
        throw new Error(
          `Consumer '${consumer.key}' resolves '${packageName}' to both '${previous.package.key}' and ` +
            `'${dependency.package.key}', so one physical local candidate cannot satisfy the consumer`,
        );
      }
      return;
    }
    selectedByName.set(packageName, dependency);
    queue.push(dependency);
  };

  for (const declaration of dependencyDeclarations(consumer.packageJson)) {
    if (!declaration.spec.startsWith('file:')) continue;
    add({
      declaration,
      declaredBy: consumer,
      direct: true,
      package: resolveFileDependency(workspaceRoot, consumer, declaration, packagesByDirectory),
    });
  }
  if (queue.length === 0) {
    throw new Error(`Consumer '${consumer.key}' must declare at least one manifest-listed 'file:' dependency`);
  }

  for (let index = 0; index < queue.length; index += 1) {
    const parent = queue[index];
    if (parent === undefined) throw new Error('unreachable');
    for (const declaration of runtimeDependencyDeclarations(parent.package)) {
      let candidate: LoadedPackage | undefined;
      if (declaration.spec.startsWith('file:')) {
        candidate = resolveFileDependency(workspaceRoot, parent.package, declaration, packagesByDirectory);
      } else {
        const matches = publishedByNameAndVersion.get(`${declaration.name}@${declaration.spec}`) ?? [];
        if (matches.length > 1) {
          throw new Error(
            `Dependency '${declaration.name}@${declaration.spec}' from '${parent.package.key}' matches multiple ` +
              `published candidates: ${matches.map((pkg) => pkg.key).join(', ')}`,
          );
        }
        candidate = matches[0];
        if (candidate === undefined) {
          const localVersions = publishedByName.get(declaration.name) ?? [];
          if (localVersions.length > 0) {
            throw new Error(
              `Dependency '${declaration.name}' from '${parent.package.key}' uses '${declaration.spec}', which does not ` +
                `exactly identify a manifest candidate (${localVersions
                  .map((pkg) => `${pkg.key}=${String(pkg.packageJson.version)}`)
                  .join(', ')})`,
            );
          }
        }
      }
      if (candidate !== undefined) {
        add({ declaration, declaredBy: parent.package, direct: false, package: candidate });
      }
    }
  }

  return [...selectedByName.values()].sort((left, right) => left.package.key.localeCompare(right.package.key));
}

function runtimeDependencyDeclarations(pkg: LoadedPackage): readonly DependencyDeclaration[] {
  return dependencyDeclarations(pkg.packageJson).filter(
    (declaration) => declaration.field === 'dependencies' || declaration.field === 'optionalDependencies',
  );
}

function resolveFileDependency(
  workspaceRoot: string,
  declaringPackage: LoadedPackage,
  declaration: DependencyDeclaration,
  packagesByDirectory: ReadonlyMap<string, LoadedPackage>,
): LoadedPackage {
  const target = resolve(declaringPackage.directory, declaration.spec.slice('file:'.length));
  if (!existsSync(target)) {
    throw new Error(
      `Package '${declaringPackage.key}' declares '${declaration.name}' as '${declaration.spec}', but ${target} does not exist`,
    );
  }
  assertInsideDirectory(workspaceRoot, target, `Linked dependency '${declaration.name}'`);
  const pkg = packagesByDirectory.get(realpathSync(target));
  if (pkg === undefined) {
    throw new Error(
      `Package '${declaringPackage.key}' links '${declaration.name}' to ${target}, which is absent from npm-manifest.json`,
    );
  }
  if (pkg.packageJson.name !== declaration.name) {
    throw new Error(
      `Package '${declaringPackage.key}' declares '${declaration.name}' but '${declaration.spec}' resolves to package ` +
        `'${String(pkg.packageJson.name)}'`,
    );
  }
  return pkg;
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

export function selectTestConsumerTargets(
  targets: readonly TestConsumerTarget[],
  selector: string,
): readonly TestConsumerTarget[] {
  const normalized = normalizeSelector(selector);
  const matches = targets.filter((target) => targetSelectors(target).has(normalized));
  if (matches.length === 0) {
    throw new Error(`No test consumer matches '${selector}'. Use 'test-consumer --list' to see available consumers.`);
  }
  const groups = new Set(matches.map((target) => target.owner?.key ?? target.source.key));
  if (groups.size > 1) {
    throw new Error(
      `Test consumer selector '${selector}' is ambiguous; use a consumer path: ${matches
        .map((target) => target.source.key)
        .sort()
        .join(', ')}`,
    );
  }
  return matches.sort((left, right) => left.moduleKind.localeCompare(right.moduleKind));
}

export function resolveConsumerTestFile(testFile: string, workspaceRoot: string, fixtureDirectory: string): string {
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

function targetSelectors(target: TestConsumerTarget): ReadonlySet<string> {
  if (target.kind === 'fixture' && target.owner !== undefined && target.published !== undefined) {
    return fixtureSelectors({
      owner: target.owner,
      published: target.published,
      fixture: target.source,
      moduleKind: target.moduleKind as ModuleKind,
    });
  }
  return selectors(target.source.key, target.source.packageJson.name);
}

function selectors(...values: readonly (string | undefined)[]): ReadonlySet<string> {
  return new Set(
    values
      .filter((value): value is string => value !== undefined)
      .flatMap((value) => [normalizeSelector(value), normalizeSelector(value.replace(/^\.\//, ''))]),
  );
}

function normalizeSelector(selector: string): string {
  return selector.replace(/\/$/, '');
}

function printTargets(targets: readonly TestConsumerTarget[]): void {
  if (targets.length === 0) {
    console.log('No checked-in test-consumer fixtures or manifest-listed consumer projects are available.');
    return;
  }
  for (const target of targets) {
    const dependencies = target.linkedDependencies.map((dependency) => dependency.package.key).join(', ');
    if (target.kind === 'fixture' && target.owner !== undefined) {
      console.log(
        `${target.owner.key} (${target.owner.packageJson.name}) -> ${target.source.key} ` +
          `[fixture, ${target.moduleKind.toUpperCase()}, links: ${dependencies}]`,
      );
    } else {
      console.log(
        `${target.source.key} (${target.source.packageJson.name}) ` +
          `[project, ${target.moduleKind.toUpperCase()}, links: ${dependencies}]`,
      );
    }
  }
}

export function linkedDependencyBuildOrder(
  packages: readonly LoadedPackage[],
  dependencies: readonly LinkedDependency[],
): readonly LoadedPackage[] {
  const ownersByPayload = new Map<string, LoadedPackage>();
  for (const pkg of packages) {
    if (pkg.inventory.kind === 'dev' && pkg.inventory.publishedRelPath !== undefined) {
      ownersByPayload.set(pkg.inventory.publishedRelPath, pkg);
    }
  }

  const nodes = new Map<string, { readonly owner: LoadedPackage; readonly payload: LoadedPackage }>();
  for (const dependency of dependencies) {
    const payload = dependency.package;
    if (payload.inventory.kind !== 'published') continue;
    const owner = ownersByPayload.get(payload.key);
    if (owner === undefined) {
      throw new Error(`Linked published dependency '${payload.key}' has no dev owner in npm-manifest.json`);
    }
    const compile = owner.packageJson.scripts?.compile;
    if (compile === undefined || compile.trim() === '') {
      throw new Error(`Dev owner '${owner.key}' must define a non-empty 'compile' script`);
    }
    nodes.set(owner.key, { owner, payload });
  }

  const ownerByPackageName = new Map<string, string>();
  for (const [ownerKey, { payload }] of nodes) {
    if (payload.packageJson.name !== undefined) ownerByPackageName.set(payload.packageJson.name, ownerKey);
  }
  const dependenciesByOwner = new Map<string, readonly string[]>();
  for (const [ownerKey, { payload }] of nodes) {
    const requiredOwners = dependencyDeclarations(payload.packageJson)
      .map((declaration) => ownerByPackageName.get(declaration.name))
      .filter((candidate): candidate is string => candidate !== undefined && candidate !== ownerKey);
    dependenciesByOwner.set(ownerKey, [...new Set(requiredOwners)].sort());
  }

  const ordered: LoadedPackage[] = [];
  const visiting = new Set<string>();
  const visited = new Set<string>();
  const visit = (ownerKey: string): void => {
    if (visited.has(ownerKey)) return;
    if (visiting.has(ownerKey)) {
      throw new Error(`Linked dependency build graph contains a cycle at '${ownerKey}'`);
    }
    visiting.add(ownerKey);
    for (const dependencyOwner of dependenciesByOwner.get(ownerKey) ?? []) visit(dependencyOwner);
    visiting.delete(ownerKey);
    visited.add(ownerKey);
    const node = nodes.get(ownerKey);
    if (node !== undefined) ordered.push(node.owner);
  };
  for (const ownerKey of [...nodes.keys()].sort()) visit(ownerKey);
  return ordered;
}

export function buildLinkedDependenciesWithMake(
  workspaceRoot: string,
  owners: readonly LoadedPackage[],
  verbosity: Verbosity,
  runner: MakeBuildRunner = runMakeBuildPackage,
  log: (message: string) => void = console.log,
): void {
  log('\n🎃 Building linked dependencies once before the serial consumer runs.\n');
  for (const owner of owners) {
    log(`  - ${owner.key} (${owner.packageJson.name ?? 'unnamed'})`);
  }
  log('');
  for (const [index, owner] of owners.entries()) {
    log(
      `🚀 Building linked dependency ${String(index + 1)}/${String(owners.length)}: ${owner.key} (${owner.packageJson.name ?? 'unnamed'})`,
    );
    runner(workspaceRoot, owner.key, verbosity);
  }
}

function consumerName(target: TestConsumerTarget): string {
  return target.source.packageJson.name ?? target.source.key;
}

function assertInsideDirectory(root: string, candidate: string, label: string): void {
  const rel = relative(realpathSync(root), realpathSync(candidate));
  if (rel === '..' || rel.startsWith(`..${sep}`) || isAbsolute(rel)) {
    throw new Error(`${label} resolves outside the sdk workspace: ${candidate}`);
  }
}

function resolveDestination(
  output: string | undefined,
  ownerKey: string,
  moduleKind: ModuleKind | 'dual',
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
  sourceDirectory: string,
  linkedDependencies: readonly LinkedDependency[],
  useCommittedLock: boolean,
): void {
  const packageJsonPath = join(destination, 'package.json');
  const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8')) as {
    dependencies?: Record<string, string>;
    devDependencies?: Record<string, string>;
    optionalDependencies?: Record<string, string>;
    peerDependencies?: Record<string, string>;
  };
  const rewritten = new Map<
    string,
    {
      readonly dependency: LinkedDependency;
      readonly installDeclaration: DependencyDeclaration;
      readonly originalRootSpec?: string;
      readonly expectedLockedResolved: string;
      readonly absoluteSpec: string;
    }
  >();
  for (const dependency of linkedDependencies) {
    const { declaration, package: linkedPackage } = dependency;
    const rootDeclarations = dependencyDeclarations(packageJson).filter(
      (candidate) => candidate.name === declaration.name,
    );
    if (rootDeclarations.length > 1) {
      throw new Error(
        `${selectedManifestLabel(packageJsonPath)} declares '${declaration.name}' in multiple dependency fields`,
      );
    }
    const originalRootDeclaration = rootDeclarations[0];
    if (dependency.direct && originalRootDeclaration?.spec !== declaration.spec) {
      throw new Error(
        `${selectedManifestLabel(packageJsonPath)} changed '${declaration.name}' in '${declaration.field}' ` +
          `from '${declaration.spec}' to '${String(originalRootDeclaration?.spec)}' while staging`,
      );
    }
    const installDeclaration =
      originalRootDeclaration ?? ({ field: 'devDependencies', name: declaration.name, spec: '' } as const);
    const absoluteSpec = pathToFileURL(linkedPackage.directory).href;
    packageJson[installDeclaration.field] ??= {};
    packageJson[installDeclaration.field]![declaration.name] = absoluteSpec;
    rewritten.set(declaration.name, {
      dependency,
      installDeclaration,
      originalRootSpec: originalRootDeclaration?.spec,
      expectedLockedResolved: dependency.direct
        ? declaration.spec
        : relativeFileSpec(sourceDirectory, linkedPackage.directory),
      absoluteSpec,
    });
  }
  writeFileSync(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`);
  if (useCommittedLock) {
    patchLocalDependencyLock(destination, rewritten);
  } else {
    rmSync(join(destination, 'package-lock.json'), { force: true });
  }
}

function patchLocalDependencyLock(
  destination: string,
  rewritten: ReadonlyMap<
    string,
    {
      readonly dependency: LinkedDependency;
      readonly installDeclaration: DependencyDeclaration;
      readonly originalRootSpec?: string;
      readonly expectedLockedResolved: string;
      readonly absoluteSpec: string;
    }
  >,
): void {
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
        version?: string;
        resolved?: string;
        link?: boolean;
      }
    >;
  };
  const root = packageLock.packages?.[''];
  if (root === undefined) throw new Error(`Consumer lockfile ${packageLockPath} has no root package`);
  for (const [packageName, rewrite] of rewritten) {
    const { dependency, installDeclaration, originalRootSpec, expectedLockedResolved, absoluteSpec } = rewrite;
    const lockedDependency = packageLock.packages?.[`node_modules/${packageName}`];
    if (lockedDependency === undefined) {
      throw new Error(`Consumer lockfile ${packageLockPath} does not lock '${packageName}'`);
    }
    const lockedSpec = root[installDeclaration.field]?.[packageName];
    if (lockedSpec !== originalRootSpec) {
      throw new Error(
        `Consumer lockfile ${packageLockPath} records '${packageName}' as '${String(lockedSpec)}' in ` +
          `'${installDeclaration.field}', expected '${String(originalRootSpec)}'; regenerate the committed lockfile`,
      );
    }
    if (lockedDependency.link === true) {
      throw new Error(
        `Consumer lockfile ${packageLockPath} locks '${packageName}' as a symlink; regenerate it with --install-links`,
      );
    }
    if (lockedDependency.version !== dependency.package.packageJson.version) {
      throw new Error(
        `Consumer lockfile ${packageLockPath} locks '${packageName}' at version ` +
          `'${String(lockedDependency.version)}', expected '${String(dependency.package.packageJson.version)}'`,
      );
    }
    if (lockedDependency.resolved !== expectedLockedResolved) {
      throw new Error(
        `Consumer lockfile ${packageLockPath} resolves '${packageName}' as '${String(lockedDependency.resolved)}', ` +
          `expected '${expectedLockedResolved}'; regenerate the committed lockfile`,
      );
    }
    root[installDeclaration.field] ??= {};
    root[installDeclaration.field]![packageName] = absoluteSpec;
    lockedDependency.resolved = absoluteSpec;
  }
  writeFileSync(packageLockPath, `${JSON.stringify(packageLock, null, 2)}\n`);
}

function relativeFileSpec(fromDirectory: string, packageDirectory: string): string {
  const rel = relative(fromDirectory, packageDirectory).split(sep).join('/');
  return `file:${rel.startsWith('.') ? rel : `./${rel}`}`;
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

function runNpm(directory: string, args: readonly string[], verbosity: Verbosity = 0): void {
  const npmArgs = [...npmVerbosityArguments(verbosity), ...args];
  if (hasProgress(verbosity)) console.log(`npm ${npmArgs.join(' ')} (${directory})`);
  try {
    execFileSync('npm', npmArgs, {
      cwd: directory,
      encoding: hasDetailedOutput(verbosity) ? undefined : 'utf8',
      stdio: hasDetailedOutput(verbosity) ? 'inherit' : 'pipe',
    });
  } catch (error) {
    if (!hasDetailedOutput(verbosity) && typeof error === 'object' && error !== null) {
      const processError = error as { stdout?: string | Buffer; stderr?: string | Buffer };
      if (processError.stdout !== undefined && processError.stdout.length > 0)
        process.stdout.write(processError.stdout);
      if (processError.stderr !== undefined && processError.stderr.length > 0)
        process.stderr.write(processError.stderr);
    }
    throw error;
  }
}

function runMakeBuildPackage(workspaceRoot: string, packageKey: string, verbosity: Verbosity = 0): void {
  const makeArgs = ['--no-print-directory'];
  if (!hasDetailedOutput(verbosity)) makeArgs.push('--silent');
  makeArgs.push('compile-package', `PACKAGE=${packageKey}`);
  try {
    execFileSync('make', makeArgs, {
      cwd: workspaceRoot,
      encoding: hasDetailedOutput(verbosity) ? undefined : 'utf8',
      stdio: hasDetailedOutput(verbosity) ? 'inherit' : 'pipe',
    });
  } catch (error) {
    if (!hasDetailedOutput(verbosity) && typeof error === 'object' && error !== null) {
      const processError = error as { stdout?: string | Buffer; stderr?: string | Buffer };
      if (processError.stdout !== undefined && processError.stdout.length > 0)
        process.stdout.write(processError.stdout);
      if (processError.stderr !== undefined && processError.stderr.length > 0)
        process.stderr.write(processError.stderr);
    }
    throw error;
  }
}

function shellQuote(value: string): string {
  return `'${value.replaceAll("'", `'\\''`)}'`;
}

function workspaceRelativeDisplayPath(workspaceRoot: string, absolutePath: string): string {
  const relativePath = relative(workspaceRoot, absolutePath).split(sep).join('/');
  return relativePath.startsWith('.') ? relativePath : `./${relativePath}`;
}

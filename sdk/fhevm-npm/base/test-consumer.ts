import { execFileSync, spawnSync } from 'node:child_process';
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
import { fileURLToPath, pathToFileURL } from 'node:url';

import { type NpmManifest, registeredConsumers } from '../manifest.ts';
import { type ModuleKind, consumerModuleKinds } from './module-kind.ts';
import { declaredEntryPoints } from './publish-check.ts';
import {
  type DependencyDeclaration,
  type LoadedPackage,
  type PackageJson,
  dependencyDeclarations,
  loadPackages,
  readPackageJson,
} from './npm.ts';
import { type Verbosity, hasDetailedOutput, hasProgress, npmVerbosityArguments } from './verbosity.ts';

/**
 * One `consumerTests` entry, resolved: `published` registers the consumer for `moduleKind`, and `owner` is
 * the dev package that owns `published`. The owner is derived from the REGISTRATION, never from where the
 * consumer's directory happens to sit, so a template registered by a plugin belongs to the plugin's owner.
 */
export type TestConsumerRegistration = {
  readonly published: LoadedPackage;
  readonly owner: LoadedPackage;
  readonly moduleKind: ModuleKind;
};

export type LinkedDependency = {
  readonly declaration: DependencyDeclaration;
  readonly declaredBy: LoadedPackage;
  readonly direct: boolean;
  readonly package: LoadedPackage;
};

/**
 * A consumer suite to run: one per registered consumer package, however many payloads register it. Every
 * registration is kept so owner and payload selectors still reach a shared consumer, which runs once.
 */
export type TestConsumerTarget = {
  readonly source: LoadedPackage;
  readonly moduleKind: ModuleKind;
  readonly registrations: readonly TestConsumerRegistration[];
  readonly linkedDependencies: readonly LinkedDependency[];
};

/**
 * How a consumer is installed is a property of the CONSUMER, not a flag: a committed lockfile is replayed
 * (`npm ci`); a workspace member has none by policy (rule 6.1.1: its installation root locks it) and is
 * resolved fresh (`npm install`); an isolated consumer with no lockfile is resolved fresh too, but that is
 * a gap rather than a policy, so it warns, and `--ci` turns the warning into an error.
 */
export type ConsumerInstallMode = {
  readonly mode: 'committed' | 'fresh';
  readonly reason: 'committed lockfile' | 'workspace member' | 'no lockfile';
};

export function consumerInstallMode(
  source: Pick<LoadedPackage, 'directory' | 'inventory'>,
  fileExists: (file: string) => boolean = existsSync,
): ConsumerInstallMode {
  if (source.inventory.member) return { mode: 'fresh', reason: 'workspace member' };
  if (fileExists(join(source.directory, 'package-lock.json')))
    return { mode: 'committed', reason: 'committed lockfile' };
  return { mode: 'fresh', reason: 'no lockfile' };
}

export function describeInstallMode(install: ConsumerInstallMode): string {
  return install.mode === 'committed' ? 'committed' : `fresh (${install.reason})`;
}

export type PrepareTestConsumerOptions = {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly packageSelector?: string;
  readonly all: boolean;
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
  const targets = resolveTestConsumerTargets(options.workspaceRoot, packages);
  if (options.list) {
    printTargets(targets);
    return;
  }
  const selected = selectTargetsToRun(targets, options.packageSelector, options.all, packages);
  for (const target of selected) {
    const testScript = target.source.packageJson.scripts?.test;
    if (testScript === undefined || testScript.trim() === '') {
      throw new Error(`Consumer '${target.source.key}' must define a non-empty 'test' script`);
    }
    const install = consumerInstallMode(target.source);
    if (install.reason === 'no lockfile') {
      if (options.ci) {
        throw new Error(
          `Consumer '${target.source.key}' is not a workspace member and has no committed package-lock.json; ` +
            "'--ci' refuses to resolve it fresh",
        );
      }
      console.warn(`⚠️  Consumer '${target.source.key}' has no committed package-lock.json; resolving it fresh`);
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
    // Several suites may share a format, so a multi-consumer output fans out by SOURCE, never by format.
    const output =
      options.output === undefined || selected.length === 1
        ? options.output
        : join(options.output, destinationDirectoryName(target.source.key));
    const testFile =
      options.testFile === undefined
        ? undefined
        : resolveConsumerTestFile(options.testFile, options.workspaceRoot, target.source.directory);
    const testFileLabel =
      testFile === undefined
        ? undefined
        : workspaceRelativeDisplayPath(options.workspaceRoot, join(target.source.directory, testFile));
    return {
      target,
      testFile,
      testFileLabel,
      install: consumerInstallMode(target.source),
      ...resolveDestination(output, target.source.key, target.moduleKind),
    };
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

  for (const [index, { target, testFile, testFileLabel, install, destination, managed }] of planned.entries()) {
    const serialIndex = index + 1;
    const startedAt = Date.now();
    let succeeded = false;
    try {
      printConsumerStartBanner(target, destination, testFileLabel, install, serialIndex, planned.length, options.run);
      prepareDestination(destination, options.force, managed);
      copyFixture(target.source.directory, destination);
      writeFileSync(
        join(destination, outputMarker),
        `${JSON.stringify(
          {
            consumer: target.source.key,
            moduleKind: target.moduleKind,
            registeredBy: target.registrations.map((registration) => registration.published.key),
            linkedDependencies: target.linkedDependencies.map((dependency) => dependency.package.key),
          },
          null,
          2,
        )}\n`,
      );
      const useCommittedLock = install.mode === 'committed';
      patchLocalDependencies(destination, target.source.directory, target.linkedDependencies, useCommittedLock);
      runNpm(destination, consumerInstallArguments(useCommittedLock), options.verbosity);
      for (const dependency of target.linkedDependencies) {
        verifyPhysicalInstallation(destination, dependency.package);
        // Entry-point and declaration resolution are properties of a PUBLISHED artifact. The linked set
        // also carries workspace dev packages — a fixture links them as devDependencies, and the top
        // level deliberately reads those (a payload under test is a devDependency in some fixtures) —
        // but nothing ships them, so there is no published surface of theirs to hold to its own manifest.
        if (dependency.package.inventory.kind !== 'published') continue;
        verifyResolvedEntryPoints(destination, dependency.package, target.moduleKind, options.verbosity);
        verifyDeclarationResolution(destination, dependency.package, target.moduleKind, options.verbosity);
      }

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
          options.run,
        );
      }
    }
  }
}

function printConsumerStartBanner(
  target: TestConsumerTarget,
  destination: string,
  testFile: string | undefined,
  install: ConsumerInstallMode,
  serialIndex: number,
  serialTotal: number,
  run: boolean,
): void {
  const separator = '================================================================================';
  const testName = consumerName(target);
  // Nothing is executed without --run, so the run banner's ceremony would announce a test session that
  // never happens. An install says so in one line.
  if (!run) {
    const position = serialTotal > 1 ? `[${String(serialIndex)}/${String(serialTotal)}] ` : '';
    console.log(`\n📦 ${position}Installing consumer test ${testName}...`);
    return;
  }
  const dependencies = target.linkedDependencies
    .map((dependency) => {
      const origin = dependency.direct ? 'direct' : `transitive via ${dependency.declaredBy.packageJson.name}`;
      return `  - ${dependency.declaration.name} -> ${dependency.package.key} (${origin})`;
    })
    .join('\n');
  console.log(`
🟦${separator}
🚦 FHEVM TEST CONSUMER START — SERIAL RUN ${String(serialIndex)}/${String(serialTotal)} — ${target.moduleKind.toUpperCase()}
🧪 PACKAGE NAME: ${testName}
📋 PACKAGE SOURCE: ${target.source.key}
📂 RUNNING IN: ${destination}
🔗 LINKED DEPENDENCIES:
${dependencies === '' ? '  - none' : dependencies}
${testFile === undefined ? '' : `🎯 TEST FILE: ${testFile}\n`}🔒 INSTALL MODE: ${install.mode === 'committed' ? 'COMMITTED LOCK (npm ci)' : `FRESH LOCK (npm install; ${install.reason})`}
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
  run: boolean,
): void {
  const separator = '================================================================================';
  const color = succeeded ? '🟩' : '🟥';
  const testName = consumerName(target);
  // The install path already reports the directory and the command to run by hand, and a PASSED verdict
  // here would describe a test suite that never ran.
  if (!run) {
    if (!succeeded) console.log(`❌ Failed to install consumer test ${testName}`);
    return;
  }
  const result = succeeded ? '✅ PASSED' : '❌ FAILED';
  console.log(`
${color}${separator}
🏁 FHEVM TEST CONSUMER END — SERIAL RUN ${String(serialIndex)}/${String(serialTotal)} — ${target.moduleKind.toUpperCase()} — ${result}
🧪 PACKAGE NAME: ${testName}
📋 PACKAGE SOURCE: ${target.source.key}
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
  const targets = resolveTestConsumerTargets(options.workspaceRoot, packages);
  const selected =
    options.packageSelector === undefined
      ? targets
      : selectTestConsumerTargets(targets, options.packageSelector, packages);
  if (selected.length === 0) throw new Error('No registered test consumers are available.');

  // A workspace member is locked by its installation root and must not carry its own lockfile (rule
  // 6.1.1). An explicit request for one is a mistake to explain; a bulk run skips it and says so.
  const [members, isolated] = partition(selected, (target) => target.source.inventory.member);
  if (options.packageSelector !== undefined && members.length > 0) {
    throw new Error(
      `Consumer '${members.map((target) => target.source.key).join("', '")}' is a workspace member: its ` +
        'dependencies are locked by its installation root, and a lockfile of its own is forbidden by rule 6.1.1',
    );
  }
  for (const target of members) {
    console.log(`⏭️  Skipped ${target.source.key}: workspace member, locked by its installation root`);
  }

  for (const target of isolated) {
    regenerateFixturePackageLock(target.source.directory, runner, target.linkedDependencies, options.verbosity);
    console.log(`✅ Regenerated ${target.source.key}/package-lock.json (${target.moduleKind.toUpperCase()})`);
  }
}

function partition<T>(values: readonly T[], predicate: (value: T) => boolean): [readonly T[], readonly T[]] {
  const matching: T[] = [];
  const rest: T[] = [];
  for (const value of values) (predicate(value) ? matching : rest).push(value);
  return [matching, rest];
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

/**
 * Resolve every consumer suite from `consumerTests`, the manifest's sole registry of consumer tests.
 * Nothing is discovered: a `test-consumer/<format>` directory beside a payload registers nothing by
 * existing, and a package with a `test` script and a `file:` dependency is not a consumer test unless
 * some payload says so. Dependency traversal still happens per consumer, but it finds what to INSTALL,
 * never what to run.
 */
export function resolveTestConsumerTargets(
  workspaceRoot: string,
  packages: readonly LoadedPackage[],
): readonly TestConsumerTarget[] {
  const byKey = new Map(packages.map((pkg) => [pkg.key, pkg]));
  const ownersByPayload = new Map<string, LoadedPackage[]>();
  for (const pkg of packages) {
    if (pkg.inventory.kind !== 'dev' || pkg.inventory.publishedRelPath === undefined) continue;
    const owners = ownersByPayload.get(pkg.inventory.publishedRelPath) ?? [];
    owners.push(pkg);
    ownersByPayload.set(pkg.inventory.publishedRelPath, owners);
  }

  const registrationsBySource = new Map<string, TestConsumerRegistration[]>();
  for (const published of packages) {
    if (published.inventory.kind !== 'published') continue;
    for (const { moduleKind, consumerKey } of registeredConsumers(published.inventory)) {
      if (!byKey.has(consumerKey)) {
        throw new Error(
          `'${published.key}' registers consumer '${consumerKey}', which is absent from npm-manifest.json`,
        );
      }
      const owners = ownersByPayload.get(published.key) ?? [];
      const owner = owners[0];
      if (owner === undefined || owners.length !== 1) {
        throw new Error(
          `'${published.key}' registers consumer tests but has ${String(owners.length)} dev owners instead of one`,
        );
      }
      const registrations = registrationsBySource.get(consumerKey) ?? [];
      registrations.push({ published, owner, moduleKind });
      registrationsBySource.set(consumerKey, registrations);
    }
  }

  const targets: TestConsumerTarget[] = [];
  for (const [sourceKey, registrations] of registrationsBySource) {
    const source = byKey.get(sourceKey);
    if (source === undefined) throw new Error('unreachable');
    const moduleKinds = new Set(registrations.map((registration) => registration.moduleKind));
    const moduleKind = [...moduleKinds][0];
    if (moduleKind === undefined || moduleKinds.size !== 1) {
      throw new Error(
        `Consumer '${sourceKey}' is registered under conflicting formats (${[...moduleKinds].sort().join(', ')}); ` +
          'one consumer package executes as one format',
      );
    }
    targets.push({
      source,
      moduleKind,
      registrations: [...registrations].sort((left, right) => left.published.key.localeCompare(right.published.key)),
      linkedDependencies: resolveLinkedDependencies(workspaceRoot, source, packages),
    });
  }
  return targets.sort((left, right) => left.source.key.localeCompare(right.source.key));
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

/**
 * `--all` means every registered consumer, which the registry makes well defined: there is no longer a
 * discovered set whose stragglers a run-all would have to filter out. It is exclusive with a selector.
 */
export function selectTargetsToRun(
  targets: readonly TestConsumerTarget[],
  selector: string | undefined,
  all: boolean,
  packages: readonly LoadedPackage[] = [],
): readonly TestConsumerTarget[] {
  if (all && selector !== undefined) {
    throw new Error(`'--all' runs every registered consumer; drop the selector '${selector}' or drop '--all'`);
  }
  if (all) {
    if (targets.length === 0) throw new Error('No consumer tests are registered in npm-manifest.json#consumerTests.');
    return targets;
  }
  if (selector === undefined) {
    throw new Error("test-consumer requires a package selector or '--all'; use '--list' to see available consumers");
  }
  return selectTestConsumerTargets(targets, selector, packages);
}

/**
 * Select by consumer key or name, by a registering payload's key or name, or by that payload's owner key
 * or name. Owner and payload selectors run every suite they register, including several of one format;
 * a consumer registered by several payloads is one target and runs once. A selector spanning more than
 * one owner (a package name two payloads share, say) is ambiguous.
 */
export function selectTestConsumerTargets(
  targets: readonly TestConsumerTarget[],
  selector: string,
  packages: readonly LoadedPackage[] = [],
): readonly TestConsumerTarget[] {
  const normalized = normalizeSelector(selector);
  const matches = targets.filter((target) => targetSelectors(target).has(normalized));
  if (matches.length === 0) {
    const unregistered = packages.find((pkg) => selectors(pkg.key, pkg.packageJson.name).has(normalized));
    if (unregistered !== undefined) {
      throw new Error(
        `'${unregistered.key}' is a manifest package but no payload registers it as a consumer test; add it to ` +
          `the 'cjs' or 'esm' array of 'consumerTests' on the published package it tests, or use one of the ` +
          `registered consumers listed by 'test-consumer --list'`,
      );
    }
    throw new Error(`No test consumer matches '${selector}'. Use 'test-consumer --list' to see available consumers.`);
  }
  // Ambiguity is a property of what the selector HIT, not of everything a matched target is registered
  // by: a consumer named directly is one suite however many payloads share it, while a name that reaches
  // registrations under two different owners could mean either.
  const owners = new Set<string>();
  for (const target of matches) {
    if (selectors(target.source.key, target.source.packageJson.name).has(normalized)) continue;
    for (const registration of target.registrations) {
      if (registrationSelectors(registration).has(normalized)) owners.add(registration.owner.key);
    }
  }
  if (owners.size > 1) {
    throw new Error(
      `Test consumer selector '${selector}' is ambiguous; use an owner or consumer path: ${[...owners].sort().join(', ')}`,
    );
  }
  return matches.sort((left, right) => left.source.key.localeCompare(right.source.key));
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

function targetSelectors(target: TestConsumerTarget): ReadonlySet<string> {
  return new Set([
    ...selectors(target.source.key, target.source.packageJson.name),
    ...target.registrations.flatMap((registration) => [...registrationSelectors(registration)]),
  ]);
}

function registrationSelectors(registration: TestConsumerRegistration): ReadonlySet<string> {
  return selectors(
    registration.owner.key,
    registration.owner.packageJson.name,
    registration.published.key,
    registration.published.packageJson.name,
  );
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
    console.log('No consumer tests are registered in npm-manifest.json#consumerTests.');
    return;
  }
  for (const target of targets) {
    const payloads = target.registrations.map((registration) => registration.published.key).join(', ');
    const owners = [...new Set(target.registrations.map((registration) => registration.owner.key))].join(', ');
    const dependencies = target.linkedDependencies.map((dependency) => dependency.package.key).join(', ');
    console.log(
      `${target.source.key} [${target.moduleKind.toUpperCase()}] payload: ${payloads}; owner: ${owners}; ` +
        `lock: ${describeInstallMode(consumerInstallMode(target.source))}; links: ${dependencies}`,
    );
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
  sourceKey: string,
  moduleKind: ModuleKind,
): { readonly destination: string; readonly managed: boolean } {
  if (output !== undefined) return { destination: resolve(output), managed: false };
  prepareManagedRoot();
  // The format leaf carries no identity (the source key is already unique); it keeps the marked directory
  // one level below the source-named one, which is where every earlier layout put its marker too.
  return { destination: join(managedOutputRoot, destinationDirectoryName(sourceKey), moduleKind), managed: true };
}

/** The destination identity of a consumer is its SOURCE key, so two suites of one format never collide. */
export function destinationDirectoryName(sourceKey: string): string {
  return sourceKey.replace(/^\.\//, '').replaceAll('/', '-');
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

/**
 * Does the INSTALLED package resolve to the entry points it declares? Rule 5.3.10 proves the tarball
 * CONTAINS them; this proves a real consumer reaches them. Everything is derived from the installed
 * `package.json` — never from a package name — so every payload is checked the same way.
 */
function verifyResolvedEntryPoints(
  destination: string,
  published: LoadedPackage,
  consumerModuleKind: ModuleKind | 'dual',
  verbosity: Verbosity = 0,
): void {
  const packageName = published.packageJson.name;
  if (packageName === undefined) throw new Error(`Published package '${published.key}' has no name`);
  const installedRoot = join(destination, 'node_modules', ...packageName.split('/'));
  const installed = readPackageJson(join(installedRoot, 'package.json'));
  if (installed === undefined) throw new Error(`${packageName} has no installed package.json in ${installedRoot}`);

  if (!declaresRootEntryPoint(installed)) {
    if (hasProgress(verbosity)) {
      console.log(`   Resolution: skipped for ${packageName} (subpath-only package, no root entry point)`);
    }
    return;
  }

  for (const moduleKind of probedModuleKinds(consumerModuleKind, installed)) {
    const resolved = resolveFromConsumer(destination, packageName, moduleKind);
    const verdict = classifyResolvedEntryPoint(installedRoot, resolved, installed);
    if (!verdict.inside) {
      throw new Error(
        `${packageName} resolved for ${moduleKind.toUpperCase()} to ${resolved}, outside its installed directory ` +
          `${installedRoot} — the consumer escaped the installed package`,
      );
    }
    if (!verdict.declared) {
      throw new Error(
        `${packageName} resolved for ${moduleKind.toUpperCase()} to '${verdict.relPath}', which its own package.json ` +
          `does not declare as an entry point (declares: ${declaredEntryPoints(installed).join(', ')})`,
      );
    }
    if (hasProgress(verbosity)) {
      console.log(`   ${moduleKind.toUpperCase()} resolution: ${packageName} -> ${verdict.relPath}`);
    }
  }
}

/**
 * Can a bare `import`/`require` of this package's NAME resolve at all? A package whose `exports` map has
 * no `.` key is reachable only through subpaths (`@scope/pkg/thing`), and resolving the bare name is
 * SUPPOSED to fail, so probing it would report a defect that is really a design choice.
 */
export function declaresRootEntryPoint(installed: PackageJson): boolean {
  const exportsField = (installed as unknown as Record<string, unknown>)['exports'];
  if (exportsField === undefined || exportsField === null) {
    return declaredEntryPoints(installed).length > 0;
  }
  // `"exports": "./index.js"` and `"exports": ["./a.js"]` are both sugar for the root.
  if (typeof exportsField === 'string' || Array.isArray(exportsField)) return true;
  if (typeof exportsField !== 'object') return false;
  return Object.keys(exportsField).some((key) => key === '.' || !key.startsWith('.'));
}

/** The kinds worth probing: those the consumer itself uses AND the installed package exposes. */
export function probedModuleKinds(
  consumerModuleKind: ModuleKind | 'dual',
  installed: PackageJson,
): readonly ModuleKind[] {
  const exposed = new Set(consumerModuleKinds(installed));
  const wanted: readonly ModuleKind[] = consumerModuleKind === 'dual' ? ['cjs', 'esm'] : [consumerModuleKind];
  return wanted.filter((kind) => exposed.has(kind));
}

/** Where the resolved file sits relative to the installed package, and whether that path is declared. */
export function classifyResolvedEntryPoint(
  installedRoot: string,
  resolvedPath: string,
  installed: PackageJson,
): { readonly relPath: string; readonly inside: boolean; readonly declared: boolean } {
  // Both sides through realpath: a consumer under a symlinked temp root (macOS /var -> /private/var)
  // resolves to the real path, and comparing that against the symlinked root reads as an escape.
  const rel = relative(realpathIfPossible(installedRoot), realpathIfPossible(resolvedPath));
  const inside = rel !== '' && rel !== '..' && !rel.startsWith(`..${sep}`) && !isAbsolute(rel);
  const relPath = rel.split(sep).join('/');
  return { relPath, inside, declared: inside && declaredEntryPoints(installed).includes(relPath) };
}

/**
 * Resolve a bare specifier the way the consumer's own module system would: `require.resolve` for CJS,
 * `import.meta.resolve` for ESM, both rooted at the consumer directory so its node_modules is used.
 */
function resolveFromConsumer(destination: string, packageName: string, moduleKind: ModuleKind): string {
  const script =
    moduleKind === 'cjs'
      ? 'process.stdout.write(require.resolve(process.argv[1]))'
      : 'process.stdout.write(import.meta.resolve(process.argv[1]))';
  const args = moduleKind === 'cjs' ? ['-e', script, packageName] : ['--input-type=module', '-e', script, packageName];
  let output: string;
  try {
    output = execFileSync(process.execPath, args, { cwd: destination, encoding: 'utf8' }).trim();
  } catch (error) {
    const detail = error instanceof Error ? error.message : String(error);
    throw new Error(
      `${packageName} could not be resolved for ${moduleKind.toUpperCase()} in ${destination}: ${detail}`,
    );
  }
  return output.startsWith('file:') ? fileURLToPath(output) : output;
}

/**
 * Does TypeScript reach the INSTALLED declarations, rather than escaping to the payload's sources in the
 * workspace? `publint`/`attw` inspect the payload directory, so nothing else covers this. Skipped when
 * the consumer has no TypeScript of its own: this must never install a compiler the consumer did not ask
 * for. The forbidden directory is the payload's own workspace path, so no package is named here either.
 */
function verifyDeclarationResolution(
  destination: string,
  published: LoadedPackage,
  consumerModuleKind: ModuleKind | 'dual',
  verbosity: Verbosity = 0,
): void {
  const packageName = published.packageJson.name;
  if (packageName === undefined) throw new Error(`Published package '${published.key}' has no name`);
  if (!existsSync(join(destination, 'node_modules', 'typescript', 'package.json'))) {
    if (hasProgress(verbosity)) {
      console.log(`   Declaration resolution: skipped for ${packageName} (consumer has no TypeScript)`);
    }
    return;
  }
  const installedRoot = join(destination, 'node_modules', ...packageName.split('/'));
  const installed = readPackageJson(join(installedRoot, 'package.json'));
  if (installed === undefined) throw new Error(`${packageName} has no installed package.json in ${installedRoot}`);
  const declarations = declaredTypeEntryPoints(installed);
  if (!declaresRootEntryPoint(installed)) {
    if (hasProgress(verbosity)) {
      console.log(`   Declaration resolution: skipped for ${packageName} (subpath-only package)`);
    }
    return;
  }
  if (declarations.length === 0) {
    if (hasProgress(verbosity)) {
      console.log(`   Declaration resolution: skipped for ${packageName} (declares no types)`);
    }
    return;
  }

  for (const moduleKind of probedModuleKinds(consumerModuleKind, installed)) {
    const trace = traceDeclarationResolution(destination, packageName, moduleKind, verbosity);
    const verdict = classifyDeclarationTrace(trace, installedRoot, declarations, published.directory);
    if (verdict.escapedTo !== undefined) {
      throw new Error(
        `TypeScript resolved ${packageName} for ${moduleKind.toUpperCase()} to the workspace sources at ` +
          `${verdict.escapedTo} instead of the installed declarations — the published types do not stand alone`,
      );
    }
    if (!verdict.reachedDeclared) {
      throw new Error(
        `TypeScript did not resolve ${packageName} for ${moduleKind.toUpperCase()} to any declaration its own ` +
          `package.json declares (${declarations.join(', ')}) under ${installedRoot}`,
      );
    }
    if (hasProgress(verbosity)) {
      console.log(`   ${moduleKind.toUpperCase()} declaration resolution: ${packageName} -> installed types`);
    }
  }
}

/** `types`, `typings`, and every `types` condition inside `exports`. */
export function declaredTypeEntryPoints(installed: PackageJson): readonly string[] {
  const record = installed as unknown as Record<string, unknown>;
  const fields = [record['types'], record['typings']];
  const paths = [...fields, ...typesConditionTargets(record['exports'])]
    .filter((value): value is string => typeof value === 'string' && value.startsWith('./'))
    .map((value) => value.slice('./'.length));
  return [...new Set(paths)].sort();
}

function typesConditionTargets(value: unknown, result: string[] = []): readonly string[] {
  if (typeof value !== 'object' || value === null) return result;
  for (const [key, nested] of Object.entries(value)) {
    if (key === 'types' && typeof nested === 'string') result.push(nested);
    typesConditionTargets(nested, result);
  }
  return result;
}

/** Did the trace reach a declared declaration under the installed root, or escape to the payload's sources? */
export function classifyDeclarationTrace(
  trace: string,
  installedRoot: string,
  declarations: readonly string[],
  forbiddenSourceDirectory: string,
): { readonly reachedDeclared: boolean; readonly escapedTo?: string } {
  const normalized = trace.split(sep).join('/');
  const forbidden = `${realpathIfPossible(forbiddenSourceDirectory).split(sep).join('/')}/`;
  // Only a source file counts as an escape: the trace legitimately mentions the payload directory while
  // walking `file:` links, so the check is whether a MODULE was resolved from there.
  const escapedTo = normalized
    .split('\n')
    .map((line) => line.trim())
    .find((line) => /^File '.*' exist|^Resolved .* to /.test(line) && line.includes(forbidden) && line.includes('.ts'));
  const root = realpathIfPossible(installedRoot).split(sep).join('/');
  const reachedDeclared = declarations.some((declaration) => normalized.includes(`${root}/${declaration}`));
  return escapedTo === undefined ? { reachedDeclared } : { reachedDeclared, escapedTo };
}

function realpathIfPossible(directory: string): string {
  try {
    return realpathSync(directory);
  } catch {
    return directory;
  }
}

/** One `tsc --traceResolution` run on a throwaway file, cleaned up afterwards. */
function traceDeclarationResolution(
  destination: string,
  packageName: string,
  moduleKind: ModuleKind,
  verbosity: Verbosity,
): string {
  // The probe's EXTENSION picks the module system, not a compiler option: .cts is always CommonJS and
  // .mts always ESM, so `nodenext` honors the matching `exports` condition for each. Selecting it with
  // `moduleResolution: node10` instead would be more faithful to an old consumer, but that option is
  // deprecated from TypeScript 6 and its opt-out value differs per major, which no generic check can pin.
  const entry = join(destination, `fhevm-resolution.${moduleKind}.${moduleKind === 'cjs' ? 'cts' : 'mts'}`);
  const config = join(destination, `tsconfig.fhevm-resolution.${moduleKind}.json`);
  const compilerOptions = { module: 'nodenext', moduleResolution: 'nodenext' };
  writeFileSync(entry, `import '${packageName}';\n`);
  writeFileSync(
    config,
    `${JSON.stringify(
      {
        compilerOptions: { ...compilerOptions, noEmit: true, skipLibCheck: true, target: 'es2022', types: [] },
        files: [`./${relative(destination, entry).split(sep).join('/')}`],
      },
      null,
      2,
    )}\n`,
  );
  try {
    const result = spawnSync(
      'npm',
      ['exec', '--offline', '--', 'tsc', '--project', relative(destination, config), '--traceResolution'],
      { cwd: destination, encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 },
    );
    if (result.error !== undefined) throw result.error;
    const trace = `${result.stdout ?? ''}${result.stderr ?? ''}`;
    if (result.status !== 0) {
      const diagnostics = trace
        .split('\n')
        .filter((line) => /error TS\d+:/.test(line))
        .join('\n');
      throw new Error(
        `TypeScript failed to compile a bare import of ${packageName} in the consumer` +
          `${diagnostics.length > 0 ? `:\n${diagnostics}` : ` (exit ${String(result.status)})`}`,
      );
    }
    if (hasDetailedOutput(verbosity)) process.stdout.write(trace);
    return trace;
  } finally {
    rmSync(entry, { force: true });
    rmSync(config, { force: true });
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

import { readFileSync, readdirSync, statSync } from 'node:fs';
import { dirname, isAbsolute, join, relative, resolve, sep } from 'node:path';

import type { Violation } from '../diagnostics.ts';
import { forgeArtifactDirectories, memoizedForgeConfigReader } from '../forge-config.ts';
import { consumerModuleKinds } from '../module-kind.ts';
import { type LoadedPackage, dependencyDeclarations } from '../npm.ts';

const sharedForgeConfigReader = memoizedForgeConfigReader();

export function validateScripts(
  packages: readonly LoadedPackage[],
  isDirectory: (directory: string) => boolean = existingDirectory,
  hasSolidityFiles?: (pkg: LoadedPackage) => boolean,
  isFile: (file: string) => boolean = existingFile,
): readonly Violation[] {
  const packagesByKey = new Map(packages.map((pkg) => [pkg.key, pkg]));
  const ownersByPublishedKey = new Map<string, LoadedPackage[]>();
  for (const pkg of packages) {
    if (pkg.inventory.kind !== 'dev' || pkg.inventory.publishedRelPath === undefined) continue;
    const owners = ownersByPublishedKey.get(pkg.inventory.publishedRelPath) ?? [];
    owners.push(pkg);
    ownersByPublishedKey.set(pkg.inventory.publishedRelPath, owners);
  }

  const violations: Violation[] = [];
  const checkedRequirements = new Set<string>();
  const requireScript = (owner: LoadedPackage, script: string, rule: string, reason: string): void => {
    const requirementKey = `${owner.key}\0${script}`;
    if (checkedRequirements.has(requirementKey)) return;
    checkedRequirements.add(requirementKey);
    const command = owner.packageJson.scripts?.[script];
    if (command === undefined || command.trim() === '') {
      violations.push({
        rule,
        packageKey: owner.key,
        message: `package must define a non-empty '${script}' script for ${reason}`,
      });
    }
  };

  for (const pkg of packages) {
    if (pkg.inventory.kind !== 'published' || !isNpmDistributed(pkg)) continue;
    if (Object.keys(pkg.packageJson.scripts ?? {}).length === 0) continue;
    violations.push({
      rule: '2.1.2',
      packageKey: pkg.key,
      message: "npm-distributed published package must not contain 'scripts'; scripts belong on its dev owner",
    });
  }

  for (const owner of packages.filter((pkg) => pkg.inventory.kind === 'dev')) {
    requireScript(owner, 'compile', 'package-scripts', `published payload '${owner.inventory.publishedRelPath}'`);
    requireScript(owner, 'clean', 'package-scripts', `published payload '${owner.inventory.publishedRelPath}'`);
    requireScript(owner, 'lint', 'package-scripts', `published payload '${owner.inventory.publishedRelPath}'`);
    const payload =
      owner.inventory.publishedRelPath === undefined ? undefined : packagesByKey.get(owner.inventory.publishedRelPath);
    if (payload !== undefined && isNpmDistributed(payload)) {
      requireScript(owner, 'pack:tarball', '2.1.2', `npm-distributed payload '${payload.key}'`);
    }
    requireScript(
      owner,
      'prettier:check',
      'package-scripts',
      `published payload '${owner.inventory.publishedRelPath}'`,
    );
    requireScript(
      owner,
      'prettier:write',
      'package-scripts',
      `published payload '${owner.inventory.publishedRelPath}'`,
    );
    if (payload !== undefined && isNpmDistributed(payload)) {
      requireScript(owner, 'check', '5.2.1', `npm-distributed payload '${payload.key}'`);
      requireScript(owner, 'check:publint', '5.2.1', `npm-distributed payload '${payload.key}'`);
      requireScript(owner, 'test:consumer', '5.3.1', `npm-distributed payload '${payload.key}'`);
    }
  }

  for (const pkg of packages.filter((candidate) =>
    ['dev', 'shared-helper', 'internal-consumer'].includes(candidate.inventory.kind),
  )) {
    requireScript(pkg, 'fmt', '5.1.4', 'private workspace hygiene');
    requireScript(pkg, 'fmt:check', '5.1.4', 'private workspace hygiene');
    requireScript(pkg, 'lint', '5.1.4', 'private workspace hygiene');
    requireScript(pkg, 'prettier:check', '5.1.4', 'private workspace hygiene');
    requireScript(pkg, 'prettier:write', '5.1.4', 'private workspace hygiene');
  }

  // A package that generates must expose the full round trip: `clean:generated` deletes and `generate`
  // rewrites, which is the only way to catch a generator that has silently stopped emitting a file:
  // nothing looks dirty, so a regenerate-and-diff gate passes. Derived from the package's own
  // 'generate:*' scripts rather than listed, and checked in both directions — leaves without the
  // aggregates cannot round-trip, and aggregates without leaves are dead wiring.
  for (const pkg of packages) {
    if (hasGenerateLeaf(pkg)) {
      requireScript(pkg, 'generate', 'package-scripts', "the 'generate:*' scripts it defines");
      requireScript(pkg, 'clean:generated', 'package-scripts', "the 'generate:*' scripts it defines");
      continue;
    }
    for (const script of ['generate', 'clean:generated'] as const) {
      if (pkg.packageJson.scripts?.[script] === undefined) continue;
      violations.push({
        rule: 'package-scripts',
        packageKey: pkg.key,
        message: `'${script}' exists but the package defines no 'generate:*' script; an aggregate over nothing is dead wiring`,
      });
    }
  }

  // Verb wiring, so the aggregates the orchestrator calls cannot silently rot. Name-wiring only —
  // reachability is computed over same-package `npm run` references, never over behavior.
  for (const pkg of packages) {
    const scripts = pkg.packageJson.scripts ?? {};

    // Every generator must be reachable from `generate`, or the regeneration gate never runs it.
    // Exempt: generate:genesis (stateful anvil deploy) and generate:patch-sites (committed baseline a
    // test compares against; auto-regenerating it would make that test unfalsifiable).
    if (scripts.generate !== undefined) {
      const reachable = reachableScriptNames(scripts, 'generate');
      for (const script of Object.keys(scripts)) {
        if (!script.startsWith('generate:') || GENERATE_GATE_EXEMPT.has(script)) continue;
        if (reachable.has(script)) continue;
        violations.push({
          rule: '5.1.4b',
          packageKey: pkg.key,
          message: `'${script}' is not reachable from 'generate'; the regeneration gate would never run it`,
        });
      }
    }

    // Every deliverable validation must be reachable from `check`. Exempt: check:mirror (clones
    // upstream — a network dependency `check` must not acquire) and generator `--check` conveniences
    // (subsumed by the regeneration gate).
    if (scripts.check !== undefined) {
      const reachable = reachableScriptNames(scripts, 'check');
      for (const [script, command] of Object.entries(scripts)) {
        if (!script.startsWith('check:') || script === 'check:mirror') continue;
        if (/(^|\s)--check(\s|$)/.test(command)) continue;
        if (reachable.has(script)) continue;
        violations.push({
          rule: '5.2.1',
          packageKey: pkg.key,
          message: `'${script}' is not reachable from 'check'`,
        });
      }
    }

    // `build` is the optional everyday sweep, and its meaning is fixed: formatting gate, lint, then
    // compile, in lifecycle order. Only in-contract source-owning kinds — a mirror payload's `build`
    // is upstream's business, and the workspace root delegates to make.
    if (['dev', 'shared-helper', 'internal-consumer'].includes(pkg.inventory.kind) && scripts.build !== undefined) {
      const reachable = reachableScriptNames(scripts, 'build');
      for (const member of ['fmt:check', 'lint', 'compile'] as const) {
        if (reachable.has(member)) continue;
        violations.push({
          rule: 'package-scripts',
          packageKey: pkg.key,
          message: `'build' is the fmt:check → lint → compile sweep; it does not reach '${member}'`,
        });
      }
    }

    // Deletion coverage, enforced only where machine-readable: every output the export manifest
    // declares must be deleted by `clean:generated`; generator outputs living in TS code rely on the
    // gate itself.
    const cleanGenerated = scripts['clean:generated'];
    if (cleanGenerated !== undefined) {
      const expanded = expandNpmRunReferences(scripts, cleanGenerated, new Set(['clean:generated']));
      for (const output of readExportManifestOutputs(pkg)) {
        if (cleanRemoves(expanded, output.replace(/^\.\//, ''))) continue;
        violations.push({
          rule: '5.1.4b',
          packageKey: pkg.key,
          message: `'clean:generated' does not delete export-manifest output '${output}'`,
        });
      }
    }
  }

  // Every source-owning private package must be able to delete its own build output. Published payloads
  // carry no scripts at all (2.1.2) and a mirror's scripts are upstream's, so both are out of scope.
  //
  // The content requirements are DERIVED from what the package actually does, never listed: a package
  // that runs tsc leaves a `*.tsbuildinfo` that makes the next typecheck resume from stale state, and a
  // Forge project leaves the directories `forge config --json` reports. Those directory names are asked
  // for, never assumed — `hardhat/v2/e2e` uses `cache-forge`, not `cache`, so any guess is already wrong
  // for one project in three.
  for (const pkg of packages.filter((candidate) =>
    ['dev', 'shared-helper', 'internal-consumer'].includes(candidate.inventory.kind),
  )) {
    if (isMirrorOnlyOwner(pkg, packages)) continue;
    requireScript(pkg, 'clean', 'package-scripts', 'removing its own build output');

    const cleanScript = pkg.packageJson.scripts?.clean;
    if (cleanScript === undefined || cleanScript.trim() === '') continue;
    const clean = expandNpmRunReferences(pkg.packageJson.scripts ?? {}, cleanScript, new Set(['clean']));

    if (runsTypeScriptCompiler(pkg) && !clean.includes('.tsbuildinfo')) {
      violations.push({
        rule: 'package-scripts',
        packageKey: pkg.key,
        message:
          "'clean' must delete '*.tsbuildinfo'; the package runs tsc, and a surviving build-info file " +
          'lets the next typecheck resume from stale state',
      });
    }

    for (const directory of forgeArtifactDirectoriesOf(pkg)) {
      if (cleanRemoves(clean, directory)) continue;
      violations.push({
        rule: 'package-scripts',
        packageKey: pkg.key,
        message: `'clean' must delete the Forge directory '${directory}' reported by 'forge config --json'`,
      });
    }
  }

  for (const pkg of packages) {
    if (isMirrorOnly(pkg)) continue;
    for (const script of ['prettier:check', 'prettier:write'] as const) {
      const command = pkg.packageJson.scripts?.[script];
      if (command === undefined || !prettierTargetsSolidity(command)) continue;
      violations.push({
        rule: '5.1.5',
        packageKey: pkg.key,
        message: `'${script}' must not target Solidity; use 'forge:fmt${script === 'prettier:check' ? ':check' : ''}'`,
      });
    }
  }

  const ownsSolidity = hasSolidityFiles ?? ((pkg: LoadedPackage) => containsOwnedSolidityFiles(pkg, packages));
  for (const pkg of packages) {
    if (
      pkg.inventory.kind === 'workspace-root' ||
      pkg.inventory.kind === 'non-package' ||
      isMirrorOnly(pkg) ||
      !ownsSolidity(pkg)
    )
      continue;
    let owner = pkg;
    if (pkg.inventory.kind === 'published') {
      const owners = ownersByPublishedKey.get(pkg.key) ?? [];
      if (owners.length !== 1 || owners[0] === undefined) continue;
      owner = owners[0];
    }
    requireScript(owner, 'forge:fmt', 'package-scripts', `package '${pkg.key}' containing Solidity`);
    requireScript(owner, 'forge:fmt:check', 'package-scripts', `package '${pkg.key}' containing Solidity`);
    requireScript(owner, 'forge:lint', 'package-scripts', `package '${pkg.key}' containing Solidity`);
  }

  for (const published of packages.filter((pkg) => pkg.inventory.kind === 'published' && isNpmDistributed(pkg))) {
    for (const moduleKind of consumerModuleKinds(published.packageJson)) {
      const configuredConsumerKey = published.inventory.consumerTests?.[moduleKind];
      if (configuredConsumerKey !== undefined) {
        const configuredConsumer = packagesByKey.get(configuredConsumerKey);
        if (configuredConsumer === undefined) {
          violations.push({
            rule: '5.3.1',
            packageKey: published.key,
            message: `configured ${moduleKind.toUpperCase()} consumer '${configuredConsumerKey}' is not registered in npm-manifest.json`,
          });
          continue;
        }
        validateConfiguredConsumer(configuredConsumer, published, moduleKind, violations, isFile);
        continue;
      }

      const testConsumerDirectory = join(dirname(published.directory), 'test-consumer', moduleKind);
      const testConsumerKey = `${published.key.slice(0, -'/pkg'.length)}/test-consumer/${moduleKind}`;
      if (!isDirectory(testConsumerDirectory)) {
        violations.push({
          rule: '5.3.1',
          packageKey: published.key,
          message: `published package exposes ${moduleKind.toUpperCase()} but has no sibling '${testConsumerKey}' directory`,
        });
        continue;
      }

      const fixture = packagesByKey.get(testConsumerKey);
      if (fixture === undefined) {
        violations.push({
          rule: '5.3.1',
          packageKey: published.key,
          message: `consumer fixture '${testConsumerKey}' exists but is not registered in npm-manifest.json`,
        });
        continue;
      }
      validateTestConsumerFixture(fixture, moduleKind, violations);
    }
  }

  for (const pkg of packages) {
    // 'test:mirror' is deliberately NOT required: the mirror spec is unimplemented and the real check
    // reports false violations, so demanding an unpassable script would be enforcement theater.
    const capabilityScripts: Array<readonly [boolean, string]> = [
      [pkg.inventory.vendored !== undefined, 'check:vendored-origin'],
    ];
    for (const [declared, script] of capabilityScripts) {
      if (!declared) continue;
      let owner = pkg;
      if (pkg.inventory.kind === 'published') {
        const owners = ownersByPublishedKey.get(pkg.key) ?? [];
        if (owners.length !== 1) {
          violations.push({
            rule: '5.1.3',
            packageKey: pkg.key,
            message: `cannot require '${script}': published package has ${owners.length} dev owners instead of one`,
          });
          continue;
        }
        const resolvedOwner = owners[0];
        if (resolvedOwner === undefined) continue;
        owner = resolvedOwner;
      }
      requireScript(owner, script, '5.1.3', `the '${pkg.key}' capability`);
    }
  }

  for (const pkg of packages) {
    if (pkg.inventory.kind === 'dev') continue;
    for (const script of ['check:publint', 'test:consumer'] as const) {
      if (pkg.packageJson.scripts?.[script] !== undefined) {
        violations.push({
          rule: '5.3.2',
          packageKey: pkg.key,
          message: `'${script}' belongs on a dev owner, not kind '${pkg.inventory.kind}'`,
        });
      }
    }
  }

  return violations;
}

function validateConfiguredConsumer(
  consumer: LoadedPackage,
  published: LoadedPackage,
  moduleKind: 'cjs' | 'esm',
  violations: Violation[],
  isFile: (file: string) => boolean,
): void {
  if (!consumerModuleKinds(consumer.packageJson).includes(moduleKind)) {
    violations.push({
      rule: '5.3.1',
      packageKey: published.key,
      message: `configured consumer '${consumer.key}' does not execute as ${moduleKind.toUpperCase()}`,
    });
  }

  validateRunnableConsumer(consumer, violations);

  const packageName = published.packageJson.name;
  const linksCandidate =
    packageName !== undefined &&
    dependencyDeclarations(consumer.packageJson).some(
      (declaration) =>
        declaration.name === packageName &&
        declaration.spec.startsWith('file:') &&
        !declaration.spec.endsWith('.tgz') &&
        resolve(consumer.directory, declaration.spec.slice('file:'.length)) === resolve(published.directory),
    );
  if (!linksCandidate) {
    violations.push({
      rule: '5.3.1',
      packageKey: published.key,
      message: `configured consumer '${consumer.key}' must directly link '${packageName ?? published.key}' to '${published.key}' with a directory 'file:' dependency`,
    });
  }

  // Only an ISOLATED consumer needs its own lock (test-consumer --ci replays it). A workspace-member
  // consumer is installed by its installation root; its isolated rehearsal belongs to the publish layer.
  if (!consumer.inventory.member && !isFile(join(consumer.directory, 'package-lock.json'))) {
    violations.push({
      rule: '5.3.1',
      packageKey: published.key,
      message: `configured consumer '${consumer.key}' must contain a committed package-lock.json for --ci`,
    });
  }
}

function isNpmDistributed(pkg: LoadedPackage): boolean {
  return (pkg.inventory.distribution ?? ['npm']).includes('npm');
}

function hasGenerateLeaf(pkg: LoadedPackage): boolean {
  return Object.keys(pkg.packageJson.scripts ?? {}).some((script) => script.startsWith('generate:'));
}

const GENERATE_GATE_EXEMPT = new Set(['generate:genesis', 'generate:patch-sites']);

/** Every same-package script name a verb reaches through `npm run` references, the verb included. */
function reachableScriptNames(scripts: Readonly<Record<string, string>>, verb: string): ReadonlySet<string> {
  const seen = new Set<string>([verb]);
  const command = scripts[verb];
  if (command !== undefined) expandNpmRunReferences(scripts, command, seen);
  return seen;
}

/** Every output path the package's export manifest declares; empty when there is no manifest. The
 * manifest's own validity is generate-exports' concern, so unreadable content reports nothing here. */
function readExportManifestOutputs(pkg: LoadedPackage): readonly string[] {
  const contents = existingTextFile(join(pkg.directory, 'export.manifest.json'));
  if (contents === undefined) return [];
  try {
    const outputs = (JSON.parse(contents) as { outputs?: unknown }).outputs;
    return collectStrings(outputs);
  } catch {
    return [];
  }
}

function collectStrings(value: unknown): readonly string[] {
  if (typeof value === 'string') return [value];
  if (typeof value !== 'object' || value === null) return [];
  return Object.values(value).flatMap(collectStrings);
}

function runsTypeScriptCompiler(pkg: LoadedPackage): boolean {
  return Object.values(pkg.packageJson.scripts ?? {}).some((command) => /(^|[\s&|;])tsc(\s|$)/.test(command));
}

/**
 * Asked of forge, never guessed. A package with no `foundry.toml` is not a Forge project and forge is
 * not run for it; if forge is missing or the call fails, this reports nothing rather than inventing a
 * violation — `check-foundry` already owns "is forge installed and the right version".
 */
function forgeArtifactDirectoriesOf(pkg: LoadedPackage): readonly string[] {
  if (!existingFile(join(pkg.directory, 'foundry.toml'))) return [];
  try {
    return forgeArtifactDirectories(sharedForgeConfigReader(pkg.directory));
  } catch {
    return [];
  }
}

/** A script as its subprocesses see it: same-package `npm run <name>` clauses replaced by the named
 * script's body, so a `clean` split into `clean:*` sub-scripts still states everything it deletes.
 * Flags after `npm run` are skipped, and a clause routed elsewhere (`--prefix`, `-w`, `--workspace`)
 * is left alone — its name resolves in another package's namespace. */
export function expandNpmRunReferences(
  scripts: Readonly<Record<string, string>>,
  command: string,
  seen: Set<string>,
): string {
  return command
    .split(/(&&|\|\||;)/)
    .map((segment) => expandNpmRunSegment(scripts, segment, seen))
    .join('');
}

const CROSS_PACKAGE_FLAGS_RE = /^(--prefix|-w|--workspace|--workspaces)(=|$)/;

function expandNpmRunSegment(scripts: Readonly<Record<string, string>>, segment: string, seen: Set<string>): string {
  const tokens = segment.trim().split(/\s+/);
  const runIndex = tokens.findIndex((token, index) => token === 'run' && tokens[index - 1] === 'npm');
  if (runIndex === -1) return segment;
  if (tokens.some((token) => CROSS_PACKAGE_FLAGS_RE.test(token))) return segment;

  let nameIndex = runIndex + 1;
  while (nameIndex < tokens.length && (tokens[nameIndex]?.startsWith('-') ?? false)) nameIndex += 1;
  const name = tokens[nameIndex];
  if (name === undefined) return segment;
  const target = scripts[name];
  if (target === undefined || seen.has(name)) return segment;
  seen.add(name);

  const before = tokens.slice(0, runIndex - 1);
  const after = tokens.slice(nameIndex + 1);
  return ` ${[...before, expandNpmRunReferences(scripts, target, seen), ...after].join(' ')} `;
}

/** `rm -rf out cache` and `rimraf ./out` both count; a substring match would also accept `outdated`. */
function cleanRemoves(command: string, directory: string): boolean {
  return new RegExp(String.raw`(^|[\s'"])\.?/?${escapeForRegExp(directory)}(/|[\s'"]|$)`).test(command);
}

function escapeForRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function isMirrorOnly(pkg: LoadedPackage): boolean {
  const distribution = pkg.inventory.distribution ?? ['npm'];
  return distribution.length === 1 && distribution[0] === 'mirror';
}

function isMirrorOnlyOwner(pkg: LoadedPackage, packages: readonly LoadedPackage[]): boolean {
  if (pkg.inventory.kind !== 'dev' || pkg.inventory.publishedRelPath === undefined) return false;
  const payload = packages.find((candidate) => candidate.key === pkg.inventory.publishedRelPath);
  return payload !== undefined && isMirrorOnly(payload);
}

export function validatePrettierConfigs(
  workspaceRoot: string,
  packages: readonly LoadedPackage[],
  readTextFile: (file: string) => string | undefined = existingTextFile,
  listFiles: (directory: string) => readonly string[] = directoryFileNames,
  listDirectories: (directory: string) => readonly string[] = directoryDirectoryNames,
): readonly Violation[] {
  const violations: Violation[] = [];

  for (const pkg of packages) {
    const scripts = pkg.packageJson.scripts ?? {};
    const usesPrettier = scripts['prettier:check'] !== undefined || scripts['prettier:write'] !== undefined;
    const files = listFiles(pkg.directory);
    const configFiles = files.filter(isPrettierConfigFileName).sort();
    const expectedConfigName = pkg.inventory.kind === 'workspace-root' ? 'prettier.base.mjs' : 'prettier.config.js';

    if (isPayloadOnlyDevOwner(pkg, files, listDirectories(pkg.directory))) {
      for (const configFile of configFiles) {
        violations.push({
          rule: '5.1.6',
          packageKey: pkg.key,
          message: `source-empty dev owner must not contain Prettier configuration file '${configFile}'`,
        });
      }
      continue;
    }

    for (const configFile of configFiles) {
      if (configFile === expectedConfigName) continue;
      violations.push({
        rule: '5.1.6',
        packageKey: pkg.key,
        message: `Prettier configuration file '${configFile}' is forbidden; use only '${expectedConfigName}'`,
      });
    }

    if (pkg.inventory.kind === 'workspace-root') {
      // Only the sdk root carries the shared base config; a cluster root is a pure installation root —
      // its members reference the sdk root's base by relative path, so a second base would be a fork.
      if (pkg.key === '.' && !configFiles.includes('prettier.base.mjs')) {
        violations.push({
          rule: '5.1.6',
          packageKey: pkg.key,
          message: "workspace root must contain 'prettier.base.mjs'",
        });
      }
      if (pkg.key !== '.') {
        for (const configFile of configFiles) {
          violations.push({
            rule: '5.1.6',
            packageKey: pkg.key,
            message: `an installation root must not carry Prettier configuration '${configFile}'`,
          });
        }
      }
      continue;
    }

    if (!usesPrettier || pkg.inventory.kind === 'published' || isMirrorOnlyOwner(pkg, packages)) continue;

    const expectedImport = relative(pkg.directory, join(workspaceRoot, 'prettier.base.mjs')).split(sep).join('/');
    const normalizedImport = expectedImport.startsWith('.') ? expectedImport : `./${expectedImport}`;
    const configFile = join(pkg.directory, 'prettier.config.js');
    const contents = readTextFile(configFile);

    if (contents === undefined) {
      violations.push({
        rule: '5.1.6',
        packageKey: pkg.key,
        message: `package with Prettier scripts must contain 'prettier.config.js' referencing '${normalizedImport}'`,
      });
      continue;
    }

    if (!isPrettierBaseReference(contents, normalizedImport, pkg.packageJson.type === 'commonjs')) {
      const expectedStatement =
        pkg.packageJson.type === 'commonjs'
          ? `module.exports = import('${normalizedImport}').then((module) => module.default);`
          : `export { default } from '${normalizedImport}';`;
      violations.push({
        rule: '5.1.6',
        packageKey: pkg.key,
        message: `'prettier.config.js' must contain: ${expectedStatement}`,
      });
    }
  }

  return violations;
}

export function validateEslintConfigs(
  packages: readonly LoadedPackage[],
  listFiles: (directory: string) => readonly string[] = directoryFileNames,
  listDirectories: (directory: string) => readonly string[] = directoryDirectoryNames,
): readonly Violation[] {
  const violations: Violation[] = [];

  for (const pkg of packages) {
    const files = listFiles(pkg.directory);
    const configFiles = files.filter(isEslintConfigFileName).sort();
    const expectedConfigName = pkg.inventory.kind === 'workspace-root' ? 'eslint.base.mjs' : 'eslint.config.js';

    if (isPayloadOnlyDevOwner(pkg, files, listDirectories(pkg.directory))) {
      for (const configFile of configFiles) {
        violations.push({
          rule: '5.1.7',
          packageKey: pkg.key,
          message: `source-empty dev owner must not contain ESLint configuration file '${configFile}'`,
        });
      }
      continue;
    }

    for (const configFile of configFiles) {
      if (configFile === expectedConfigName) continue;
      violations.push({
        rule: '5.1.7',
        packageKey: pkg.key,
        message: `ESLint configuration file '${configFile}' is forbidden; use only '${expectedConfigName}'`,
      });
    }

    if (pkg.inventory.kind === 'workspace-root') {
      // Same asymmetry as 5.1.6: base configs live at the sdk root only.
      if (pkg.key === '.' && !configFiles.includes('eslint.base.mjs')) {
        violations.push({
          rule: '5.1.7',
          packageKey: pkg.key,
          message: "workspace root must contain 'eslint.base.mjs'",
        });
      }
      if (pkg.key !== '.') {
        for (const configFile of configFiles) {
          violations.push({
            rule: '5.1.7',
            packageKey: pkg.key,
            message: `an installation root must not carry ESLint configuration '${configFile}'`,
          });
        }
      }
      continue;
    }

    if (
      pkg.inventory.kind !== 'published' &&
      !isMirrorOnlyOwner(pkg, packages) &&
      pkg.packageJson.scripts?.lint !== undefined &&
      !configFiles.includes('eslint.config.js')
    ) {
      violations.push({
        rule: '5.1.7',
        packageKey: pkg.key,
        message: "package with a 'lint' script must contain the exact file 'eslint.config.js'",
      });
    }
  }

  return violations;
}

function validateTestConsumerFixture(fixture: LoadedPackage, moduleKind: 'cjs' | 'esm', violations: Violation[]): void {
  if (fixture.inventory.kind !== 'standalone' || fixture.inventory.member !== false) {
    violations.push({
      rule: '5.3.1',
      packageKey: fixture.key,
      message: `consumer fixture must be kind 'standalone' with member=false`,
    });
  }

  if (fixture.packageJson.private !== true) {
    violations.push({
      rule: '5.3.1',
      packageKey: fixture.key,
      message: `consumer fixture must set private=true`,
    });
  }

  validateRunnableConsumer(fixture, violations);
}

function validateRunnableConsumer(fixture: LoadedPackage, violations: Violation[]): void {
  const testScript = fixture.packageJson.scripts?.test;
  if (testScript === undefined || testScript.trim() === '') {
    violations.push({
      rule: '5.3.1',
      packageKey: fixture.key,
      message: `consumer fixture must define a non-empty 'test' script`,
    });
  } else if (invokesNodeTest(testScript) && !/(?:^|\s)--test-concurrency(?:=|\s+)1(?:\s|$)/.test(testScript)) {
    violations.push({
      rule: '5.3.9',
      packageKey: fixture.key,
      message: "test-consumer parallelism is forbidden; 'node --test' must set '--test-concurrency=1'",
    });
  }
}

function invokesNodeTest(command: string): boolean {
  return /(?:^|\s)node(?:\s+[^&|;\n]+)*\s--test(?:\s|$)/.test(command);
}

function existingDirectory(directory: string): boolean {
  try {
    return statSync(directory).isDirectory();
  } catch {
    return false;
  }
}

function existingFile(file: string): boolean {
  try {
    return statSync(file).isFile();
  } catch {
    return false;
  }
}

function existingTextFile(file: string): string | undefined {
  try {
    return readFileSync(file, 'utf8');
  } catch {
    return undefined;
  }
}

function directoryFileNames(directory: string): readonly string[] {
  try {
    return readdirSync(directory, { withFileTypes: true })
      .filter((entry) => entry.isFile())
      .map((entry) => entry.name);
  } catch {
    return [];
  }
}

function directoryDirectoryNames(directory: string): readonly string[] {
  try {
    return readdirSync(directory, { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => entry.name);
  } catch {
    return [];
  }
}

function isPayloadOnlyDevOwner(pkg: LoadedPackage, files: readonly string[], directories: readonly string[]): boolean {
  if (pkg.inventory.kind !== 'dev' || pkg.inventory.publishedRelPath !== `${pkg.key}/pkg`) return false;
  const nonConfigFiles = files.filter(
    (fileName) => !isPrettierConfigFileName(fileName) && !isEslintConfigFileName(fileName),
  );
  return (
    nonConfigFiles.length === 1 &&
    nonConfigFiles[0] === 'package.json' &&
    directories.length === 1 &&
    directories[0] === 'pkg'
  );
}

function isEslintConfigFileName(fileName: string): boolean {
  return (
    fileName === 'eslint.base.mjs' || /^eslint\.config\..+$/.test(fileName) || /^\.eslintrc(?:\..+)?$/.test(fileName)
  );
}

function isPrettierConfigFileName(fileName: string): boolean {
  return (
    fileName === 'prettier.base.mjs' ||
    /^prettier\.config\..+$/.test(fileName) ||
    /^\.prettierrc(?:\..+)?$/.test(fileName)
  );
}

function isPrettierBaseReference(contents: string, expectedImport: string, commonjs: boolean): boolean {
  const escapedImport = expectedImport.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  if (commonjs) {
    return new RegExp(
      `^\\s*module\\.exports\\s*=\\s*import\\(\\s*(['"])${escapedImport}\\1\\s*\\)\\.then\\(\\s*\\(module\\)\\s*=>\\s*module\\.default\\s*\\)\\s*;?\\s*$`,
    ).test(contents);
  }
  return new RegExp(`^\\s*export\\s*\\{\\s*default\\s*\\}\\s*from\\s*(['"])${escapedImport}\\1\\s*;?\\s*$`).test(
    contents,
  );
}

export function prettierTargetsSolidity(command: string): boolean {
  if (/\.sol(?:\b|$)/i.test(command)) return true;
  return [...command.matchAll(/\{([^{}]+)\}/g)].some((match) =>
    (match[1] ?? '')
      .split(',')
      .map((extension) => extension.trim().toLowerCase())
      .includes('sol'),
  );
}

function containsOwnedSolidityFiles(pkg: LoadedPackage, packages: readonly LoadedPackage[]): boolean {
  const excludedDirectories = packages
    .filter((candidate) => candidate.key !== pkg.key && isDescendantDirectory(pkg.directory, candidate.directory))
    .map((candidate) => candidate.directory);
  return containsSolidityFiles(pkg.directory, excludedDirectories);
}

function containsSolidityFiles(directory: string, excludedDirectories: readonly string[]): boolean {
  try {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const child = join(directory, entry.name);
      if (excludedDirectories.includes(child)) continue;
      if (entry.isFile() && entry.name.endsWith('.sol')) return true;
      if (entry.isDirectory() && entry.name !== 'node_modules' && containsSolidityFiles(child, excludedDirectories)) {
        return true;
      }
    }
    return false;
  } catch {
    return false;
  }
}

function isDescendantDirectory(parent: string, candidate: string): boolean {
  const rel = relative(parent, candidate);
  return rel !== '' && rel !== '..' && !rel.startsWith(`..${sep}`) && !isAbsolute(rel);
}

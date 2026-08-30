import { readFileSync, readdirSync, statSync } from 'node:fs';
import { dirname, isAbsolute, join, relative, sep } from 'node:path';

import type { Violation } from '../diagnostics.ts';
import { consumerModuleKinds } from '../module-kind.ts';
import type { LoadedPackage } from '../npm.ts';

export function validateScripts(
  packages: readonly LoadedPackage[],
  isDirectory: (directory: string) => boolean = existingDirectory,
  hasSolidityFiles?: (pkg: LoadedPackage) => boolean,
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

  for (const owner of packages.filter((pkg) => pkg.inventory.kind === 'dev')) {
    requireScript(owner, 'build', 'package-scripts', `published payload '${owner.inventory.publishedRelPath}'`);
    requireScript(owner, 'clean', 'package-scripts', `published payload '${owner.inventory.publishedRelPath}'`);
    requireScript(owner, 'lint', 'package-scripts', `published payload '${owner.inventory.publishedRelPath}'`);
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
    requireScript(owner, 'test:publint', '5.2.1', `published payload '${owner.inventory.publishedRelPath}'`);
    requireScript(owner, 'test:consumer', '5.3.1', `published payload '${owner.inventory.publishedRelPath}'`);
  }

  for (const pkg of packages.filter((candidate) =>
    ['dev', 'shared-helper', 'internal-consumer'].includes(candidate.inventory.kind),
  )) {
    requireScript(pkg, 'lint', '5.1.4', 'private workspace hygiene');
    requireScript(pkg, 'prettier:check', '5.1.4', 'private workspace hygiene');
    requireScript(pkg, 'prettier:write', '5.1.4', 'private workspace hygiene');
  }

  for (const pkg of packages) {
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
    if (pkg.inventory.kind === 'non-package' || !ownsSolidity(pkg)) continue;
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

  for (const published of packages.filter((pkg) => pkg.inventory.kind === 'published')) {
    for (const moduleKind of consumerModuleKinds(published.packageJson)) {
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
    const capabilityScripts: Array<readonly [boolean, string]> = [
      [pkg.inventory.vendored !== undefined, 'test:vendored'],
      [pkg.inventory.mirror !== undefined, 'test:mirror'],
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
    for (const script of ['test:publint', 'test:consumer'] as const) {
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

export function validatePrettierConfigs(
  workspaceRoot: string,
  packages: readonly LoadedPackage[],
  readTextFile: (file: string) => string | undefined = existingTextFile,
): readonly Violation[] {
  const violations: Violation[] = [];

  for (const pkg of packages) {
    if (pkg.inventory.kind === 'published' || pkg.inventory.kind === 'workspace-root') continue;
    const scripts = pkg.packageJson.scripts ?? {};
    if (scripts['prettier:check'] === undefined && scripts['prettier:write'] === undefined) continue;

    const configFile = join(pkg.directory, '.prettierrc.mjs');
    const expectedImport = relative(pkg.directory, join(workspaceRoot, 'prettier.base.mjs')).split(sep).join('/');
    const normalizedImport = expectedImport.startsWith('.') ? expectedImport : `./${expectedImport}`;
    const contents = readTextFile(configFile);

    if (contents === undefined) {
      violations.push({
        rule: '5.1.6',
        packageKey: pkg.key,
        message: `package with Prettier scripts must contain '.prettierrc.mjs' re-exporting '${normalizedImport}'`,
      });
      continue;
    }

    if (!isPrettierBaseReExport(contents, normalizedImport)) {
      violations.push({
        rule: '5.1.6',
        packageKey: pkg.key,
        message: `'.prettierrc.mjs' must contain: export { default } from '${normalizedImport}';`,
      });
    }
  }

  return violations;
}

/**
 * The one ESLint config filename a package may carry, decided by its own module type.
 *
 * A `.js` config is parsed with the package's `type`, so a `"type": "commonjs"` package cannot write
 * `import` — and the shared base is `eslint.base.mjs`. Those packages get `eslint.config.mjs`, which
 * ESLint resolves natively; everyone else keeps `eslint.config.js`. Still exactly one name per package,
 * so two configs remain a violation either way.
 */
function expectedEslintConfigName(pkg: LoadedPackage): string {
  return pkg.packageJson.type === 'commonjs' ? 'eslint.config.mjs' : 'eslint.config.js';
}

export function validateEslintConfigs(
  packages: readonly LoadedPackage[],
  listFiles: (directory: string) => readonly string[] = directoryFileNames,
): readonly Violation[] {
  const violations: Violation[] = [];

  for (const pkg of packages) {
    if (pkg.inventory.kind === 'published' || pkg.inventory.kind === 'workspace-root') continue;
    if (pkg.packageJson.scripts?.lint === undefined) continue;

    const expected = expectedEslintConfigName(pkg);
    const configFiles = listFiles(pkg.directory).filter(isEslintConfigFileName).sort();
    if (!configFiles.includes(expected)) {
      violations.push({
        rule: '5.1.7',
        packageKey: pkg.key,
        message: `package with a 'lint' script must contain the exact file '${expected}'`,
      });
    }

    for (const configFile of configFiles) {
      if (configFile === expected) continue;
      violations.push({
        rule: '5.1.7',
        packageKey: pkg.key,
        message: `ESLint configuration file '${configFile}' is forbidden; use only '${expected}'`,
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

function isEslintConfigFileName(fileName: string): boolean {
  return /^eslint\.config\..+$/.test(fileName) || /^\.eslintrc(?:\..+)?$/.test(fileName);
}

function isPrettierBaseReExport(contents: string, expectedImport: string): boolean {
  const escapedImport = expectedImport.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
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

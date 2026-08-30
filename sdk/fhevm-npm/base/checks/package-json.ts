import { readFileSync, statSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import { sortPackageJsonFields } from '../../package-json-order.ts';
import type { Violation } from '../diagnostics.ts';
import { consumerModuleKinds } from '../module-kind.ts';
import { type LoadedPackage, dependencyDeclarations } from '../npm.ts';

export function validatePackageJson(
  packages: readonly LoadedPackage[],
  policies: NpmManifest['packageJson'] = {},
  isFile: (file: string) => boolean = existingFile,
): readonly Violation[] {
  const violations: Violation[] = [];

  for (const pkg of packages) {
    const packageJsonKey = pkg.key === '.' ? './package.json' : `${pkg.key}/package.json`;
    const actualFields = Object.keys(pkg.packageJson);
    const expectedFields = sortPackageJsonFields(actualFields);
    if (actualFields.some((field, index) => field !== expectedFields[index])) {
      violations.push({
        rule: 'package-json-order',
        packageKey: packageJsonKey,
        message: `top-level entries must follow package-json-order.ts; expected: ${expectedFields.join(', ')}`,
      });
    }

    if (Object.hasOwn(pkg.packageJson, '//')) {
      violations.push({
        rule: 'package-json',
        packageKey: packageJsonKey,
        message: 'package.json must not contain a top-level "//" entry',
      });
    }

    const expectedConsumerType = consumerPackageType(pkg.key);
    if (pkg.packageJson.type === undefined) {
      violations.push({
        rule: 'package-json-type',
        packageKey: packageJsonKey,
        message:
          expectedConsumerType === undefined
            ? "package.json must define 'type' as 'module' or 'commonjs'"
            : `${expectedConsumerType.label} consumer package must define "type": "${expectedConsumerType.type}"`,
      });
    } else if (expectedConsumerType !== undefined && pkg.packageJson.type !== expectedConsumerType.type) {
      violations.push({
        rule: 'package-json-type',
        packageKey: packageJsonKey,
        message: `${expectedConsumerType.label} consumer package must define "type": "${expectedConsumerType.type}"`,
      });
    }

    const moduleKinds = consumerModuleKinds(pkg.packageJson);
    const actualManifestType = moduleKinds.length === 2 ? 'dual' : moduleKinds[0];
    if (actualManifestType !== pkg.inventory.type) {
      violations.push({
        rule: 'package-json-type',
        packageKey: packageJsonKey,
        message: `manifest type '${pkg.inventory.type}' does not match package.json entry points '${actualManifestType}'`,
      });
    }

    if (pkg.inventory.kind === 'published') {
      if (pkg.packageJson.license !== 'BSD-3-Clause-Clear') {
        const actualLicense =
          typeof pkg.packageJson.license === 'string' ? `"${pkg.packageJson.license}"` : '<missing>';
        violations.push({
          rule: 'published-license',
          packageKey: packageJsonKey,
          message: `published package must set "license": "BSD-3-Clause-Clear"; found ${actualLicense}`,
        });
      }
      if (!isFile(join(pkg.directory, 'LICENSE'))) {
        violations.push({
          rule: 'published-license',
          packageKey: packageJsonKey,
          message: "published package must contain a regular 'LICENSE' file next to package.json",
        });
      }
    }

    violations.push(...validateNodeTypesVersion(packageJsonKey, pkg));

    const policy = policies[pkg.inventory.kind];
    if (policy !== undefined) {
      for (const field of policy.required) {
        if (pkg.inventory.kind === 'published' && field === 'license') continue;
        if (Object.hasOwn(pkg.packageJson, field)) continue;
        violations.push({
          rule: 'package-json-policy',
          packageKey: packageJsonKey,
          message: `kind '${pkg.inventory.kind}' must define '${field}' as required by npm-manifest.json#packageJson.${pkg.inventory.kind}.required`,
        });
      }
      for (const field of policy.excluded) {
        if (!Object.hasOwn(pkg.packageJson, field)) continue;
        violations.push({
          rule: 'package-json-policy',
          packageKey: packageJsonKey,
          message: `kind '${pkg.inventory.kind}' must not contain '${field}' as excluded by npm-manifest.json#packageJson.${pkg.inventory.kind}.excluded`,
        });
      }
    }

    const actualScripts = Object.keys(pkg.packageJson.scripts ?? {});
    const sortedScripts = [...actualScripts].sort();
    if (actualScripts.some((script, index) => script !== sortedScripts[index])) {
      violations.push({
        rule: 'scripts-order',
        packageKey: packageJsonKey,
        message: "'scripts' entries must be alphabetically ordered",
      });
    }

    // `workspaces` is deliberately NOT ordered here. npm runs `--workspaces` scripts in the order this
    // array lists them, so it carries build order: a member that consumes a sibling's build output must
    // come after it. Alphabetical order cannot express that, and enforcing it silently reintroduced the
    // failure where `hardhat/v2/e2e` built before the `hardhat/v2/plugin` it depends on.
  }

  return violations;
}

function validateNodeTypesVersion(packageJsonKey: string, pkg: LoadedPackage): readonly Violation[] {
  const declarations = dependencyDeclarations(pkg.packageJson).filter(
    (declaration) => declaration.name === '@types/node',
  );
  if (declarations.length === 0) return [];

  const engine = pkg.packageJson.engines?.node;
  const engineMajor = engine === undefined ? undefined : minimumNodeMajor(engine);
  if (engineMajor === undefined) {
    return [
      {
        rule: 'node-types-version',
        packageKey: packageJsonKey,
        message:
          engine === undefined
            ? 'declares \'@types/node\' but must also define \'engines.node\', for example "node": ">=22"'
            : `cannot determine the minimum Node major from 'engines.node' value "${engine}"`,
      },
    ];
  }

  const violations: Violation[] = [];
  for (const declaration of declarations) {
    const typesMajor = nodeTypesMajor(declaration.spec);
    if (typesMajor === engineMajor) continue;
    violations.push({
      rule: 'node-types-version',
      packageKey: packageJsonKey,
      message:
        typesMajor === undefined
          ? `'@types/node' in '${declaration.field}' has unsupported version "${declaration.spec}"; its major must equal the 'engines.node' minimum major ${engineMajor}`
          : `'@types/node' in '${declaration.field}' has major ${typesMajor}, but 'engines.node' "${engine}" requires major ${engineMajor}`,
    });
  }
  return violations;
}

function minimumNodeMajor(engine: string): number | undefined {
  const match = /^>=\s*(\d+)(?:\.\d+){0,2}$/.exec(engine);
  return match?.[1] === undefined ? undefined : Number(match[1]);
}

function nodeTypesMajor(version: string): number | undefined {
  const match = /^[~^]?(\d+)(?:\.\d+){0,2}$/.exec(version);
  return match?.[1] === undefined ? undefined : Number(match[1]);
}

function existingFile(file: string): boolean {
  try {
    return statSync(file).isFile();
  } catch {
    return false;
  }
}

function consumerPackageType(
  packageKey: string,
): { readonly label: 'CJS' | 'ESM'; readonly type: 'commonjs' | 'module' } | undefined {
  if (packageKey.endsWith('/test-consumer/cjs')) return { label: 'CJS', type: 'commonjs' };
  if (packageKey.endsWith('/test-consumer/esm')) return { label: 'ESM', type: 'module' };
  return undefined;
}

export function sortPackageJson(packages: readonly LoadedPackage[]): readonly string[] {
  const sortedFiles: string[] = [];

  for (const pkg of packages) {
    const actualFields = Object.keys(pkg.packageJson);
    const sortedFields = sortPackageJsonFields(actualFields);
    const scripts = pkg.packageJson.scripts;
    const actualScripts = Object.keys(scripts ?? {});
    const sortedScripts = [...actualScripts].sort();
    const scriptsChanged = actualScripts.some((script, index) => script !== sortedScripts[index]);
    const fieldsChanged = actualFields.some((field, index) => field !== sortedFields[index]);
    if (!fieldsChanged && !scriptsChanged) continue;

    const packageJsonFile = join(pkg.directory, 'package.json');
    const packageJson = JSON.parse(readFileSync(packageJsonFile, 'utf8')) as Record<string, unknown>;
    if (scriptsChanged && scripts !== undefined) {
      packageJson.scripts = Object.fromEntries(sortedScripts.map((script) => [script, scripts[script]!]));
    }
    // `workspaces` is left exactly as written — it encodes build order. See the check above.
    const sortedPackageJson = Object.fromEntries(sortedFields.map((field) => [field, packageJson[field]]));
    writeFileSync(packageJsonFile, `${JSON.stringify(sortedPackageJson, null, 2)}\n`);
    sortedFiles.push(pkg.key === '.' ? './package.json' : `${pkg.key}/package.json`);
  }

  return sortedFiles;
}

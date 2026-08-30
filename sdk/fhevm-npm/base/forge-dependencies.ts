import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

import type { NpmManifest } from '../manifest.ts';
import { loadPackages, type LoadedPackage } from './npm.ts';

export type ForgeDependencyProject = {
  readonly packageKey: string;
  readonly packageName?: string;
  readonly directory: string;
  readonly dependencies: Readonly<Record<string, string>>;
};

export type ForgeCommandRunner = {
  readonly readConfig: (directory: string) => unknown;
  readonly install: (directory: string) => void;
};

export type InstallForgeDependenciesOptions = {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly packageSelector?: string;
  readonly runner?: ForgeCommandRunner;
};

export function installForgeDependencies(options: InstallForgeDependenciesOptions): void {
  const runner = options.runner ?? defaultForgeCommandRunner;
  const projects = discoverForgeDependencyProjects(options.workspaceRoot, options.manifest, runner);
  const selected = selectForgeDependencyProjects(projects, options.packageSelector);

  if (selected.length === 0) {
    console.log('No manifest package declares Forge dependencies.');
    return;
  }

  for (const project of selected) {
    const dependencies = Object.entries(project.dependencies)
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([name, version]) => `${name}@${version}`)
      .join(', ');
    console.log(`📦 ${project.packageKey}: forge soldeer install (${dependencies})`);
    runner.install(project.directory);
    console.log(`✅ ${project.packageKey}: Forge dependencies installed`);
  }
}

export function discoverForgeDependencyProjects(
  workspaceRoot: string,
  manifest: NpmManifest,
  runner: ForgeCommandRunner = defaultForgeCommandRunner,
): readonly ForgeDependencyProject[] {
  const projects: ForgeDependencyProject[] = [];
  for (const pkg of loadPackages(workspaceRoot, manifest)) {
    if (!existsSync(join(pkg.directory, 'foundry.toml'))) continue;
    const dependencies = parseForgeDependencies(runner.readConfig(pkg.directory), pkg);
    if (Object.keys(dependencies).length === 0) continue;
    projects.push({
      packageKey: pkg.key,
      packageName: pkg.packageJson.name,
      directory: pkg.directory,
      dependencies,
    });
  }
  return projects;
}

export function selectForgeDependencyProjects(
  projects: readonly ForgeDependencyProject[],
  selector?: string,
): readonly ForgeDependencyProject[] {
  if (selector === undefined) return projects;
  const normalized = selector === '.' || selector.startsWith('./') ? selector : `./${selector.replace(/^\//, '')}`;
  const matches = projects.filter(
    (project) => project.packageKey === normalized || project.packageName === selector,
  );
  if (matches.length === 0) {
    throw new Error(`No manifest package with Forge dependencies matches '${selector}'`);
  }
  if (matches.length > 1) {
    throw new Error(
      `Forge dependency package selector '${selector}' is ambiguous; use a package path: ${matches
        .map((project) => project.packageKey)
        .join(', ')}`,
    );
  }
  return matches;
}

const defaultForgeCommandRunner: ForgeCommandRunner = {
  readConfig(directory) {
    const output = execFileSync('forge', ['config', '--json'], { cwd: directory, encoding: 'utf8' });
    return JSON.parse(output) as unknown;
  },
  install(directory) {
    execFileSync('forge', ['soldeer', 'install'], { cwd: directory, stdio: 'inherit' });
  },
};

function parseForgeDependencies(value: unknown, pkg: LoadedPackage): Readonly<Record<string, string>> {
  if (!isRecord(value) || !isRecord(value.dependencies)) {
    throw new Error(`${pkg.key}: forge config --json did not return a dependencies object`);
  }
  const dependencies: Record<string, string> = {};
  for (const [name, version] of Object.entries(value.dependencies)) {
    if (typeof version !== 'string') {
      throw new Error(`${pkg.key}: Forge dependency '${name}' does not have a string version`);
    }
    dependencies[name] = version;
  }
  return dependencies;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

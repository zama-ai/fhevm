import { execFileSync } from 'node:child_process';

import { memoizedForgeConfigReader } from './forge-config.ts';
import { existsSync, rmSync } from 'node:fs';
import { basename, join, relative, resolve, isAbsolute } from 'node:path';
import { createInterface } from 'node:readline/promises';

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

export type CleanForgeDependenciesOptions = {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly packageSelector?: string;
  readonly dryRun: boolean;
  readonly force: boolean;
  readonly runner?: ForgeCommandRunner;
};

/**
 * The directories forge installs dependencies into, asked of `forge config --json` rather than
 * guessed: every `libs` entry except npm's `node_modules` trees (those are `npm install`'s to
 * restore). Only directories inside the package are eligible — a lib pointing outside it is shared
 * territory this command must not delete.
 */
export function forgeDependencyDirectories(config: unknown, packageDirectory: string): readonly string[] {
  if (!isRecord(config) || !Array.isArray(config.libs)) return [];
  const directories: string[] = [];
  for (const lib of config.libs) {
    if (typeof lib !== 'string' || basename(lib) === 'node_modules') continue;
    const target = resolve(packageDirectory, lib);
    const rel = relative(packageDirectory, target);
    if (rel === '' || rel.startsWith('..') || isAbsolute(rel)) continue;
    directories.push(target);
  }
  return directories;
}

/** Deletes the forge-installed dependency trees, after SHOWING them and asking — mirror of the
 * clean-node-modules.sh contract: `--dry-run` lists and stops, `--force` skips the question, and a
 * non-interactive stdin without `--force` refuses rather than deleting silently. */
export async function cleanForgeDependencies(options: CleanForgeDependenciesOptions): Promise<void> {
  const runner = options.runner ?? defaultForgeCommandRunner;
  const projects = discoverForgeDependencyProjects(options.workspaceRoot, options.manifest, runner);
  const selected = selectForgeDependencyProjects(projects, options.packageSelector);

  const targets: Array<{ readonly packageKey: string; readonly directory: string }> = [];
  for (const project of selected) {
    for (const directory of forgeDependencyDirectories(runner.readConfig(project.directory), project.directory)) {
      if (existsSync(directory)) targets.push({ packageKey: project.packageKey, directory });
    }
  }

  if (targets.length === 0) {
    console.log('No installed Forge dependency directories to remove.');
    return;
  }

  console.log('The following Forge dependency directories will be removed:');
  console.log('');
  for (const target of targets) {
    console.log(`   ${directorySize(target.directory).padStart(8)}  ${target.packageKey}  ${target.directory}`);
  }
  console.log('');
  console.log(`   ${String(targets.length)} directory(ies). Restore afterwards with: install-forge-dependencies`);

  if (options.dryRun) {
    console.log('');
    console.log('   Dry run — nothing was removed.');
    return;
  }

  if (!options.force) {
    if (!process.stdin.isTTY) {
      throw new Error('stdin is not a terminal, so there is nobody to confirm. Re-run with --force.');
    }
    const readline = createInterface({ input: process.stdin, output: process.stdout });
    const reply = (await readline.question('Remove them? [y/N] ')).trim().toLowerCase();
    readline.close();
    if (reply !== 'y' && reply !== 'yes') {
      console.log('Aborted. Nothing was removed.');
      return;
    }
  }

  for (const target of targets) {
    rmSync(target.directory, { recursive: true, force: true });
  }
  console.log(`   🧹 removed ${String(targets.length)} directory(ies).`);
}

function directorySize(directory: string): string {
  try {
    return execFileSync('du', ['-sh', directory], { encoding: 'utf8' }).split('\t')[0]?.trim() ?? '?';
  } catch {
    return '?';
  }
}

export function selectForgeDependencyProjects(
  projects: readonly ForgeDependencyProject[],
  selector?: string,
): readonly ForgeDependencyProject[] {
  if (selector === undefined) return projects;
  const normalized = selector === '.' || selector.startsWith('./') ? selector : `./${selector.replace(/^\//, '')}`;
  const matches = projects.filter((project) => project.packageKey === normalized || project.packageName === selector);
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

const sharedForgeConfigReader = memoizedForgeConfigReader();

const defaultForgeCommandRunner: ForgeCommandRunner = {
  // Routed through the shared reader rather than shelling out again: `forge config --json` has exactly
  // one caller in this codebase, so there is one place that can be wrong about how forge is asked.
  readConfig: sharedForgeConfigReader,
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

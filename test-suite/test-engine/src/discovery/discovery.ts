import { existsSync, promises as fs } from 'node:fs';
import path from 'node:path';

import { parseManifest } from '../manifest/manifest.js';
import { MANIFEST_FILE_NAME, ManifestError, type ScenarioManifest } from '../manifest/types.js';
import { compiledPathFor, isWithin } from '../paths.js';

/** A manifest that passed every check, with all of its paths resolved to absolute paths. */
export interface DiscoveredScenario {
  manifest: ScenarioManifest;
  manifestPath: string;
  directory: string;
  featurePath: string;
  /** Compiled JavaScript of the scenario's support code, in manifest order. */
  supportModules: string[];
}

export interface DiscoveryOptions {
  scenariosDir: string;
  /** Root of the TypeScript sources compiled by `tsc` (the package root). */
  sourceRoot: string;
  /** Output directory of `tsc`. */
  distDir: string;
}

export interface DiscoveryResult {
  /** Valid scenarios, sorted by id. */
  scenarios: DiscoveredScenario[];
  /** One entry per invalid manifest (or per duplicated id). Empty when everything is valid. */
  errors: ManifestError[];
}

async function findManifestPaths(dir: string): Promise<string[]> {
  const entries = await fs.readdir(dir, { withFileTypes: true });
  const found: string[] = [];
  for (const entry of entries) {
    const entryPath = path.join(dir, entry.name);
    if (entry.isDirectory() && entry.name !== 'node_modules') {
      found.push(...(await findManifestPaths(entryPath)));
    } else if (entry.isFile() && entry.name === MANIFEST_FILE_NAME) {
      found.push(entryPath);
    }
  }
  return found;
}

function resolveScenario(
  manifestPath: string,
  manifest: ScenarioManifest,
  options: DiscoveryOptions,
): DiscoveredScenario {
  const directory = path.dirname(manifestPath);
  const problems: string[] = [];

  const resolveLocal = (field: string, relativePath: string): string | undefined => {
    const absolute = path.resolve(directory, relativePath);
    if (!isWithin(directory, absolute)) {
      problems.push(`${field}: '${relativePath}' must stay inside the scenario directory`);
      return undefined;
    }
    if (!existsSync(absolute)) {
      problems.push(`${field}: file not found: ${relativePath}`);
      return undefined;
    }
    return absolute;
  };

  const featurePath = resolveLocal('spec.entrypoint', manifest.spec.entrypoint);

  const supportModules: string[] = [];
  for (const source of manifest.spec.supportCode) {
    const sourcePath = resolveLocal('spec.supportCode', source);
    if (!sourcePath) continue;
    const compiled = compiledPathFor(sourcePath, options.sourceRoot, options.distDir);
    if (!compiled) {
      problems.push(`spec.supportCode: '${source}' is outside the compiled source root ${options.sourceRoot}`);
    } else if (!existsSync(compiled)) {
      problems.push(`spec.supportCode: '${source}' has not been compiled (expected ${compiled}); run 'npm run build'`);
    } else {
      supportModules.push(compiled);
    }
  }

  if (problems.length > 0 || !featurePath) throw new ManifestError(manifestPath, problems);
  return { manifest, manifestPath, directory, featurePath, supportModules };
}

/**
 * Finds every `scenario.yaml` below `scenariosDir`, validates it and resolves the files it
 * references. Invalid manifests are reported, never silently dropped.
 */
export async function discoverScenarios(options: DiscoveryOptions): Promise<DiscoveryResult> {
  if (!existsSync(options.scenariosDir)) {
    throw new Error(`Scenarios directory not found: ${options.scenariosDir}`);
  }

  const manifestPaths = (await findManifestPaths(options.scenariosDir)).sort();
  const scenarios: DiscoveredScenario[] = [];
  const errors: ManifestError[] = [];

  for (const manifestPath of manifestPaths) {
    try {
      const manifest = parseManifest(manifestPath, await fs.readFile(manifestPath, 'utf8'));
      scenarios.push(resolveScenario(manifestPath, manifest, options));
    } catch (error) {
      if (!(error instanceof ManifestError)) throw error;
      errors.push(error);
    }
  }

  const byId = new Map<string, DiscoveredScenario[]>();
  for (const scenario of scenarios) {
    const id = scenario.manifest.metadata.id;
    byId.set(id, [...(byId.get(id) ?? []), scenario]);
  }
  const unique: DiscoveredScenario[] = [];
  for (const [id, group] of byId) {
    if (group.length === 1) {
      unique.push(group[0]!);
      continue;
    }
    for (const duplicate of group) {
      const others = group
        .filter((other) => other !== duplicate)
        .map((other) => path.relative(options.scenariosDir, other.manifestPath));
      errors.push(
        new ManifestError(duplicate.manifestPath, [`duplicate metadata.id '${id}' (also in ${others.join(', ')})`]),
      );
    }
  }

  unique.sort((a, b) => a.manifest.metadata.id.localeCompare(b.manifest.metadata.id));
  return { scenarios: unique, errors };
}

import { mkdir, rm, stat } from "fs/promises";
import { join } from "path";

export interface DotFhevmPaths {
  root: string;
  env: string;
  compose: string;
  keys: string;
  keysMinioSnapshot: string;
  keysVolumeSnapshot: string;
  logs: string;
  stateFile: string;
  versionCache: string;
}

export function resolveProjectRoot(): string {
  return process.cwd();
}

export function getDotFhevmPaths(projectRoot = resolveProjectRoot()): DotFhevmPaths {
  const root = join(projectRoot, ".fhevm");
  const keys = join(root, "keys");

  return {
    root,
    env: join(projectRoot, "test-suite", "fhevm", "env", "staging"),
    compose: join(root, "compose"),
    keys,
    keysMinioSnapshot: join(keys, "minio-snapshot"),
    keysVolumeSnapshot: join(keys, "volume-snapshot"),
    logs: join(root, "logs"),
    stateFile: join(root, "state.json"),
    versionCache: join(root, "version-cache.json"),
  };
}

export async function ensureDotFhevm(projectRoot = resolveProjectRoot()): Promise<DotFhevmPaths> {
  const paths = getDotFhevmPaths(projectRoot);
  await Promise.all([
    mkdir(paths.env, { recursive: true }),
    mkdir(paths.compose, { recursive: true }),
    mkdir(paths.keysMinioSnapshot, { recursive: true }),
    mkdir(paths.keysVolumeSnapshot, { recursive: true }),
    mkdir(paths.logs, { recursive: true }),
  ]);
  return paths;
}

export async function cleanDotFhevm(
  paths: DotFhevmPaths,
  options: { all: boolean; dryRun: boolean },
): Promise<string[]> {
  if (!(await exists(paths.root))) {
    return [];
  }

  if (options.all) {
    if (options.dryRun) {
      return [paths.root];
    }
    await rm(paths.root, { recursive: true, force: true });
    return [paths.root];
  }

  const targets = [paths.env, paths.compose, paths.logs, paths.stateFile, paths.versionCache];
  const removable = [] as string[];

  for (const target of targets) {
    if (await exists(target)) {
      removable.push(target);
      if (!options.dryRun) {
        await rm(target, { recursive: true, force: true });
      }
    }
  }

  return removable;
}

async function exists(path: string): Promise<boolean> {
  try {
    await stat(path);
    return true;
  } catch {
    return false;
  }
}

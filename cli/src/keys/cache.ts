import { mkdir, readdir, rm } from "fs/promises";
import { join } from "path";

import type { DotFhevmPaths } from "../config/dotfhevm";
import type { FhevmConfig } from "../config/model";

import { exportVolume, importVolume } from "../docker/volumes";
import { downloadBucket, uploadBucket } from "./minio-s3";

export interface KeyCacheState {
  hasMinioSnapshot: boolean;
  hasVolumeSnapshot: boolean;
  isComplete: boolean;
  snapshotDate?: string;
}

interface KeyCacheOps {
  downloadBucket: typeof downloadBucket;
  uploadBucket: typeof uploadBucket;
  exportVolume: typeof exportVolume;
  importVolume: typeof importVolume;
}

interface SnapshotMeta {
  date: string;
  minioObjectCount: number;
  volumeExported: boolean;
}

const DEFAULT_OPS: KeyCacheOps = {
  downloadBucket,
  uploadBucket,
  exportVolume,
  importVolume,
};

const SNAPSHOT_META_FILE = "snapshot-meta.json";
const KEYS_CACHE_VOLUME = "fhevm_keys-cache";

let ops: KeyCacheOps = DEFAULT_OPS;

function resolveMinioEndpoint(config: FhevmConfig): string {
  // Always use localhost for host-side MinIO access (container IPs are not
  // reachable from macOS). The container IP in config.runtime.minioIp is only
  // for container-to-container env vars.
  return `http://localhost:${config.ports.minioApi}`;
}

async function directoryHasFiles(path: string): Promise<boolean> {
  try {
    const entries = await readdir(path, { withFileTypes: true });

    for (const entry of entries) {
      if (entry.isFile()) {
        return true;
      }
      if (entry.isDirectory() && (await directoryHasFiles(join(path, entry.name)))) {
        return true;
      }
    }

    return false;
  } catch {
    return false;
  }
}

async function readSnapshotDate(paths: DotFhevmPaths): Promise<string | undefined> {
  const filePath = join(paths.keys, SNAPSHOT_META_FILE);
  if (!(await Bun.file(filePath).exists())) {
    return undefined;
  }

  try {
    const parsed = JSON.parse(await Bun.file(filePath).text()) as Partial<SnapshotMeta>;
    return typeof parsed.date === "string" ? parsed.date : undefined;
  } catch {
    return undefined;
  }
}

export async function checkKeyCache(paths: DotFhevmPaths): Promise<KeyCacheState> {
  const [hasMinioSnapshot, hasVolumeSnapshot, snapshotDate] = await Promise.all([
    directoryHasFiles(paths.keysMinioSnapshot),
    directoryHasFiles(paths.keysVolumeSnapshot),
    readSnapshotDate(paths),
  ]);

  return {
    hasMinioSnapshot,
    hasVolumeSnapshot,
    isComplete: hasMinioSnapshot && hasVolumeSnapshot,
    snapshotDate,
  };
}

export async function snapshotKeys(paths: DotFhevmPaths, config: FhevmConfig): Promise<void> {
  await mkdir(paths.keysMinioSnapshot, { recursive: true });
  await mkdir(paths.keysVolumeSnapshot, { recursive: true });

  const endpoint = resolveMinioEndpoint(config);
  const minioObjectCount = await ops.downloadBucket(endpoint, config.minio.buckets.public, paths.keysMinioSnapshot);
  await ops.exportVolume(KEYS_CACHE_VOLUME, paths.keysVolumeSnapshot);

  const meta: SnapshotMeta = {
    date: new Date().toISOString(),
    minioObjectCount,
    volumeExported: true,
  };

  await Bun.write(join(paths.keys, SNAPSHOT_META_FILE), `${JSON.stringify(meta, null, 2)}\n`);
}

export async function restoreKeys(paths: DotFhevmPaths, config: FhevmConfig): Promise<void> {
  await ops.importVolume(paths.keysVolumeSnapshot, KEYS_CACHE_VOLUME);
  await ops.uploadBucket(resolveMinioEndpoint(config), config.minio.buckets.public, paths.keysMinioSnapshot);
}

export async function clearKeyCache(paths: DotFhevmPaths): Promise<void> {
  await rm(paths.keys, { recursive: true, force: true });
}

export const __internal = {
  keysCacheVolumeName: KEYS_CACHE_VOLUME,
  resetOpsForTests(): void {
    ops = DEFAULT_OPS;
  },
  setOpsForTests(overrides: Partial<KeyCacheOps>): void {
    ops = { ...DEFAULT_OPS, ...overrides };
  },
};

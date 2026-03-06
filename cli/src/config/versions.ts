import { mkdir, rename } from "fs/promises";
import { ExitCode, FhevmCliError } from "../errors";

export type VersionGroup =
  | "coprocessor"
  | "kms-connector"
  | "contracts"
  | "core"
  | "relayer"
  | "test-suite";

export interface VersionSource {
  group: VersionGroup;
  repo: string;
  envPrefix: string;
  /** "release" (default) fetches /releases/latest, "tag" fetches /tags[0], "prefixed-release" filters releases by tagPrefix */
  strategy?: "release" | "tag" | "prefixed-release";
  /** For prefixed-release strategy: the tag prefix to match (e.g., "relayer@") */
  tagPrefix?: string;
  /** Skip version resolution and use this value directly (still overridable via env vars). */
  pinnedVersion?: string;
}

export interface VersionCache {
  versions: Record<string, { version: string; fetchedAt: number }>;
  ttlMs: number;
}

export const VERSION_TTL_MS = 24 * 60 * 60 * 1000;
export const VERSION_CACHE_PATH = ".fhevm/version-cache.json";

export const VERSION_REGISTRY: readonly VersionSource[] = [
  { group: "coprocessor", repo: "zama-ai/fhevm", envPrefix: "COPROCESSOR", pinnedVersion: "2458fa9" },
  { group: "kms-connector", repo: "zama-ai/fhevm", envPrefix: "KMS_CONNECTOR", pinnedVersion: "2458fa9" },
  { group: "contracts", repo: "zama-ai/fhevm", envPrefix: "FHEVM_CONTRACTS", pinnedVersion: "2458fa9" },
  { group: "core", repo: "zama-ai/kms", envPrefix: "KMS_CORE" },
  { group: "relayer", repo: "zama-ai/console", envPrefix: "FHEVM_RELAYER", strategy: "prefixed-release", tagPrefix: "relayer@" },
  { group: "test-suite", repo: "zama-ai/fhevm", envPrefix: "FHEVM_TEST_SUITE", pinnedVersion: "2458fa9" },
] as const;

const VERSION_ENV_MAP: Record<VersionGroup, string[]> = {
  coprocessor: [
    "COPROCESSOR_DB_MIGRATION_VERSION",
    "COPROCESSOR_HOST_LISTENER_VERSION",
    "COPROCESSOR_GW_LISTENER_VERSION",
    "COPROCESSOR_TFHE_WORKER_VERSION",
    "COPROCESSOR_ZKPROOF_WORKER_VERSION",
    "COPROCESSOR_SNS_WORKER_VERSION",
    "COPROCESSOR_TX_SENDER_VERSION",
  ],
  "kms-connector": [
    "CONNECTOR_DB_MIGRATION_VERSION",
    "CONNECTOR_GW_LISTENER_VERSION",
    "CONNECTOR_KMS_WORKER_VERSION",
    "CONNECTOR_TX_SENDER_VERSION",
  ],
  contracts: ["GATEWAY_VERSION", "HOST_VERSION"],
  core: ["CORE_VERSION"],
  relayer: ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"],
  "test-suite": ["TEST_SUITE_VERSION"],
};

function dirnameOf(filePath: string): string {
  const index = filePath.lastIndexOf("/");
  if (index === -1) {
    return ".";
  }
  return filePath.slice(0, index) || ".";
}

function getSource(group: string): VersionSource {
  const source = VERSION_REGISTRY.find((entry) => entry.group === group);
  if (!source) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      message: `unknown version group: ${group}`,
      step: "version-resolution",
    });
  }
  return source;
}

export async function loadVersionCache(cachePath: string = VERSION_CACHE_PATH): Promise<VersionCache | null> {
  const file = Bun.file(cachePath);
  if (!(await file.exists())) {
    return null;
  }

  try {
    const parsed = (await file.json()) as VersionCache;
    if (!parsed || typeof parsed !== "object" || !parsed.versions || typeof parsed.ttlMs !== "number") {
      return null;
    }
    return parsed;
  } catch {
    return null;
  }
}

export async function saveVersionCache(
  cache: VersionCache,
  cachePath: string = VERSION_CACHE_PATH,
): Promise<void> {
  await mkdir(dirnameOf(cachePath), { recursive: true });
  const tmpPath = `${cachePath}.tmp.${Date.now()}`;
  await Bun.write(tmpPath, JSON.stringify(cache, null, 2));
  await rename(tmpPath, cachePath);
}

function githubHeaders(): HeadersInit {
  const headers: HeadersInit = {
    Accept: "application/vnd.github+json",
    "User-Agent": "fhevm-cli",
  };
  const token = Bun.env.GITHUB_TOKEN;
  if (token) {
    headers.Authorization = `Bearer ${token}`;
  }
  return headers;
}

async function fetchLatestRelease(repo: string): Promise<string> {
  const response = await fetch(`https://api.github.com/repos/${repo}/releases/latest`, { headers: githubHeaders() });
  if (!response.ok) {
    throw new Error(`failed to fetch latest release for ${repo}: ${response.status}`);
  }
  const payload = (await response.json()) as { tag_name?: string };
  if (!payload.tag_name) {
    throw new Error(`missing tag_name in latest release payload for ${repo}`);
  }
  return payload.tag_name;
}

async function fetchLatestTag(repo: string): Promise<string> {
  const response = await fetch(`https://api.github.com/repos/${repo}/tags?per_page=1`, { headers: githubHeaders() });
  if (!response.ok) {
    throw new Error(`failed to fetch tags for ${repo}: ${response.status}`);
  }
  const payload = (await response.json()) as Array<{ name?: string }>;
  if (!payload[0]?.name) {
    throw new Error(`no tags found for ${repo}`);
  }
  return payload[0].name;
}

async function fetchPrefixedRelease(repo: string, prefix: string): Promise<string> {
  const response = await fetch(`https://api.github.com/repos/${repo}/releases?per_page=20`, { headers: githubHeaders() });
  if (!response.ok) {
    throw new Error(`failed to fetch releases for ${repo}: ${response.status}`);
  }
  const payload = (await response.json()) as Array<{ tag_name?: string; prerelease?: boolean }>;
  const match = payload.find((r) => r.tag_name?.startsWith(prefix) && !r.prerelease);
  const tag = match?.tag_name ?? payload.find((r) => r.tag_name?.startsWith(prefix))?.tag_name;
  if (!tag) {
    throw new Error(`no release matching prefix "${prefix}" found for ${repo}`);
  }
  // Strip the prefix to get the version (e.g., "relayer@v0.9.0" → "v0.9.0")
  return tag.slice(prefix.length);
}

export async function fetchVersion(source: VersionSource): Promise<string> {
  const strategy = source.strategy ?? "release";
  switch (strategy) {
    case "tag":
      return fetchLatestTag(source.repo);
    case "prefixed-release":
      return fetchPrefixedRelease(source.repo, source.tagPrefix ?? "");
    default:
      return fetchLatestRelease(source.repo);
  }
}

function getServiceEnvOverride(group: VersionGroup): string | undefined {
  for (const envKey of VERSION_ENV_MAP[group]) {
    const value = Bun.env[envKey];
    if (value) {
      return value;
    }
  }
  return undefined;
}

function isAuthLikeError(error: unknown): boolean {
  if (!(error instanceof Error)) return false;
  return /\b(401|403|404)\b/.test(error.message);
}

export async function resolveVersion(
  group: VersionGroup,
  cachePath: string = VERSION_CACHE_PATH,
  includeServiceOverrides = true,
): Promise<string> {
  const source = getSource(group);
  const envOverride = Bun.env[`${source.envPrefix}_VERSION`];
  if (envOverride) {
    return envOverride;
  }
  if (includeServiceOverrides) {
    const serviceOverride = getServiceEnvOverride(group);
    if (serviceOverride) {
      return serviceOverride;
    }
  }

  if (source.pinnedVersion) {
    return source.pinnedVersion;
  }

  const cache = await loadVersionCache(cachePath);
  const now = Date.now();
  const ttlMs = cache?.ttlMs ?? VERSION_TTL_MS;
  const cachedEntry = cache?.versions?.[group];
  if (cachedEntry && now - cachedEntry.fetchedAt <= ttlMs) {
    return cachedEntry.version;
  }

  try {
    const version = await fetchVersion(source);
    const nextCache: VersionCache = {
      ttlMs,
      versions: {
        ...(cache?.versions ?? {}),
        [group]: { version, fetchedAt: now },
      },
    };
    await saveVersionCache(nextCache, cachePath);
    return version;
  } catch (cause) {
    const hint = isAuthLikeError(cause) && !Bun.env.GITHUB_TOKEN
      ? ". If this is a private repository, set GITHUB_TOKEN"
      : "";
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "version-resolution",
      message: `unable to resolve version for ${group}${hint}`,
      cause,
    });
  }
}

export async function resolveAllVersions(
  cachePath: string = VERSION_CACHE_PATH,
): Promise<Record<VersionGroup, string>> {
  // Resolve sequentially to avoid concurrent cache file writes.
  const result = {} as Record<VersionGroup, string>;
  for (const source of VERSION_REGISTRY) {
    result[source.group] = await resolveVersion(source.group, cachePath, false);
  }
  return result;
}

export function buildVersionEnvVars(versions: Record<VersionGroup, string>): Record<string, string> {
  const output: Record<string, string> = {};
  for (const group of Object.keys(versions) as VersionGroup[]) {
    const source = getSource(group);
    const groupOverride = Bun.env[`${source.envPrefix}_VERSION`];
    for (const envKey of VERSION_ENV_MAP[group]) {
      output[envKey] = Bun.env[envKey] ?? groupOverride ?? versions[group];
    }
  }
  return output;
}

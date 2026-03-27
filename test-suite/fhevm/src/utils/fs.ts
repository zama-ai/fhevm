import fs from "node:fs/promises";
import path from "node:path";
import { MINIO_EXTERNAL_URL, MINIO_INTERNAL_URL, MINIO_PORT } from "../layout";

export type RunOptions = {
  cwd?: string;
  env?: Record<string, string>;
  input?: string;
  allowFailure?: boolean;
};

export type RunResult = {
  stdout: string;
  stderr: string;
  code: number;
};

/** Ensures a directory exists before generated files are written into it. */
export const ensureDir = (dir: string) => fs.mkdir(dir, { recursive: true });

/** Reads and parses a JSON file into the requested type. */
export const readJson = async <T>(file: string) => JSON.parse(await fs.readFile(file, "utf8")) as T;

/** Writes JSON atomically through a temporary file. */
export const writeJson = async (file: string, value: unknown) => {
  await ensureDir(path.dirname(file));
  const tmp = `${file}.tmp`;
  await fs.writeFile(tmp, `${JSON.stringify(value, null, 2)}\n`);
  await fs.rename(tmp, file);
};

/** Parses a dotenv-style file into a flat key/value map. */
const parseEnv = (text: string) => {
  const out: Record<string, string> = {};
  for (const line of text.split(/\r?\n/)) {
    if (!line || line.trimStart().startsWith("#")) {
      continue;
    }
    const idx = line.indexOf("=");
    if (idx < 0) {
      continue;
    }
    const key = line.slice(0, idx).trim();
    let value = line.slice(idx + 1).trim();
    if (
      (value.startsWith('"') && value.endsWith('"')) ||
      (value.startsWith("'") && value.endsWith("'"))
    ) {
      value = value.slice(1, -1);
    }
    out[key] = value;
  }
  return out;
};

/** Reads and parses a dotenv file from disk. */
export const readEnvFile = async (file: string) => parseEnv(await fs.readFile(file, "utf8"));

/** Reads a dotenv file when present and otherwise returns an empty env map. */
export const readEnvFileIfExists = async (file: string) =>
  (await exists(file)) ? readEnvFile(file) : {};

/** Quotes an env value only when writing it bare would be unsafe. */
const quoteEnvValue = (value: string) => {
  if (!needsQuotes(value)) {
    return value;
  }
  if (!value.includes("'")) {
    return `'${value}'`;
  }
  if (!value.includes('"')) {
    return `"${value}"`;
  }
  throw new Error(`Cannot safely encode env value containing both quote types: ${value}`);
};

/** Writes a normalized dotenv file with sorted keys. */
export const writeEnvFile = async (file: string, env: Record<string, string>) => {
  await ensureDir(path.dirname(file));
  const body = Object.entries(env)
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([key, value]) => `${key}=${quoteEnvValue(value)}`)
    .join("\n");
  await fs.writeFile(file, `${body}\n`);
};

/** Reports whether a filesystem path exists. */
export const exists = async (file: string) => {
  try {
    await fs.access(file);
    return true;
  } catch {
    return false;
  }
};

/** Removes a file or directory tree if it exists. */
export const remove = async (target: string) => fs.rm(target, { recursive: true, force: true });

/** Normalizes image repository strings across registry aliases. */
export const normalizeRepository = (value: string) =>
  value
    .replace(/^hub\.zama\.org\/(?:ghcr\/)?(?:zama-protocol|internal)\//, "ghcr.io/")
    .replace(/^hub\.zama\.org\/ghcr\//, "ghcr.io/")
    .replace(/^docker\.io\//, "")
    .trim();

/** Adds a `0x` prefix to a hex string when missing. */
export const withHexPrefix = (value: string) => (value.startsWith("0x") ? value : `0x${value}`);

/** Formats a bigint as a zero-padded 256-bit hex id. */
const uint256ToId = (value: bigint) => value.toString(16).padStart(64, "0");

/** Predicts the default FHE key id before bootstrap discovery runs. */
export const predictedKeyId = () => uint256ToId((4n << 248n) + 1n);
/** Predicts the default CRS key id before bootstrap discovery runs. */
export const predictedCrsId = () => uint256ToId((5n << 248n) + 1n);

/** Merges CLI flags while replacing earlier duplicates of the same flag. */
export const mergeArgs = (base: unknown, extras: string[]) => {
  const next = Array.isArray(base) ? [...base.map(String)] : [];
  for (const extra of extras) {
    const prefix = extra.startsWith("--") && extra.includes("=") ? `${extra.split("=")[0]}=` : undefined;
    for (let i = next.length - 1; i >= 0; i -= 1) {
      if (next[i] === extra || (prefix && next[i].startsWith(prefix))) {
        next.splice(i, 1);
      }
    }
    next.push(extra);
  }
  return next;
};

/** Formats a coprocessor service suffix into its instance-specific service name. */
export const toServiceName = (suffix: string, index: number) =>
  index === 0 ? `coprocessor-${suffix}` : `coprocessor${index}-${suffix}`;

/** Detects whether an env value needs quoting in a dotenv file. */
const needsQuotes = (value: string) => /\s|["'[\]{}]/.test(value);

/** Rewrites container RPC URLs into host-reachable URLs. */
export const hostReachableRpcUrl = (url: string) => {
  try {
    const next = new URL(url);
    if (/^[a-z][a-z0-9-]*$/i.test(next.hostname)) {
      next.hostname = "localhost";
    }
    return next.toString().replace(/\/$/, "");
  } catch {
    return url;
  }
};

/** Rewrites container material URLs into host-reachable MinIO URLs. */
export const hostReachableMaterialUrl = (url: string) => {
  try {
    const next = new URL(url);
    const external = new URL(MINIO_EXTERNAL_URL);
    const looksInternal =
      next.port === String(MINIO_PORT) &&
      (/^[a-z][a-z0-9-]*$/i.test(next.hostname) || /^\d+\.\d+\.\d+\.\d+$/.test(next.hostname));
    if (!looksInternal) {
      return next.toString().replace(/\/$/, "");
    }
    next.protocol = external.protocol;
    next.hostname = external.hostname;
    next.port = external.port;
    return next.toString().replace(/\/$/, "");
  } catch {
    return url === MINIO_INTERNAL_URL ? MINIO_EXTERNAL_URL : url;
  }
};

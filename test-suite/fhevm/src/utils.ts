import fs from "node:fs/promises";
import path from "node:path";

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

export type Runner = (argv: string[], options?: RunOptions) => Promise<RunResult>;

export const run: Runner = async (argv, options = {}) => {
  const proc = Bun.spawn(argv, {
    cwd: options.cwd,
    env: { ...process.env, ...options.env },
    stdin: options.input ? new Blob([options.input]) : undefined,
    stdout: "pipe",
    stderr: "pipe",
  });
  const [stdout, stderr, code] = await Promise.all([
    new Response(proc.stdout).text(),
    new Response(proc.stderr).text(),
    proc.exited,
  ]);
  if (code !== 0 && !options.allowFailure) {
    throw new Error(`${argv.join(" ")} failed (${code})\n${stderr || stdout}`.trim());
  }
  return { stdout, stderr, code };
};

export const runLive = async (argv: string[], options: Omit<RunOptions, "input"> = {}) => {
  const proc = Bun.spawn(argv, {
    cwd: options.cwd,
    env: { ...process.env, ...options.env },
    stdout: "inherit",
    stderr: "inherit",
    stdin: "inherit",
  });
  const code = await proc.exited;
  if (code !== 0 && !options.allowFailure) {
    throw new Error(`${argv.join(" ")} failed (${code})`);
  }
  return code;
};

export const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export const ensureDir = (dir: string) => fs.mkdir(dir, { recursive: true });

export const readJson = async <T>(file: string) => JSON.parse(await fs.readFile(file, "utf8")) as T;

export const writeJson = async (file: string, value: unknown) => {
  await ensureDir(path.dirname(file));
  const tmp = `${file}.tmp`;
  await fs.writeFile(tmp, `${JSON.stringify(value, null, 2)}\n`);
  await fs.rename(tmp, file);
};

export const parseEnv = (text: string) => {
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

export const readEnvFile = async (file: string) => parseEnv(await fs.readFile(file, "utf8"));

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

export const writeEnvFile = async (file: string, env: Record<string, string>) => {
  await ensureDir(path.dirname(file));
  const body = Object.entries(env)
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([key, value]) => `${key}=${quoteEnvValue(value)}`)
    .join("\n");
  await fs.writeFile(file, `${body}\n`);
};

export const exists = async (file: string) => {
  try {
    await fs.access(file);
    return true;
  } catch {
    return false;
  }
};

export const remove = async (target: string) => fs.rm(target, { recursive: true, force: true });

export const normalizeRepository = (value: string) =>
  value
    .replace(/^hub\.zama\.org\/(?:ghcr\/)?(?:zama-protocol|internal)\//, "ghcr.io/")
    .replace(/^hub\.zama\.org\/ghcr\//, "ghcr.io/")
    .replace(/^docker\.io\//, "")
    .trim();

export const withHexPrefix = (value: string) => (value.startsWith("0x") ? value : `0x${value}`);

export const uint256ToId = (value: bigint) => value.toString(16).padStart(64, "0");

export const predictedKeyId = () => uint256ToId((4n << 248n) + 1n);
export const predictedCrsId = () => uint256ToId((5n << 248n) + 1n);

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

export const toServiceName = (suffix: string, index: number) =>
  index === 0 ? `coprocessor-${suffix}` : `coprocessor${index}-${suffix}`;

export const needsQuotes = (value: string) => /\s|["'[\]{}]/.test(value);

export const hostReachableRpcUrl = (url: string) =>
  url
    .replace("http://gateway-node:8546", "http://localhost:8546")
    .replace("http://host-node:8545", "http://localhost:8545");

export const hostReachableMaterialUrl = (url: string) =>
  url
    .replace(/http:\/\/[^/]+:9000/, "http://localhost:9000")
    .replace("http://minio:9000", "http://localhost:9000");

import { createHash, randomUUID } from "node:crypto";
import { execFileSync } from "node:child_process";
import {
  closeSync,
  constants,
  fchmodSync,
  fstatSync,
  fsyncSync,
  lstatSync,
  mkdirSync,
  openSync,
  readdirSync,
  readFileSync,
  renameSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { open as openFile } from "node:fs/promises";
import { hostname } from "node:os";
import { basename, dirname, join, resolve } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { z } from "zod";

import type {
  FheHandlePoolItem,
  InputProofPoolItem,
  PersistedPoolMeta,
  PoolMeta,
} from "./types";

const DIRECTORY_MODE = 0o700;
const FILE_MODE = 0o600;
const CURSOR_SCHEMA_VERSION = 2;
const POOL_SCHEMA_VERSION = 2;
const LOCK_TIMEOUT_MS = 10_000;
const LOCK_RETRY_MS = 10;
const LOCK_HEARTBEAT_MS = 5_000;
const INVALID_LOCK_GRACE_MS = 2_000;
const CREATE_TEMP_GRACE_MS = 30_000;
const NOFOLLOW = constants.O_NOFOLLOW ?? 0;
const waitBuffer = new Int32Array(new SharedArrayBuffer(4));
const HOSTNAME = hostname();
const PROCESS_STARTED_AT_MS = Math.round(Date.now() - process.uptime() * 1_000);

const isErrno = (error: unknown, code: string): boolean =>
  (error as NodeJS.ErrnoException).code === code;

/**
 * Returns an OS-verified process incarnation, not merely a PID. Undefined
 * means this platform cannot safely distinguish a live PID from a reused PID.
 */
const verifiedProcessIdentity = (pid: number): string | undefined => {
  try {
    if (process.platform === "linux") {
      const bootId = readFileSync("/proc/sys/kernel/random/boot_id", "utf8").trim();
      const stat = readFileSync(`/proc/${pid.toString()}/stat`, "utf8");
      const commandEnd = stat.lastIndexOf(")");
      if (commandEnd < 0) return undefined;
      // Fields after the command name begin at field 3; starttime is field 22.
      const fields = stat.slice(commandEnd + 1).trim().split(/\s+/);
      const startTicks = fields[19];
      return bootId && startTicks ? `linux:${bootId}:${startTicks}` : undefined;
    }
    if (process.platform === "darwin") {
      const started = execFileSync(
        "/bin/ps",
        ["-p", pid.toString(), "-o", "lstart="],
        { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"], timeout: 1_000 },
      ).trim();
      return started ? `darwin:${started}` : undefined;
    }
  } catch {
    // Fail closed: never reap a live PID without a verified incarnation.
  }
  return undefined;
};

const PROCESS_IDENTITY = verifiedProcessIdentity(process.pid);

const pathExistsNoFollow = (path: string): boolean => {
  try {
    lstatSync(path);
    return true;
  } catch (error) {
    if (isErrno(error, "ENOENT")) return false;
    throw error;
  }
};

const assertDirectory = (path: string, repairMode = false): void => {
  const entry = lstatSync(path);
  if (entry.isSymbolicLink() || !entry.isDirectory()) {
    throw new Error(`Expected a real directory, refusing symlink or non-directory: ${path}`);
  }
  if (process.platform === "win32") return;
  const fd = openSync(path, constants.O_RDONLY | NOFOLLOW);
  try {
    if (!fstatSync(fd).isDirectory()) throw new Error(`Expected a real directory: ${path}`);
    if (repairMode) fchmodSync(fd, DIRECTORY_MODE);
  } finally {
    closeSync(fd);
  }
};

const ensurePrivateDirectory = (path: string): void => {
  mkdirSync(path, { recursive: true, mode: DIRECTORY_MODE });
  assertDirectory(path, true);
};

/** Directory-entry fsync is unavailable on Windows and some network filesystems. */
const syncDirectoryBestEffort = (path: string): void => {
  if (process.platform === "win32") return;
  let fd: number | undefined;
  try {
    fd = openSync(path, constants.O_RDONLY | NOFOLLOW);
    fsyncSync(fd);
  } catch (error) {
    if (!["EINVAL", "ENOTSUP", "EOPNOTSUPP", "EPERM", "EISDIR"].some((code) =>
      isErrno(error, code),
    )) {
      throw error;
    }
  } finally {
    if (fd !== undefined) closeSync(fd);
  }
};

const readRegularFile = (path: string): Buffer => {
  let fd: number | undefined;
  try {
    fd = openSync(path, constants.O_RDONLY | NOFOLLOW);
    if (!fstatSync(fd).isFile()) throw new Error(`Expected a regular file: ${path}`);
    const contents = readFileSync(fd);
    if (process.platform !== "win32") fchmodSync(fd, FILE_MODE);
    return contents;
  } finally {
    if (fd !== undefined) closeSync(fd);
  }
};

/** Private unique temp, file fsync, rename publish, then best-effort directory fsync. */
const atomicWritePrivateFile = (
  path: string,
  contents: string,
  beforePublish?: () => void,
): void => {
  const parent = dirname(path);
  ensurePrivateDirectory(parent);
  const tmp = `${path}.tmp-${process.pid.toString()}-${randomUUID()}`;
  let fd: number | undefined;
  try {
    fd = openSync(
      tmp,
      constants.O_CREAT | constants.O_EXCL | constants.O_WRONLY | NOFOLLOW,
      FILE_MODE,
    );
    writeFileSync(fd, contents, "utf8");
    fsyncSync(fd);
    closeSync(fd);
    fd = undefined;
    // Keep ownership validation adjacent to the commit-pointer rename rather
    // than before the potentially slow temp write and fsync.
    beforePublish?.();
    renameSync(tmp, path);
    syncDirectoryBestEffort(parent);
  } catch (error) {
    if (fd !== undefined) closeSync(fd);
    try {
      rmSync(tmp, { force: true });
    } catch (cleanupError) {
      throw new AggregateError([error, cleanupError], `Could not clean up ${tmp}`);
    }
    throw error;
  }
};

type LockOwner = Readonly<{
  schemaVersion: 3 | 4;
  hostname: string;
  pid: number;
  processStartedAtMs?: number;
  processIdentity?: string;
  token: string;
  createdAtMs: number;
  heartbeatAtMs: number;
}>;

const lockOwnerPath = (path: string): string => join(path, "owner.json");

const validLockOwner = (value: unknown): value is LockOwner => {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return false;
  const owner = value as Partial<LockOwner>;
  return (
    (owner.schemaVersion === 3 || owner.schemaVersion === 4) &&
    typeof owner.hostname === "string" && owner.hostname.length > 0 &&
    Number.isSafeInteger(owner.pid) &&
    (owner.pid ?? 0) > 0 &&
    (owner.schemaVersion === 3
      ? Number.isFinite(owner.processStartedAtMs)
      : owner.processIdentity === undefined ||
        (typeof owner.processIdentity === "string" && owner.processIdentity.length > 0)) &&
    typeof owner.token === "string" &&
    owner.token.length > 0 &&
    Number.isFinite(owner.createdAtMs) &&
    Number.isFinite(owner.heartbeatAtMs)
  );
};

const readLockOwner = (path: string): LockOwner | undefined => {
  try {
    const value: unknown = JSON.parse(readRegularFile(lockOwnerPath(path)).toString("utf8"));
    return validLockOwner(value) ? value : undefined;
  } catch {
    return undefined;
  }
};

const processIsAlive = (pid: number): boolean => {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return !isErrno(error, "ESRCH");
  }
};

const writeLockOwner = (path: string, owner: LockOwner): void => {
  atomicWritePrivateFile(lockOwnerPath(path), `${JSON.stringify(owner)}\n`);
};

const tryRecoverStaleLock = (path: string): void => {
  let observed = readLockOwner(path);
  let stale = false;
  if (observed) {
    // Writer locks are intentionally local-host only. A foreign-host owner is
    // never reaped because a local PID probe says nothing about that process.
    const alive = processIsAlive(observed.pid);
    const liveIdentity = alive
      ? verifiedProcessIdentity(observed.pid)
      : undefined;
    stale = observed.hostname === HOSTNAME && (
      !alive ||
      (observed.processIdentity !== undefined &&
        liveIdentity !== undefined &&
        observed.processIdentity !== liveIdentity)
    );
  } else {
    try {
      stale = Date.now() - statSync(path).mtimeMs >= INVALID_LOCK_GRACE_MS;
    } catch (error) {
      if (isErrno(error, "ENOENT")) return;
      throw error;
    }
  }
  if (!stale) return;

  // Re-read immediately before atomic quarantine so a heartbeat cannot be reaped.
  const current = readLockOwner(path);
  if (
    observed &&
    (!current ||
      current.token !== observed.token ||
      current.heartbeatAtMs !== observed.heartbeatAtMs)
  ) {
    return;
  }
  if (!observed && current) return;

  const quarantine = `${path}.reaped-${randomUUID()}`;
  try {
    renameSync(path, quarantine);
  } catch (error) {
    if (isErrno(error, "ENOENT")) return;
    throw error;
  }
  rmSync(quarantine, { recursive: true, force: true });
  syncDirectoryBestEffort(dirname(path));
};

class StorageLock {
  private heartbeat: NodeJS.Timeout | undefined;
  private heartbeatError: unknown;

  constructor(
    readonly path: string,
    readonly owner: LockOwner,
    heartbeat: boolean,
  ) {
    if (heartbeat) {
      this.heartbeat = setInterval(() => {
        try {
          const current = readLockOwner(path);
          if (!current || current.token !== owner.token) {
            throw new Error(`Storage lock ownership changed unexpectedly: ${path}`);
          }
          writeLockOwner(path, { ...owner, heartbeatAtMs: Date.now() });
        } catch (error) {
          this.heartbeatError ??= error;
        }
      }, LOCK_HEARTBEAT_MS);
      this.heartbeat.unref();
    }
  }

  assertHealthy(): void {
    if (this.heartbeatError) {
      throw new Error(`Storage lock heartbeat failed: ${this.path}`, {
        cause: this.heartbeatError,
      });
    }
  }

  assertOwned(): void {
    this.assertHealthy();
    const current = readLockOwner(this.path);
    if (
      !current ||
      current.hostname !== HOSTNAME ||
      current.pid !== process.pid ||
      current.processIdentity !== this.owner.processIdentity ||
      current.token !== this.owner.token
    ) {
      throw new Error(`Storage lock ownership changed unexpectedly: ${this.path}`);
    }
  }

  release(): void {
    if (this.heartbeat) clearInterval(this.heartbeat);
    const heartbeatError = this.heartbeatError;
    const current = readLockOwner(this.path);
    if (
      !current || current.hostname !== HOSTNAME || current.pid !== process.pid ||
      current.processIdentity !== this.owner.processIdentity || current.token !== this.owner.token
    ) {
      throw new Error(`Storage lock ownership changed unexpectedly: ${this.path}`);
    }
    rmSync(this.path, { recursive: true });
    syncDirectoryBestEffort(dirname(this.path));
    if (heartbeatError) {
      throw new Error(`Storage lock heartbeat failed: ${this.path}`, {
        cause: heartbeatError,
      });
    }
  }
}

const tryAcquireLock = (path: string, heartbeat: boolean): StorageLock | undefined => {
  ensurePrivateDirectory(dirname(path));
  try {
    mkdirSync(path, { mode: DIRECTORY_MODE });
  } catch (error) {
    if (isErrno(error, "EEXIST")) return undefined;
    throw error;
  }
  const now = Date.now();
  const owner: LockOwner = {
    schemaVersion: 4,
    hostname: HOSTNAME,
    pid: process.pid,
    processStartedAtMs: PROCESS_STARTED_AT_MS,
    ...(PROCESS_IDENTITY ? { processIdentity: PROCESS_IDENTITY } : {}),
    token: randomUUID(),
    createdAtMs: now,
    heartbeatAtMs: now,
  };
  try {
    writeLockOwner(path, owner);
    syncDirectoryBestEffort(dirname(path));
    return new StorageLock(path, owner, heartbeat);
  } catch (error) {
    rmSync(path, { recursive: true, force: true });
    throw error;
  }
};

const acquireLockSync = (path: string): StorageLock => {
  const deadline = Date.now() + LOCK_TIMEOUT_MS;
  while (true) {
    const lock = tryAcquireLock(path, false);
    if (lock) return lock;
    tryRecoverStaleLock(path);
    if (Date.now() >= deadline) throw new Error(`Timed out waiting for storage lock ${path}`);
    Atomics.wait(waitBuffer, 0, 0, LOCK_RETRY_MS);
  }
};

const acquireLock = async (path: string): Promise<StorageLock> => {
  const deadline = Date.now() + LOCK_TIMEOUT_MS;
  while (true) {
    const lock = tryAcquireLock(path, true);
    if (lock) return lock;
    tryRecoverStaleLock(path);
    if (Date.now() >= deadline) throw new Error(`Timed out waiting for storage lock ${path}`);
    await delay(LOCK_RETRY_MS);
  }
};

const withExclusiveLock = <T>(path: string, operation: () => T): T => {
  const lock = acquireLockSync(path);
  try {
    return operation();
  } finally {
    lock.release();
  }
};

const parseNonNegativeInteger = (value: unknown, label: string): bigint => {
  if (typeof value !== "string" || !/^(0|[1-9]\d*)$/.test(value)) {
    throw new Error(`${label} must be a canonical non-negative integer string`);
  }
  return BigInt(value);
};

const readCursorPosition = (path: string, initial: bigint): bigint => {
  let text: string;
  try {
    text = readRegularFile(path).toString("utf8");
  } catch (error) {
    if (isErrno(error, "ENOENT")) return initial;
    throw error;
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch (error) {
    throw new Error(`Corrupt cursor JSON at ${path}`, { cause: error });
  }
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    throw new Error(`Invalid cursor document at ${path}`);
  }
  const cursor = parsed as { schemaVersion?: unknown; next?: unknown };
  if (cursor.schemaVersion !== CURSOR_SCHEMA_VERSION) {
    throw new Error(
      `Unsupported cursor schemaVersion ${String(cursor.schemaVersion)} at ${path}; expected 2`,
    );
  }
  return parseNonNegativeInteger(cursor.next, `Cursor next value at ${path}`);
};

/** Crash-safe monotonic counter: a crash can skip a claim but cannot reuse it. */
export class Cursor {
  private constructor(
    private readonly path: string,
    private readonly initial: bigint,
  ) {}

  static open(path: string, initial = 0n): Cursor {
    if (initial < 0n) throw new Error("Cursor initial position cannot be negative");
    ensurePrivateDirectory(dirname(path));
    const cursor = new Cursor(path, initial);
    void cursor.position;
    return cursor;
  }

  get position(): bigint {
    return withExclusiveLock(`${this.path}.lock`, () =>
      readCursorPosition(this.path, this.initial),
    );
  }

  claim(count = 1): bigint {
    if (!Number.isSafeInteger(count) || count <= 0) {
      throw new Error("Cursor claim count must be a positive safe integer");
    }
    return withExclusiveLock(`${this.path}.lock`, () => {
      const claimed = readCursorPosition(this.path, this.initial);
      atomicWritePrivateFile(
        this.path,
        `${JSON.stringify({
          schemaVersion: CURSOR_SCHEMA_VERSION,
          next: (claimed + BigInt(count)).toString(),
        })}\n`,
      );
      return claimed;
    });
  }
}

const META_FILE = "meta.json";
const WRITER_LOCK = ".writer.lock";
const CREATE_OWNER_FILE = ".create-owner.json";
const SHA256_PATTERN = /^[0-9a-f]{64}$/;
const ITEMS_FILE_PATTERN = /^items-[0-9a-f]{64}\.jsonl$/;
const HEX = /^0x(?:[0-9a-fA-F]{2})*$/;
const HEX_32 = /^0x[0-9a-fA-F]{64}$/;
const RAW_HEX = /^(?:[0-9a-fA-F]{2})+$/;
const ADDRESS = /^0x[0-9a-fA-F]{40}$/;
const VALUE_TYPES = [
  "bool",
  "uint8",
  "uint16",
  "uint32",
  "uint64",
  "uint128",
  "uint256",
  "address",
] as const;
const NETWORKS = [
  "testnet",
  "testnet-amoy",
  "devnet",
  "devnet-amoy",
  "mainnet",
] as const;

const safeInteger = z.number().int().safe();
const nonNegative = safeInteger.nonnegative();
const ownerIndex = safeInteger.refine((value) => value >= 0 || value === -1, {
  message: "owner index must be non-negative or PRIVATE_KEY_LANE (-1)",
});
const delegateIndex = safeInteger.refine((value) => value >= 0 || value === -2, {
  message: "delegate index must be non-negative or DELEGATE_KEY_LANE (-2)",
});
const address = z.string().regex(ADDRESS);
const canonicalDate = z.string().refine((value) => {
  const parsed = new Date(value);
  return !Number.isNaN(parsed.valueOf()) && parsed.toISOString() === value;
}, "expected a canonical ISO-8601 timestamp");
const digestSchema = z.object({
  algorithm: z.literal("sha256"),
  value: z.string().regex(SHA256_PATTERN),
}).strict();
const persistedFields = {
  schemaVersion: z.literal(POOL_SCHEMA_VERSION),
  itemsFile: z.string().regex(ITEMS_FILE_PATTERN),
  itemsDigest: digestSchema,
};
const commonMetaFields = {
  network: z.enum(NETWORKS),
  contractChainId: nonNegative,
  contractAddress: address,
  createdAt: canonicalDate,
  count: nonNegative,
};
const inputProofMetaSchema = z
  .object({
    ...commonMetaFields,
    ...persistedFields,
    kind: z.literal("input-proof-payloads"),
    flow: z.literal("input-proof"),
    relayerUrl: z.url().optional(),
  })
  .strict();
const handleMetaBase = {
  ...commonMetaFields,
  ...persistedFields,
  kind: z.literal("fhe-handles"),
  ownerIndices: z.array(ownerIndex).min(1).refine(
    (indices) => new Set(indices).size === indices.length,
    "ownerIndices must be unique",
  ),
};
const delegationExpiration = z.string().regex(/^(0|[1-9]\d*)$/);
const ownerIndexKey = z.string().regex(/^(?:-1|0|[1-9]\d*)$/);
const handleMetaSchema = z.discriminatedUnion("flow", [
  z.object({ ...handleMetaBase, flow: z.literal("public-decrypt") }).strict(),
  z.object({ ...handleMetaBase, flow: z.literal("user-decrypt") }).strict(),
  z.object({
    ...handleMetaBase,
    flow: z.literal("delegated-user-decrypt"),
    delegateIndex,
    delegateAddress: address,
    delegationExpiration: delegationExpiration.optional(),
    delegationExpirations: z
      .record(ownerIndexKey, delegationExpiration)
      .optional(),
  }).strict(),
]);
const persistedMetaSchema = z.union([inputProofMetaSchema, handleMetaSchema]);

const valueSchema = z.object({
  type: z.enum(VALUE_TYPES),
  value: z.string(),
}).strict().superRefine(({ type, value }, context) => {
  if (type === "bool") {
    if (value !== "true" && value !== "false") {
      context.addIssue({ code: "custom", message: "bool values must be true or false" });
    }
    return;
  }
  if (type === "address") {
    if (!ADDRESS.test(value)) context.addIssue({ code: "custom", message: "invalid address value" });
    return;
  }
  if (!/^(0|[1-9]\d*)$/.test(value)) {
    context.addIssue({ code: "custom", message: `${type} values must be canonical non-negative integers` });
    return;
  }
  const bits = Number(type.slice("uint".length));
  if (BigInt(value) >= 1n << BigInt(bits)) {
    context.addIssue({ code: "custom", message: `${type} value is out of range` });
  }
});

const inputProofItemSchema = z
  .object({
    index: nonNegative,
    contractChainId: nonNegative,
    contractAddress: address,
    userAddress: address,
    ciphertextWithInputVerification: z.string().regex(RAW_HEX),
    extraData: z.string().regex(HEX),
    expectedHandles: z.array(z.string().regex(HEX_32)).min(1),
    values: z.array(valueSchema).min(1),
  })
  .strict()
  .refine((item) => item.expectedHandles.length === item.values.length, {
    message: "expectedHandles and values must have equal lengths",
  });
const handleItemSchema = z.object({
  index: nonNegative,
  type: z.enum(VALUE_TYPES),
  value: z.string(),
  handle: z.string().regex(HEX_32),
  ownerIndex,
  ownerAddress: address,
  isPublic: z.boolean(),
  transactionHash: z.string().regex(HEX_32),
}).strict().superRefine((item, context) => {
  const result = valueSchema.safeParse({ type: item.type, value: item.value });
  if (!result.success) {
    context.addIssue({ code: "custom", message: z.prettifyError(result.error), path: ["value"] });
  }
});

const parsePoolItem = (
  value: unknown,
  meta: PersistedPoolMeta,
  label: string,
): InputProofPoolItem | FheHandlePoolItem => {
  const schema =
    meta.kind === "input-proof-payloads" ? inputProofItemSchema : handleItemSchema;
  const result = schema.safeParse(value);
  if (!result.success) {
    throw new Error(`${label}: ${z.prettifyError(result.error)}`);
  }
  if (meta.kind === "input-proof-payloads") {
    const item = result.data as InputProofPoolItem;
    if (
      item.contractChainId !== meta.contractChainId ||
      item.contractAddress.toLowerCase() !== meta.contractAddress.toLowerCase()
    ) {
      throw new Error(`${label}: contract identity differs from pool metadata`);
    }
    return item;
  }

  const item = result.data as FheHandlePoolItem;
  if (item.isPublic !== (meta.flow === "public-decrypt")) {
    throw new Error(`${label}: isPublic conflicts with pool flow ${meta.flow}`);
  }
  if (!meta.ownerIndices?.includes(item.ownerIndex)) {
    throw new Error(`${label}: ownerIndex is not declared in pool metadata`);
  }
  return item;
};

const sha256 = (contents: Buffer): string =>
  createHash("sha256").update(contents).digest("hex");

const itemsPath = (dir: string, file: string): string => {
  if (!ITEMS_FILE_PATTERN.test(file) || basename(file) !== file) {
    throw new Error(`Invalid pool items filename: ${file}`);
  }
  const path = resolve(dir, file);
  if (dirname(path) !== resolve(dir)) throw new Error(`Pool items path escapes ${dir}`);
  return path;
};

const parsePoolMeta = (contents: Buffer | string, path: string): PersistedPoolMeta => {
  let parsed: unknown;
  try {
    parsed = JSON.parse(typeof contents === "string" ? contents : contents.toString("utf8"));
  } catch (error) {
    throw new Error(`Corrupt pool metadata JSON at ${path}`, { cause: error });
  }
  const result = persistedMetaSchema.safeParse(parsed);
  if (!result.success) {
    const version = typeof parsed === "object" && parsed !== null
      ? (parsed as { schemaVersion?: unknown }).schemaVersion
      : undefined;
    if (version !== POOL_SCHEMA_VERSION) {
      throw new Error(
        `Unsupported pool schemaVersion ${String(version)} at ${path}; expected 2`,
      );
    }
    throw new Error(`Invalid pool metadata at ${path}: ${z.prettifyError(result.error)}`);
  }
  return result.data as PersistedPoolMeta;
};

const parseItems = <Item>(
  contents: Buffer,
  path: string,
  meta: PersistedPoolMeta,
): Item[] => {
  if (contents.length === 0) return [];
  let text: string;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(contents);
  } catch (error) {
    throw new Error(`Pool items file is not valid UTF-8: ${path}`, { cause: error });
  }
  if (!text.endsWith("\n")) {
    throw new Error(`Pool items file has an incomplete final record: ${path}`);
  }
  const items = text.split("\n").filter((line) => line.length > 0).map((line, index) => {
    let parsed: unknown;
    try {
      parsed = JSON.parse(line);
    } catch (error) {
      throw new Error(
        `Pool items file has invalid JSON at record ${(index + 1).toString()}: ${path}`,
        { cause: error },
      );
    }
    return parsePoolItem(
      parsed,
      meta,
      `Invalid pool item at record ${(index + 1).toString()} in ${path}`,
    ) as Item;
  });
  const indices = items.map((item) => (item as { index: number }).index);
  const unique = new Set(indices);
  if (
    unique.size !== items.length ||
    indices.some((index) => index < 0 || index >= items.length) ||
    [...unique].some((_, index) => !unique.has(index))
  ) {
    throw new Error(
      `Pool item indices must be unique and contiguous from 0 through ${(items.length - 1).toString()}: ${path}`,
    );
  }
  return items;
};

const verifyItems = <Item>(
  dir: string,
  meta: PersistedPoolMeta,
): { bytes: Buffer; items: Item[] } => {
  const path = itemsPath(dir, meta.itemsFile);
  let bytes: Buffer;
  try {
    bytes = readRegularFile(path);
  } catch (error) {
    if (isErrno(error, "ENOENT")) {
      throw new Error(`Pool items file is missing: ${path}`, { cause: error });
    }
    throw error;
  }
  const actualDigest = sha256(bytes);
  if (actualDigest !== meta.itemsDigest.value) {
    throw new Error(
      `Pool items digest mismatch at ${path}: expected ${meta.itemsDigest.value}, got ${actualDigest}`,
    );
  }
  const items = parseItems<Item>(bytes, path, meta);
  if (items.length !== meta.count) {
    throw new Error(
      `Pool item count mismatch at ${path}: metadata says ${meta.count.toString()}, found ${items.length.toString()}`,
    );
  }
  return { bytes, items };
};

const readPersistedMeta = (dir: string): PersistedPoolMeta => {
  const path = join(dir, META_FILE);
  try {
    return parsePoolMeta(readRegularFile(path), path);
  } catch (error) {
    if (isErrno(error, "ENOENT")) {
      throw new Error(`Pool metadata file is missing: ${path}`, { cause: error });
    }
    throw error;
  }
};

const withoutPersistence = (meta: PersistedPoolMeta): PoolMeta => {
  const {
    schemaVersion: _schemaVersion,
    itemsFile: _itemsFile,
    itemsDigest: _itemsDigest,
    ...poolMeta
  } = meta;
  return poolMeta;
};

const assertStablePoolIdentity = (
  requested: PoolMeta,
  committed: PersistedPoolMeta,
): void => {
  for (const field of [
    "kind",
    "flow",
    "network",
    "contractChainId",
    "contractAddress",
    "createdAt",
    "relayerUrl",
  ] as const) {
    if (requested[field] !== committed[field]) {
      throw new Error(`Pool metadata identity field ${field} cannot change after creation`);
    }
  }
};

type CreateTempOwner = Readonly<{
  schemaVersion: 1 | 2;
  hostname: string;
  pid: number;
  processStartedAtMs: number;
  processIdentity?: string;
  token: string;
  createdAtMs: number;
}>;

const readCreateTempOwner = (dir: string): CreateTempOwner | undefined => {
  try {
    const value: unknown = JSON.parse(readRegularFile(join(dir, CREATE_OWNER_FILE)).toString("utf8"));
    if (typeof value !== "object" || value === null || Array.isArray(value)) return undefined;
    const owner = value as Partial<CreateTempOwner>;
    if (
      (owner.schemaVersion !== 1 && owner.schemaVersion !== 2) || typeof owner.hostname !== "string" ||
      !Number.isSafeInteger(owner.pid) || (owner.pid ?? 0) <= 0 ||
      !Number.isFinite(owner.processStartedAtMs) ||
      (owner.processIdentity !== undefined &&
        (typeof owner.processIdentity !== "string" || owner.processIdentity.length === 0)) ||
      typeof owner.token !== "string" ||
      owner.token.length === 0 || !Number.isFinite(owner.createdAtMs)
    ) return undefined;
    return owner as CreateTempOwner;
  } catch {
    return undefined;
  }
};

const scavengeCreateTemps = (parent: string, poolName: string): void => {
  const prefix = `.${poolName}.tmp-`;
  for (const entry of readdirSync(parent)) {
    if (!entry.startsWith(prefix)) continue;
    const path = join(parent, entry);
    const stat = lstatSync(path);
    if (stat.isSymbolicLink() || !stat.isDirectory()) continue;
    const owner = readCreateTempOwner(path);
    const alive = owner ? processIsAlive(owner.pid) : false;
    const liveIdentity = owner && alive ? verifiedProcessIdentity(owner.pid) : undefined;
    const stale = owner
      ? owner.hostname === HOSTNAME && (
          !alive ||
          (owner.processIdentity !== undefined &&
            liveIdentity !== undefined &&
            owner.processIdentity !== liveIdentity)
        )
      : Date.now() - stat.mtimeMs >= CREATE_TEMP_GRACE_MS;
    if (!stale) continue;
    rmSync(path, { recursive: true, force: true });
    syncDirectoryBestEffort(parent);
  }
};

const persistMetaPointer = (
  dir: string,
  meta: PoolMeta,
  file: string,
  digest: string,
  beforePublish?: () => void,
): PersistedPoolMeta => {
  const value = {
    ...meta,
    schemaVersion: POOL_SCHEMA_VERSION,
    itemsFile: file,
    itemsDigest: { algorithm: "sha256", value: digest },
  };
  const serialized = `${JSON.stringify(value, null, 2)}\n`;
  const validated = parsePoolMeta(serialized, join(dir, META_FILE));
  atomicWritePrivateFile(join(dir, META_FILE), serialized, beforePublish);
  return validated;
};

export interface PoolItemsWriter<Item> {
  /** First index reserved by the rebased writer transaction. */
  readonly startIndex: number;
  write(item: Item): Promise<void>;
  close(): Promise<void>;
}

class TransactionalItemsWriter<Item> implements PoolItemsWriter<Item> {
  private tail = Promise.resolve();
  private failure: unknown;
  private closed = false;
  private appended = 0;

  constructor(
    private readonly dir: string,
    private readonly tempPath: string,
    private readonly handle: Awaited<ReturnType<typeof openFile>>,
    private readonly lock: StorageLock,
    private readonly baseMeta: PersistedPoolMeta,
    private readonly onCommit: (meta: PersistedPoolMeta) => void,
  ) {}

  get startIndex(): number {
    return this.baseMeta.count;
  }

  write(item: Item): Promise<void> {
    if (this.closed) return Promise.reject(new Error("Cannot write to a closed pool writer"));
    const operation = this.tail.then(async () => {
      if (this.failure) throw this.failure;
      this.lock.assertHealthy();
      const parsed = parsePoolItem(item, this.baseMeta, "Invalid new pool item");
      await this.handle.writeFile(`${JSON.stringify(parsed)}\n`, "utf8");
      this.appended += 1;
    });
    this.tail = operation.catch((error: unknown) => {
      this.failure ??= error;
    });
    return operation;
  }

  async close(): Promise<void> {
    if (this.closed) return;
    this.closed = true;
    let failure: unknown;
    try {
      await this.tail;
      if (this.failure) throw this.failure;
      this.lock.assertHealthy();
      await this.handle.sync();
      await this.handle.close();

      const bytes = readRegularFile(this.tempPath);
      const parsed = parseItems<Item>(bytes, this.tempPath, this.baseMeta);
      const expectedCount = this.baseMeta.count + this.appended;
      if (parsed.length !== expectedCount) {
        throw new Error(
          `Pool writer record count mismatch: expected ${expectedCount.toString()}, found ${parsed.length.toString()}`,
        );
      }
      const digest = sha256(bytes);
      const file = `items-${digest}.jsonl`;
      const finalPath = itemsPath(this.dir, file);
      if (pathExistsNoFollow(finalPath)) {
        const existing = readRegularFile(finalPath);
        if (!existing.equals(bytes)) {
          throw new Error(`Digest collision for pool snapshot ${finalPath}`);
        }
        rmSync(this.tempPath);
      } else {
        this.lock.assertOwned();
        renameSync(this.tempPath, finalPath);
        syncDirectoryBestEffort(this.dir);
      }
      // Publication is fenced by the exact owner token. Stale writers never
      // move the sole metadata commit pointer after losing ownership.
      const committed = persistMetaPointer(
        this.dir,
        { ...withoutPersistence(this.baseMeta), count: expectedCount },
        file,
        digest,
        () => this.lock.assertOwned(),
      );
      this.onCommit(committed);
    } catch (error) {
      failure = error;
      try {
        await this.handle.close();
      } catch {
        // The successful path already closed it; preserve the primary failure.
      }
      rmSync(this.tempPath, { force: true });
    }
    try {
      this.lock.release();
    } catch (error) {
      failure = failure
        ? new AggregateError(
            [failure, error],
            failure instanceof Error ? failure.message : "Pool writer close failed",
          )
        : error;
    }
    if (failure) throw failure;
  }
}

/**
 * Schema-v2 pool storage. Item snapshots are immutable and digest-addressed;
 * the fsynced atomic `meta.json` replacement is the sole commit pointer.
 * SHA-256 detects accidental corruption only—it does not authenticate pools.
 */
export class PoolStore<Item> {
  private constructor(
    readonly dir: string,
    private currentMeta: PersistedPoolMeta,
  ) {}

  get meta(): PersistedPoolMeta {
    return this.currentMeta;
  }

  static async create<Item>(dir: string, meta: PoolMeta): Promise<PoolStore<Item>> {
    if (meta.count !== 0) {
      throw new Error("A newly created pool must have an item count of zero");
    }
    const absolute = resolve(dir);
    const parent = dirname(absolute);
    ensurePrivateDirectory(parent);
    scavengeCreateTemps(parent, basename(absolute));
    if (pathExistsNoFollow(absolute)) {
      throw new Error(`Refusing to create a pool over existing storage at ${absolute}`);
    }

    const temporary = join(
      parent,
      `.${basename(absolute)}.tmp-${process.pid.toString()}-${randomUUID()}`,
    );
    mkdirSync(temporary, { mode: DIRECTORY_MODE });
    try {
      atomicWritePrivateFile(join(temporary, CREATE_OWNER_FILE), `${JSON.stringify({
        schemaVersion: 2,
        hostname: HOSTNAME,
        pid: process.pid,
        processStartedAtMs: PROCESS_STARTED_AT_MS,
        ...(PROCESS_IDENTITY ? { processIdentity: PROCESS_IDENTITY } : {}),
        token: randomUUID(),
        createdAtMs: Date.now(),
      } satisfies CreateTempOwner)}\n`);
      const digest = sha256(Buffer.alloc(0));
      const file = `items-${digest}.jsonl`;
      const path = itemsPath(temporary, file);
      const fd = openSync(
        path,
        constants.O_CREAT | constants.O_EXCL | constants.O_WRONLY | NOFOLLOW,
        FILE_MODE,
      );
      fsyncSync(fd);
      closeSync(fd);
      const persisted = persistMetaPointer(temporary, meta, file, digest);
      syncDirectoryBestEffort(temporary);
      renameSync(temporary, absolute);
      rmSync(join(absolute, CREATE_OWNER_FILE), { force: true });
      syncDirectoryBestEffort(absolute);
      syncDirectoryBestEffort(parent);
      return new PoolStore<Item>(absolute, persisted);
    } catch (error) {
      rmSync(temporary, { recursive: true, force: true });
      throw error;
    }
  }

  static async open<Item>(dir: string): Promise<PoolStore<Item>> {
    const absolute = resolve(dir);
    try {
      assertDirectory(absolute, true);
    } catch (error) {
      if (isErrno(error, "ENOENT")) {
        throw new Error(`Pool directory is missing: ${absolute}`, { cause: error });
      }
      throw error;
    }
    rmSync(join(absolute, CREATE_OWNER_FILE), { force: true });
    const meta = readPersistedMeta(absolute);
    verifyItems<Item>(absolute, meta);
    return new PoolStore<Item>(absolute, meta);
  }

  static async openIfExists<Item>(dir: string): Promise<PoolStore<Item> | undefined> {
    try {
      lstatSync(dir);
    } catch (error) {
      if (isErrno(error, "ENOENT")) return undefined;
      throw error;
    }
    return PoolStore.open<Item>(dir);
  }

  async itemsWriter(
    updateMeta?: (latest: PoolMeta) => PoolMeta,
  ): Promise<PoolItemsWriter<Item>> {
    const lock = await acquireLock(join(this.dir, WRITER_LOCK));
    let handle: Awaited<ReturnType<typeof openFile>> | undefined;
    let tempPath: string | undefined;
    try {
      const latest = readPersistedMeta(this.dir);
      const { bytes } = verifyItems<Item>(this.dir, latest);
      let baseMeta = latest;
      if (updateMeta) {
        const requested = updateMeta(withoutPersistence(latest));
        if (requested.count !== latest.count) {
          throw new Error("Writer metadata update cannot change the committed item count");
        }
        assertStablePoolIdentity(requested, latest);
        baseMeta = parsePoolMeta(JSON.stringify({
          ...requested,
          schemaVersion: POOL_SCHEMA_VERSION,
          itemsFile: latest.itemsFile,
          itemsDigest: latest.itemsDigest,
        }), join(this.dir, META_FILE));
        parseItems<Item>(bytes, itemsPath(this.dir, latest.itemsFile), baseMeta);
      }
      for (const entry of readdirSync(this.dir)) {
        if (entry.startsWith(".items.tmp-")) {
          rmSync(join(this.dir, entry), { force: true });
        }
      }
      tempPath = join(this.dir, `.items.tmp-${process.pid.toString()}-${randomUUID()}`);
      handle = await openFile(
        tempPath,
        constants.O_CREAT | constants.O_EXCL | constants.O_WRONLY | NOFOLLOW,
        FILE_MODE,
      );
      await handle.writeFile(bytes);
      this.currentMeta = latest;
      return new TransactionalItemsWriter(
        this.dir,
        tempPath,
        handle,
        lock,
        baseMeta,
        (meta) => { this.currentMeta = meta; },
      );
    } catch (error) {
      if (handle) await handle.close();
      if (tempPath) rmSync(tempPath, { force: true });
      lock.release();
      throw error;
    }
  }

  async loadItems(): Promise<Item[]> {
    const latest = readPersistedMeta(this.dir);
    const { items } = verifyItems<Item>(this.dir, latest);
    this.currentMeta = latest;
    return items;
  }

  async saveMeta(meta: PoolMeta): Promise<void> {
    await this.updateMeta(() => meta);
  }

  /** Reloads and updates metadata while holding the writer lock. */
  async updateMeta(update: (latest: PoolMeta) => PoolMeta): Promise<void> {
    const lock = await acquireLock(join(this.dir, WRITER_LOCK));
    try {
      const latest = readPersistedMeta(this.dir);
      const { bytes } = verifyItems<Item>(this.dir, latest);
      const meta = update(withoutPersistence(latest));
      if (meta.count !== latest.count) {
        throw new Error(
          `Cannot save pool metadata count ${meta.count.toString()}; committed snapshot contains ${latest.count.toString()} record(s)`,
        );
      }
      assertStablePoolIdentity(meta, latest);
      const candidate = parsePoolMeta(
        JSON.stringify({
          ...meta,
          schemaVersion: POOL_SCHEMA_VERSION,
          itemsFile: latest.itemsFile,
          itemsDigest: latest.itemsDigest,
        }),
        join(this.dir, META_FILE),
      );
      parseItems<Item>(bytes, itemsPath(this.dir, latest.itemsFile), candidate);
      this.currentMeta = persistMetaPointer(
        this.dir,
        meta,
        latest.itemsFile,
        latest.itemsDigest.value,
        () => lock.assertOwned(),
      );
    } finally {
      lock.release();
    }
  }

  /** Writers commit their exact count; this compatibility method verifies it. */
  async updateCount(count: number): Promise<void> {
    const latest = readPersistedMeta(this.dir);
    if (count !== latest.count) {
      throw new Error(
        `Cannot set pool count to ${count.toString()}; committed snapshot contains ${latest.count.toString()} record(s)`,
      );
    }
    this.currentMeta = latest;
  }

  cursor(name: string): Cursor {
    if (!/^[A-Za-z0-9][A-Za-z0-9._-]*$/.test(name)) {
      throw new Error(`Invalid cursor name: ${name}`);
    }
    assertDirectory(this.dir);
    return Cursor.open(join(this.dir, `cursor-${name}.json`));
  }
}

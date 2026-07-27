import { execFile, spawn } from "node:child_process";
import { once } from "node:events";
import {
  chmod,
  lstat,
  mkdir,
  mkdtemp,
  readFile,
  readdir,
  rename,
  rm,
  stat,
  symlink,
  utimes,
  writeFile,
} from "node:fs/promises";
import { hostname, tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { promisify } from "node:util";
import { afterEach, beforeEach, describe, expect, it } from "vitest";

import { Cursor, PoolStore } from "../src/pool/store";
import type {
  FheHandlePoolItem,
  InputProofPoolItem,
  PoolMeta,
} from "../src/pool/types";

const execFileAsync = promisify(execFile);
const packageDir = join(dirname(fileURLToPath(import.meta.url)), "..");
const storeModuleUrl = pathToFileURL(
  join(packageDir, "src/pool/store.ts"),
).href;

let dir: string;

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-pool-"));
});

afterEach(async () => {
  await rm(dir, { recursive: true, force: true });
});

const meta: PoolMeta = {
  kind: "input-proof-payloads",
  flow: "input-proof",
  network: "testnet",
  contractChainId: 11_155_111,
  contractAddress: "0x0000000000000000000000000000000000000001",
  createdAt: "2026-01-01T00:00:00.000Z",
  count: 0,
};

const item = (index: number): InputProofPoolItem => ({
  index,
  contractChainId: meta.contractChainId,
  contractAddress: meta.contractAddress,
  userAddress: "0x0000000000000000000000000000000000000002",
  ciphertextWithInputVerification: `00${index.toString(16).padStart(2, "0")}`,
  extraData: "0x00",
  expectedHandles: [`0x${"00".repeat(32)}`],
  values: [{ type: "uint64", value: index.toString() }],
});

const handleMeta: PoolMeta = {
  kind: "fhe-handles",
  flow: "public-decrypt",
  network: "testnet",
  contractChainId: meta.contractChainId,
  contractAddress: meta.contractAddress,
  createdAt: meta.createdAt,
  count: 0,
  ownerIndices: [0],
};

const handleItem: FheHandlePoolItem = {
  index: 0,
  type: "uint64",
  value: "42",
  handle: `0x${"01".repeat(32)}`,
  ownerIndex: 0,
  ownerAddress: "0x0000000000000000000000000000000000000002",
  isPublic: true,
  transactionHash: `0x${"02".repeat(32)}`,
};

const delegatedMeta: PoolMeta = {
  ...handleMeta,
  flow: "delegated-user-decrypt",
  ownerIndices: [0],
  delegateIndex: -2,
  delegateAddress: "0x0000000000000000000000000000000000000003",
};

const delegatedItem = (index: number, ownerIndex: number): FheHandlePoolItem => ({
  ...handleItem,
  index,
  ownerIndex,
  ownerAddress: ownerIndex === -1
    ? "0x0000000000000000000000000000000000000004"
    : handleItem.ownerAddress,
  isPublic: false,
});

const committedItemsPath = async (poolDir: string): Promise<string> => {
  const persisted = JSON.parse(await readFile(join(poolDir, "meta.json"), "utf8")) as {
    itemsFile: string;
  };
  return join(poolDir, persisted.itemsFile);
};

const childClaims = async (path: string, count: number): Promise<number[]> => {
  const source = `
    const { Cursor } = await import(${JSON.stringify(storeModuleUrl)});
    const cursor = Cursor.open(${JSON.stringify(path)});
    const claims = Array.from({ length: ${count.toString()} }, () => Number(cursor.claim()));
    process.stdout.write(JSON.stringify(claims));
  `;
  const { stdout } = await execFileAsync(
    process.execPath,
    ["--import", "tsx", "--input-type=module", "--eval", source],
    { cwd: packageDir },
  );
  return JSON.parse(stdout) as number[];
};

describe("Cursor", () => {
  it("claims monotonically and persists across reopen", () => {
    const path = join(dir, "cursor.json");
    const cursor = Cursor.open(path);
    expect(cursor.claim()).toBe(0n);
    expect(cursor.claim(3)).toBe(1n);
    expect(cursor.position).toBe(4n);

    const reopened = Cursor.open(path);
    expect(reopened.position).toBe(4n);
    expect(reopened.claim()).toBe(4n);
  });

  it("never reuses a durable claim after a crash and ignores orphan temps", async () => {
    const path = join(dir, "cursor.json");
    const claimed = Cursor.open(path).claim();
    await writeFile(`${path}.tmp-crashed-process`, "partial");

    const reopened = Cursor.open(path);
    expect(reopened.claim()).toBeGreaterThan(claimed);
  });

  it("fails closed for corrupt, legacy, and invalid cursor state", async () => {
    const path = join(dir, "cursor.json");
    await writeFile(path, "{not-json", { mode: 0o600 });
    expect(() => Cursor.open(path)).toThrow(/Corrupt cursor JSON/);

    await writeFile(path, JSON.stringify({ schemaVersion: 1, next: "9" }));
    expect(() => Cursor.open(path)).toThrow(/expected 2/);

    await writeFile(path, JSON.stringify({ schemaVersion: 2, next: "-1" }));
    expect(() => Cursor.open(path)).toThrow(/canonical non-negative/);
  });

  it("recovers a lock left by a dead process", async () => {
    const path = join(dir, "cursor.json");
    await mkdir(`${path}.lock`, { mode: 0o700 });
    await writeFile(
      join(`${path}.lock`, "owner.json"),
      `${JSON.stringify({
        schemaVersion: 3,
        hostname: hostname(),
        pid: 2_147_483_647,
        processStartedAtMs: Date.now() - 1_000,
        token: "dead",
        createdAtMs: Date.now(),
        heartbeatAtMs: Date.now(),
      })}\n`,
      { mode: 0o600 },
    );
    expect(Cursor.open(path).claim()).toBe(0n);
  });

  it.runIf(process.platform === "linux")(
    "recovers a local lock whose live PID has a different verified incarnation",
    async () => {
      const path = join(dir, "cursor.json");
      await mkdir(`${path}.lock`, { mode: 0o700 });
      await writeFile(join(`${path}.lock`, "owner.json"), `${JSON.stringify({
        schemaVersion: 4,
        hostname: hostname(),
        pid: process.ppid,
        processStartedAtMs: Date.now() - 60_000,
        processIdentity: "verified-incarnation-that-does-not-match",
        token: "reused-pid",
        createdAtMs: Date.now() - 60_000,
        heartbeatAtMs: Date.now() - 60_000,
      })}\n`);
      expect(Cursor.open(path).claim()).toBe(0n);
    },
  );

  it("recovers an abandoned, malformed lock directory after its grace period", async () => {
    const path = join(dir, "cursor.json");
    await mkdir(`${path}.lock`, { mode: 0o700 });
    const old = new Date(Date.now() - 5_000);
    await utimes(`${path}.lock`, old, old);
    expect(Cursor.open(path).claim()).toBe(0n);
  });

  it("never reaps a writer lock owned by another host", async () => {
    const path = join(dir, "cursor.json");
    await mkdir(`${path}.lock`, { mode: 0o700 });
    await writeFile(join(`${path}.lock`, "owner.json"), `${JSON.stringify({
      schemaVersion: 3,
      hostname: `${hostname()}-foreign`,
      pid: 2_147_483_647,
      processStartedAtMs: Date.now() - 60_000,
      token: "foreign",
      createdAtMs: Date.now() - 60_000,
      heartbeatAtMs: Date.now() - 60_000,
    })}\n`);
    const source = `
      const { Cursor } = await import(${JSON.stringify(storeModuleUrl)});
      process.stdout.write("started\\n");
      Cursor.open(${JSON.stringify(path)}).claim();
      process.stdout.write("acquired\\n");
    `;
    const child = spawn(
      process.execPath,
      ["--import", "tsx", "--input-type=module", "--eval", source],
      { cwd: packageDir, stdio: ["ignore", "pipe", "pipe"] },
    );
    let output = "";
    try {
      await new Promise<void>((resolveStarted, rejectStarted) => {
        const timeout = setTimeout(() => rejectStarted(new Error("foreign-lock child did not start")), 2_000);
        child.once("error", rejectStarted);
        child.stdout.on("data", (chunk: Buffer) => {
          output += chunk.toString("utf8");
          if (output.includes("started")) {
            clearTimeout(timeout);
            resolveStarted();
          }
        });
      });
      await new Promise((resolve) => setTimeout(resolve, 200));
      expect(output).not.toContain("acquired");
    } finally {
      if (child.exitCode === null) {
        child.kill("SIGKILL");
        await once(child, "exit");
      }
    }
  });

  it("serializes claims across contending processes without duplicates", async () => {
    const path = join(dir, "cursor.json");
    const claims = (
      await Promise.all(
        Array.from({ length: 6 }, () => childClaims(path, 25)),
      )
    ).flat();

    expect(claims).toHaveLength(150);
    expect(new Set(claims).size).toBe(150);
    expect([...claims].sort((a, b) => a - b)).toEqual(
      Array.from({ length: 150 }, (_, index) => index),
    );
    expect(Cursor.open(path).position).toBe(150n);
  }, 20_000);

  it.runIf(process.platform !== "win32")("writes private cursor files", async () => {
    const path = join(dir, "cursor.json");
    Cursor.open(path).claim();
    expect((await stat(path)).mode & 0o777).toBe(0o600);
    expect((await stat(dir)).mode & 0o777).toBe(0o700);
  });
});

describe("PoolStore", () => {
  it("round-trips strict v2 metadata and an immutable committed snapshot", async () => {
    const poolDir = join(dir, "pool");
    const store = await PoolStore.create<InputProofPoolItem>(poolDir, meta);
    const writer = await store.itemsWriter();
    await writer.write(item(0));
    await writer.write(item(1));
    await writer.close();
    await store.updateCount(2);

    const reopened = await PoolStore.open<InputProofPoolItem>(poolDir);
    expect(reopened.meta).toMatchObject({
      schemaVersion: 2,
      count: 2,
      itemsFile: expect.stringMatching(/^items-[0-9a-f]{64}\.jsonl$/),
      itemsDigest: { algorithm: "sha256", value: expect.stringMatching(/^[0-9a-f]{64}$/) },
    });
    expect(await reopened.loadItems()).toEqual([item(0), item(1)]);
  });

  it("enforces the fhe-handle metadata and item branch", async () => {
    const poolDir = join(dir, "handles");
    const store = await PoolStore.create<FheHandlePoolItem>(poolDir, handleMeta);
    const writer = await store.itemsWriter();
    await writer.write(handleItem);
    await writer.close();
    expect(await store.loadItems()).toEqual([handleItem]);
    await expect(
      store.saveMeta({ ...handleMeta, count: 1, ownerIndices: [1] }),
    ).rejects.toThrow(/ownerIndex is not declared/);
  });

  it("serializes synchronized writers and rebases the contender on the latest commit", async () => {
    const poolDir = join(dir, "pool");
    const store = await PoolStore.create<InputProofPoolItem>(poolDir, meta);
    const secondStore = await PoolStore.open<InputProofPoolItem>(poolDir);
    let writer = await store.itemsWriter();
    const waitingWriter = secondStore.itemsWriter();
    await writer.write(item(0));
    await writer.close();

    writer = await waitingWriter;
    await writer.write(item(1));
    await writer.close();

    expect(await store.loadItems()).toEqual([item(0), item(1)]);
  });

  it("rejects stale producer indices before publishing a new snapshot", async () => {
    const poolDir = join(dir, "pool");
    const store = await PoolStore.create<InputProofPoolItem>(poolDir, meta);
    const first = await store.itemsWriter();
    await first.write(item(0));
    await first.close();

    const staleProducer = await store.itemsWriter();
    expect(staleProducer.startIndex).toBe(1);
    await staleProducer.write(item(0));
    await expect(staleProducer.close()).rejects.toThrow(/unique and contiguous/);
    expect(await store.loadItems()).toEqual([item(0)]);
  });

  it("allows concurrent production order while enforcing contiguous indices", async () => {
    const store = await PoolStore.create<InputProofPoolItem>(join(dir, "pool"), meta);
    const writer = await store.itemsWriter();
    await writer.write(item(1));
    await writer.write(item(0));
    await writer.close();
    expect((await store.loadItems()).map((entry) => entry.index).sort()).toEqual([0, 1]);
  });

  it("fences a writer whose owner token changed before publication", async () => {
    const poolDir = join(dir, "pool");
    const store = await PoolStore.create<InputProofPoolItem>(poolDir, meta);
    const writer = await store.itemsWriter();
    await writer.write(item(0));
    const ownerPath = join(poolDir, ".writer.lock", "owner.json");
    const owner = JSON.parse(await readFile(ownerPath, "utf8")) as Record<string, unknown>;
    await writeFile(ownerPath, `${JSON.stringify({ ...owner, token: "replacement" })}\n`);
    await expect(writer.close()).rejects.toThrow(/ownership changed/);
    expect((await PoolStore.open<InputProofPoolItem>(poolDir)).meta.count).toBe(0);
  });

  it("retains a reader-observed immutable snapshot across publication", async () => {
    const poolDir = join(dir, "pool");
    const store = await PoolStore.create<InputProofPoolItem>(poolDir, meta);
    const oldSnapshot = await committedItemsPath(poolDir);
    const writer = await store.itemsWriter();
    await writer.write(item(0));
    await writer.close();
    expect(await readFile(oldSnapshot, "utf8")).toBe("");
    expect(await store.loadItems()).toEqual([item(0)]);
  });

  it("rejects legacy or corrupt metadata and partial pool directories", async () => {
    const poolDir = join(dir, "pool");
    await PoolStore.create(poolDir, meta);
    const metaPath = join(poolDir, "meta.json");

    await writeFile(metaPath, JSON.stringify({ ...meta, schemaVersion: 1 }));
    await expect(PoolStore.open(poolDir)).rejects.toThrow(/expected 2/);

    await writeFile(metaPath, "{broken");
    await expect(PoolStore.open(poolDir)).rejects.toThrow(/Corrupt pool metadata/);

    const partialDir = join(dir, "partial");
    await mkdir(partialDir);
    await expect(PoolStore.openIfExists(partialDir)).rejects.toThrow(/metadata file is missing/);
  });

  it("rejects unknown metadata and item fields at the runtime boundary", async () => {
    const poolDir = join(dir, "pool");
    const store = await PoolStore.create<InputProofPoolItem>(poolDir, meta);
    const writer = await store.itemsWriter();
    await expect(writer.write({ ...item(0), surprise: true } as InputProofPoolItem)).rejects.toThrow();
    await expect(writer.write(item(0))).rejects.toThrow();
    await expect(writer.close()).rejects.toThrow();

    const persisted = JSON.parse(await readFile(join(poolDir, "meta.json"), "utf8")) as object;
    await writeFile(join(poolDir, "meta.json"), JSON.stringify({ ...persisted, surprise: true }));
    await expect(PoolStore.open(poolDir)).rejects.toThrow(/Invalid pool metadata/);
  });

  it("supports reserved lane sentinels and rejects other invalid lane identities", async () => {
    const sentinelStore = await PoolStore.create<FheHandlePoolItem>(join(dir, "sentinels"), {
      ...delegatedMeta,
      ownerIndices: [-1],
      delegationExpirations: { "-1": "4102444800" },
      delegationExpiration: "4102444800",
    });
    const sentinelWriter = await sentinelStore.itemsWriter();
    await sentinelWriter.write(delegatedItem(0, -1));
    await sentinelWriter.close();

    await expect(PoolStore.create(join(dir, "negative"), {
      ...handleMeta, ownerIndices: [-3],
    })).rejects.toThrow();
    await expect(PoolStore.create(join(dir, "duplicate"), {
      ...handleMeta, ownerIndices: [0, 0],
    })).rejects.toThrow(/unique/);
    await expect(PoolStore.create(join(dir, "delegate"), {
      ...handleMeta,
      flow: "delegated-user-decrypt",
      delegateIndex: -1,
      delegateAddress: "0x0000000000000000000000000000000000000003",
    })).rejects.toThrow();
    await expect(PoolStore.create(join(dir, "expiration-key"), {
      ...delegatedMeta,
      delegationExpirations: { "-2": "4102444800" },
    })).rejects.toThrow();

    const store = await PoolStore.create<FheHandlePoolItem>(join(dir, "handles"), handleMeta);
    const writer = await store.itemsWriter();
    await expect(writer.write({ ...handleItem, handle: "0x00" })).rejects.toThrow();
    await expect(writer.close()).rejects.toThrow();

    const inputStore = await PoolStore.create<InputProofPoolItem>(join(dir, "input"), meta);
    const inputWriter = await inputStore.itemsWriter();
    await expect(inputWriter.write({
      ...item(0),
      values: [{ type: "uint8", value: "256" }],
    })).rejects.toThrow(/out of range/);
    await expect(inputWriter.close()).rejects.toThrow();
  });

  it("detects snapshot corruption on open and on an already-open store", async () => {
    const poolDir = join(dir, "pool");
    const store = await PoolStore.create<InputProofPoolItem>(poolDir, meta);
    const writer = await store.itemsWriter();
    await writer.write(item(0));
    await writer.close();

    await writeFile(await committedItemsPath(poolDir), `${JSON.stringify(item(999))}\n`, { flag: "a" });
    await expect(store.loadItems()).rejects.toThrow(/digest mismatch/);
    await expect(PoolStore.open(poolDir)).rejects.toThrow(/digest mismatch/);
  });

  it("atomically merges delegated producer metadata with each item snapshot", async () => {
    const poolDir = join(dir, "delegated");
    const firstStore = await PoolStore.create<FheHandlePoolItem>(poolDir, delegatedMeta);
    const secondStore = await PoolStore.open<FheHandlePoolItem>(poolDir);
    const firstWriter = await firstStore.itemsWriter((latest) => ({
      ...latest,
      delegationExpirations: { ...(latest.delegationExpirations ?? {}), "0": "4102444800" },
      delegationExpiration: "4102444800",
    }));
    const waitingWriter = secondStore.itemsWriter((latest) => ({
      ...latest,
      ownerIndices: [...new Set([...(latest.ownerIndices ?? []), -1])],
      delegationExpirations: { ...(latest.delegationExpirations ?? {}), "-1": "4102444900" },
      delegationExpiration: "4102444800",
    }));

    await firstWriter.write(delegatedItem(0, 0));
    await firstWriter.close();
    expect(firstStore.meta.delegationExpirations).toEqual({ "0": "4102444800" });

    const secondWriter = await waitingWriter;
    expect(secondWriter.startIndex).toBe(1);
    await secondWriter.write(delegatedItem(1, -1));
    await secondWriter.close();

    const reopened = await PoolStore.open<FheHandlePoolItem>(poolDir);
    expect(reopened.meta.delegationExpirations).toEqual({
      "0": "4102444800",
      "-1": "4102444900",
    });
    expect(await reopened.loadItems()).toHaveLength(2);
  });

  it("does not publish delegated metadata when its writer process is killed", async () => {
    const poolDir = join(dir, "delegated-crash");
    await PoolStore.create<FheHandlePoolItem>(poolDir, delegatedMeta);
    const source = `
      const { PoolStore } = await import(${JSON.stringify(storeModuleUrl)});
      const store = await PoolStore.open(${JSON.stringify(poolDir)});
      const writer = await store.itemsWriter((latest) => ({
        ...latest,
        delegationExpirations: { ...(latest.delegationExpirations ?? {}), "0": "4102444800" },
        delegationExpiration: "4102444800",
      }));
      await writer.write(${JSON.stringify(delegatedItem(0, 0))});
      process.stdout.write("ready\\n");
      setInterval(() => {}, 1000);
    `;
    const child = spawn(
      process.execPath,
      ["--import", "tsx", "--input-type=module", "--eval", source],
      { cwd: packageDir, stdio: ["ignore", "pipe", "pipe"] },
    );
    await new Promise<void>((resolveReady, rejectReady) => {
      child.once("error", rejectReady);
      child.stdout.once("data", () => resolveReady());
    });
    child.kill("SIGKILL");
    await once(child, "exit");

    const recovered = await PoolStore.open<FheHandlePoolItem>(poolDir);
    expect(recovered.meta.count).toBe(0);
    expect(recovered.meta.delegationExpirations).toBeUndefined();
  });

  it("keeps the previous commit readable when a writer process is killed", async () => {
    const poolDir = join(dir, "pool");
    await PoolStore.create<InputProofPoolItem>(poolDir, meta);
    const source = `
      const { PoolStore } = await import(${JSON.stringify(storeModuleUrl)});
      const store = await PoolStore.open(${JSON.stringify(poolDir)});
      const writer = await store.itemsWriter();
      await writer.write(${JSON.stringify(item(0))});
      process.stdout.write("ready\\n");
      setInterval(() => {}, 1000);
    `;
    const child = spawn(
      process.execPath,
      ["--import", "tsx", "--input-type=module", "--eval", source],
      { cwd: packageDir, stdio: ["ignore", "pipe", "pipe"] },
    );
    await new Promise<void>((resolveReady, rejectReady) => {
      child.once("error", rejectReady);
      child.stdout.once("data", () => resolveReady());
    });
    child.kill("SIGKILL");
    await once(child, "exit");

    const recovered = await PoolStore.open<InputProofPoolItem>(poolDir);
    expect(await recovered.loadItems()).toEqual([]);
    const writer = await recovered.itemsWriter();
    expect((await readdir(poolDir)).filter((entry) => entry.startsWith(".items.tmp-"))).toHaveLength(1);
    await writer.write(item(0));
    await writer.close();
    expect(await recovered.loadItems()).toEqual([item(0)]);
  });

  it.runIf(process.platform !== "win32")("writes private pool files and repairs overly broad modes on open", async () => {
    const poolDir = join(dir, "pool");
    await PoolStore.create(poolDir, meta);
    const items = await committedItemsPath(poolDir);
    await chmod(poolDir, 0o755);
    await chmod(join(poolDir, "meta.json"), 0o644);
    await chmod(items, 0o644);

    await PoolStore.open(poolDir);
    expect((await stat(poolDir)).mode & 0o777).toBe(0o700);
    expect((await stat(join(poolDir, "meta.json"))).mode & 0o777).toBe(0o600);
    expect((await stat(items)).mode & 0o777).toBe(0o600);
  });

  it.runIf(process.platform !== "win32")("refuses symlinked pool paths and snapshot files", async () => {
    const realPool = join(dir, "real-pool");
    await PoolStore.create(realPool, meta);
    const poolLink = join(dir, "pool-link");
    await symlink(realPool, poolLink, "dir");
    await expect(PoolStore.open(poolLink)).rejects.toThrow(/refusing symlink/);

    const items = await committedItemsPath(realPool);
    const target = join(dir, "snapshot-copy.jsonl");
    await rename(items, target);
    await symlink(target, items);
    expect((await lstat(items)).isSymbolicLink()).toBe(true);
    await expect(PoolStore.open(realPool)).rejects.toThrow();
  });

  it("creates through a sibling temp directory and never leaves an invalid final pool", async () => {
    const poolDir = join(dir, "pool");
    await expect(PoolStore.create(poolDir, { ...meta, flow: "public-decrypt" })).rejects.toThrow(/Invalid pool metadata/);
    await expect(lstat(poolDir)).rejects.toMatchObject({ code: "ENOENT" });
    expect((await readdir(dir)).some((entry) => entry.startsWith(".pool.tmp-"))).toBe(false);
  });

  it("serializes concurrent creates without exposing a partial final pool", async () => {
    const poolDir = join(dir, "pool");
    const results = await Promise.allSettled([
      PoolStore.create(poolDir, meta),
      PoolStore.create(poolDir, meta),
    ]);
    expect(results.filter((result) => result.status === "fulfilled")).toHaveLength(1);
    expect(results.filter((result) => result.status === "rejected")).toHaveLength(1);
    expect((await PoolStore.open(poolDir)).meta.count).toBe(0);
  });

  it("scavenges a create temp owned by a dead local process", async () => {
    const poolDir = join(dir, "pool");
    const orphan = join(dir, ".pool.tmp-dead-owner");
    await mkdir(orphan);
    await writeFile(join(orphan, ".create-owner.json"), `${JSON.stringify({
      schemaVersion: 1,
      hostname: hostname(),
      pid: 2_147_483_647,
      processStartedAtMs: Date.now() - 60_000,
      token: "dead",
      createdAtMs: Date.now() - 60_000,
    })}\n`);
    await PoolStore.create(poolDir, meta);
    await expect(lstat(orphan)).rejects.toMatchObject({ code: "ENOENT" });
  });

  it("treats an empty create-temp process identity as malformed and honors the grace period", async () => {
    const poolDir = join(dir, "pool");
    const orphan = join(dir, ".pool.tmp-empty-identity");
    await mkdir(orphan);
    await writeFile(join(orphan, ".create-owner.json"), `${JSON.stringify({
      schemaVersion: 2,
      hostname: hostname(),
      pid: process.pid,
      processStartedAtMs: Date.now() - 60_000,
      processIdentity: "",
      token: "malformed",
      createdAtMs: Date.now() - 60_000,
    })}\n`);

    await PoolStore.create(poolDir, meta);
    expect((await lstat(orphan)).isDirectory()).toBe(true);

    await rm(poolDir, { recursive: true });
    const old = new Date(Date.now() - 31_000);
    await utimes(orphan, old, old);
    await PoolStore.create(poolDir, meta);
    await expect(lstat(orphan)).rejects.toMatchObject({ code: "ENOENT" });
  });

  it.runIf(process.platform === "linux")(
    "scavenges a create temp whose live PID has a different verified incarnation",
    async () => {
      const poolDir = join(dir, "pool");
      const orphan = join(dir, ".pool.tmp-reused-pid");
      await mkdir(orphan);
      await writeFile(join(orphan, ".create-owner.json"), `${JSON.stringify({
        schemaVersion: 2,
        hostname: hostname(),
        pid: process.ppid,
        processStartedAtMs: Date.now() - 60_000,
        processIdentity: "verified-incarnation-that-does-not-match",
        token: "reused-pid",
        createdAtMs: Date.now() - 60_000,
      })}\n`);
      await PoolStore.create(poolDir, meta);
      await expect(lstat(orphan)).rejects.toMatchObject({ code: "ENOENT" });
    },
  );

  it("never scavenges a create temp owned by another host", async () => {
    const poolDir = join(dir, "pool");
    const foreign = join(dir, ".pool.tmp-foreign-owner");
    await mkdir(foreign);
    await writeFile(join(foreign, ".create-owner.json"), `${JSON.stringify({
      schemaVersion: 1,
      hostname: `${hostname()}-foreign`,
      pid: 2_147_483_647,
      processStartedAtMs: Date.now() - 60_000,
      token: "foreign",
      createdAtMs: Date.now() - 60_000,
    })}\n`);
    await PoolStore.create(poolDir, meta);
    expect((await lstat(foreign)).isDirectory()).toBe(true);
  });

  it("returns undefined only for a genuinely missing pool", async () => {
    expect(await PoolStore.openIfExists(join(dir, "missing"))).toBeUndefined();
  });

  it("scopes and validates named cursors", async () => {
    const store = await PoolStore.create<InputProofPoolItem>(join(dir, "pool"), meta);
    const submit = store.cursor("submit");
    submit.claim(5);
    expect(store.cursor("submit").position).toBe(5n);
    expect(store.cursor("combos-k2").position).toBe(0n);
    expect(() => store.cursor("../escape")).toThrow(/Invalid cursor name/);
  });
});

import { describe, expect, it, beforeEach, afterEach } from "bun:test";
import { tmpdir } from "os";
import { join } from "path";
import { mkdtempSync, writeFileSync, readFileSync, rmSync } from "fs";
import { patchEnvVar, resolveVersions, VERSION_DEFAULTS } from "../env.js";

describe("resolveVersions", () => {
  const savedEnv: Record<string, string | undefined> = {};

  beforeEach(() => {
    // Save and clear all version env vars
    for (const key of Object.keys(VERSION_DEFAULTS)) {
      savedEnv[key] = process.env[key];
      delete process.env[key];
    }
  });

  afterEach(() => {
    // Restore
    for (const [key, val] of Object.entries(savedEnv)) {
      if (val === undefined) {
        delete process.env[key];
      } else {
        process.env[key] = val;
      }
    }
  });

  it("returns all expected keys", () => {
    const versions = resolveVersions();
    for (const key of Object.keys(VERSION_DEFAULTS)) {
      expect(versions).toHaveProperty(key);
    }
  });

  it("falls back to defaults when env vars are unset", () => {
    const versions = resolveVersions();
    for (const [key, defaultVal] of Object.entries(VERSION_DEFAULTS)) {
      expect(versions[key]).toBe(defaultVal);
    }
  });

  it("prefers env var over default", () => {
    process.env.CORE_VERSION = "v99.0.0";
    process.env.GATEWAY_VERSION = "v88.0.0";

    const versions = resolveVersions();
    expect(versions.CORE_VERSION).toBe("v99.0.0");
    expect(versions.GATEWAY_VERSION).toBe("v88.0.0");
    // Others should still be defaults
    expect(versions.HOST_VERSION).toBe(VERSION_DEFAULTS.HOST_VERSION);
  });

  it("returns exactly the same keys as VERSION_DEFAULTS", () => {
    const versions = resolveVersions();
    const versionKeys = Object.keys(versions).sort();
    const defaultKeys = Object.keys(VERSION_DEFAULTS).sort();
    expect(versionKeys).toEqual(defaultKeys);
  });
});

describe("patchEnvVar", () => {
  let tmpDir: string;

  beforeEach(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "fhevm-test-"));
  });

  afterEach(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  it("replaces an existing key", async () => {
    const file = join(tmpDir, ".env.test");
    writeFileSync(file, "FOO=old\nBAR=keep\n");

    await patchEnvVar(file, "FOO", "new");

    const content = readFileSync(file, "utf-8");
    expect(content).toContain("FOO=new");
    expect(content).toContain("BAR=keep");
    expect(content).not.toContain("FOO=old");
  });

  it("appends a missing key", async () => {
    const file = join(tmpDir, ".env.test");
    writeFileSync(file, "EXISTING=value\n");

    await patchEnvVar(file, "NEW_KEY", "new_value");

    const content = readFileSync(file, "utf-8");
    expect(content).toContain("EXISTING=value");
    expect(content).toContain("NEW_KEY=new_value");
  });

  it("is idempotent — patching twice with same value produces same result", async () => {
    const file = join(tmpDir, ".env.test");
    writeFileSync(file, "KEY=original\nOTHER=stuff\n");

    await patchEnvVar(file, "KEY", "patched");
    const after1 = readFileSync(file, "utf-8");

    await patchEnvVar(file, "KEY", "patched");
    const after2 = readFileSync(file, "utf-8");

    expect(after1).toBe(after2);
  });

  it("handles values with special characters", async () => {
    const file = join(tmpDir, ".env.test");
    writeFileSync(file, "URL=http://old:9000\n");

    await patchEnvVar(file, "URL", "http://172.17.0.2:9000");

    const content = readFileSync(file, "utf-8");
    expect(content).toContain("URL=http://172.17.0.2:9000");
  });

  it("handles values with equals signs", async () => {
    const file = join(tmpDir, ".env.test");
    writeFileSync(file, "CACHE=type=local,src=/tmp\n");

    await patchEnvVar(file, "CACHE", "type=local,src=/new");

    const content = readFileSync(file, "utf-8");
    expect(content).toContain("CACHE=type=local,src=/new");
  });

  it("preserves other lines exactly", async () => {
    const file = join(tmpDir, ".env.test");
    const original = "# comment\nA=1\nB=2\nC=3\n";
    writeFileSync(file, original);

    await patchEnvVar(file, "B", "updated");

    const content = readFileSync(file, "utf-8");
    expect(content).toContain("# comment");
    expect(content).toContain("A=1");
    expect(content).toContain("B=updated");
    expect(content).toContain("C=3");
  });

  it("round-trips: patch then read back yields the value", async () => {
    const file = join(tmpDir, ".env.test");
    writeFileSync(file, "TARGET=before\n");

    await patchEnvVar(file, "TARGET", "0xDeadBeef1234567890abcdef1234567890AbCdEf");

    const content = readFileSync(file, "utf-8");
    const match = content.match(/^TARGET=(.*)$/m);
    expect(match).not.toBeNull();
    expect(match![1]).toBe("0xDeadBeef1234567890abcdef1234567890AbCdEf");
  });
});

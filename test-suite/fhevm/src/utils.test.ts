import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, describe, expect, test } from "bun:test";

import {
  mergeArgs,
  needsQuotes,
  normalizeRepository,
  parseEnv,
  toServiceName,
  uint256ToId,
  withHexPrefix,
  writeEnvFile,
} from "./utils";

const tempDirs: string[] = [];

afterEach(async () => {
  await Promise.all(tempDirs.splice(0).map((dir) => fs.rm(dir, { recursive: true, force: true })));
});

describe("parseEnv", () => {
  test("parses simple KEY=VALUE pairs", () => {
    expect(parseEnv("FOO=bar\nBAZ=qux")).toEqual({ FOO: "bar", BAZ: "qux" });
  });

  test("ignores comments and blank lines", () => {
    expect(parseEnv("# comment\n\nFOO=bar\n  # indented\n")).toEqual({ FOO: "bar" });
  });

  test("strips surrounding quotes", () => {
    expect(parseEnv(`A="hello"\nB='world'`)).toEqual({ A: "hello", B: "world" });
  });

  test("handles values containing equals signs", () => {
    expect(parseEnv("URL=http://host:8545?chain=1")).toEqual({ URL: "http://host:8545?chain=1" });
  });

  test("handles empty values", () => {
    expect(parseEnv("EMPTY=\nSET=1")).toEqual({ EMPTY: "", SET: "1" });
  });

  test("skips lines without equals", () => {
    expect(parseEnv("NOPE\nOK=1")).toEqual({ OK: "1" });
  });

  test("handles Windows line endings", () => {
    expect(parseEnv("A=1\r\nB=2\r\n")).toEqual({ A: "1", B: "2" });
  });
});

describe("normalizeRepository", () => {
  test("rewrites hub.zama.org ghcr paths", () => {
    expect(normalizeRepository("hub.zama.org/ghcr/zama-ai/fhevm/coprocessor/host-listener")).toBe(
      "ghcr.io/zama-ai/fhevm/coprocessor/host-listener",
    );
  });

  test("rewrites hub.zama.org internal paths", () => {
    expect(normalizeRepository("hub.zama.org/internal/kms/core-service-enclave")).toBe(
      "ghcr.io/kms/core-service-enclave",
    );
  });

  test("rewrites hub.zama.org zama-protocol paths", () => {
    expect(normalizeRepository("hub.zama.org/zama-protocol/fhevm/test-suite/e2e")).toBe(
      "ghcr.io/fhevm/test-suite/e2e",
    );
  });

  test("strips docker.io prefix", () => {
    expect(normalizeRepository("docker.io/library/postgres")).toBe("library/postgres");
  });

  test("passes through ghcr.io paths unchanged", () => {
    expect(normalizeRepository("ghcr.io/zama-ai/fhevm/gateway-contracts")).toBe(
      "ghcr.io/zama-ai/fhevm/gateway-contracts",
    );
  });

  test("trims whitespace", () => {
    expect(normalizeRepository("  ghcr.io/foo  ")).toBe("ghcr.io/foo");
  });
});

describe("mergeArgs", () => {
  test("appends new args to empty base", () => {
    expect(mergeArgs([], ["--flag=value"])).toEqual(["--flag=value"]);
  });

  test("replaces existing --key=value args by prefix", () => {
    expect(mergeArgs(["--port=8080", "--host=0.0.0.0"], ["--port=9090"])).toEqual([
      "--host=0.0.0.0",
      "--port=9090",
    ]);
  });

  test("replaces exact duplicate args", () => {
    expect(mergeArgs(["--verbose", "--debug"], ["--verbose"])).toEqual(["--debug", "--verbose"]);
  });

  test("handles non-array base gracefully", () => {
    expect(mergeArgs(undefined, ["--new"])).toEqual(["--new"]);
    expect(mergeArgs(null, ["--new"])).toEqual(["--new"]);
  });

  test("preserves order for non-conflicting args", () => {
    expect(mergeArgs(["--a=1", "--b=2"], ["--c=3"])).toEqual(["--a=1", "--b=2", "--c=3"]);
  });

  test("does not over-match bare flags by prefix", () => {
    expect(mergeArgs(["--coprocessor-api-key-rotation", "--coprocessor-api-key"], ["--coprocessor-api-key"])).toEqual([
      "--coprocessor-api-key-rotation",
      "--coprocessor-api-key",
    ]);
  });
});

describe("writeEnvFile", () => {
  test("single-quotes JSON values for docker compose env files", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "fhevm-utils-"));
    tempDirs.push(dir);
    const file = path.join(dir, "test.env");
    await writeEnvFile(file, {
      KMS_CONNECTOR_HOST_CHAINS: '[{"url":"http://host-node:8545","chain_id":12345,"acl_address":"0x7"}]',
    });
    expect(await fs.readFile(file, "utf8")).toBe(
      "KMS_CONNECTOR_HOST_CHAINS='[{\"url\":\"http://host-node:8545\",\"chain_id\":12345,\"acl_address\":\"0x7\"}]'\n",
    );
  });
});

describe("toServiceName", () => {
  test("index 0 uses coprocessor- prefix", () => {
    expect(toServiceName("db-migration", 0)).toBe("coprocessor-db-migration");
    expect(toServiceName("host-listener", 0)).toBe("coprocessor-host-listener");
  });

  test("index > 0 uses coprocessorN- prefix", () => {
    expect(toServiceName("db-migration", 1)).toBe("coprocessor1-db-migration");
    expect(toServiceName("host-listener", 2)).toBe("coprocessor2-host-listener");
    expect(toServiceName("tfhe-worker", 4)).toBe("coprocessor4-tfhe-worker");
  });
});

describe("needsQuotes", () => {
  test("returns true for values with spaces", () => {
    expect(needsQuotes("hello world")).toBe(true);
  });

  test("returns true for values with special characters", () => {
    expect(needsQuotes('key="value"')).toBe(true);
    expect(needsQuotes("[1,2]")).toBe(true);
    expect(needsQuotes("{a:b}")).toBe(true);
  });

  test("returns false for simple values", () => {
    expect(needsQuotes("simple")).toBe(false);
    expect(needsQuotes("http://localhost:8080")).toBe(false);
    expect(needsQuotes("v0.11.0")).toBe(false);
  });
});

describe("withHexPrefix", () => {
  test("adds 0x prefix when missing", () => {
    expect(withHexPrefix("deadbeef")).toBe("0xdeadbeef");
  });

  test("preserves existing 0x prefix", () => {
    expect(withHexPrefix("0xdeadbeef")).toBe("0xdeadbeef");
  });
});

describe("uint256ToId", () => {
  test("pads to 64 hex characters", () => {
    const result = uint256ToId(1n);
    expect(result).toBe("0000000000000000000000000000000000000000000000000000000000000001");
    expect(result.length).toBe(64);
  });

  test("handles large values", () => {
    const result = uint256ToId((4n << 248n) + 1n);
    expect(result).toBe("0400000000000000000000000000000000000000000000000000000000000001");
  });
});

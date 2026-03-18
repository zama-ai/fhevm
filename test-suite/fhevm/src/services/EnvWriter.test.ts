import { describe, expect, test } from "bun:test";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { resolveEnvMap, rewriteRelayerConfig, writeWritableFile } from "./EnvWriter";

describe("resolveEnvMap", () => {
  test("resolves single-level interpolation", () => {
    const env = { A: "hello", B: "${A}-world" };
    resolveEnvMap(env);
    expect(env.B).toBe("hello-world");
  });

  test("resolves multi-level interpolation", () => {
    const env = { A: "1", B: "${A}2", C: "${B}3" };
    resolveEnvMap(env);
    expect(env.C).toBe("123");
  });

  test("leaves escaped placeholders untouched", () => {
    const env = { A: "hello", B: "$${A}" };
    resolveEnvMap(env);
    expect(env.B).toBe("$${A}");
  });

  test("passes through values without placeholders", () => {
    const env = { A: "plain", B: "also-plain" };
    resolveEnvMap(env);
    expect(env.A).toBe("plain");
    expect(env.B).toBe("also-plain");
  });

  test("throws on unresolved references", () => {
    const env = { A: "${MISSING}" };
    expect(() => resolveEnvMap(env)).toThrow("Unresolved env interpolation");
  });
});

describe("rewriteRelayerConfig", () => {
  test("returns config unchanged for modern relayer", () => {
    const config = {
      gateway: {
        readiness_checker: {
          gw_ciphertext_check: { retry: { max_retries: 5 } },
          host_acl_check: { retry: { max_retries: 3 } },
          public_decrypt: { timeout: 60 },
          user_decrypt: { timeout: 60 },
        },
      },
    };
    const state = { versions: { target: "latest-supported" as const, lockName: "t", env: { RELAYER_VERSION: "v0.10.0" }, sources: [] } };
    const result = rewriteRelayerConfig(config, state);
    // Modern relayer: no rewrite
    expect((result.gateway as Record<string, unknown>).readiness_checker).toHaveProperty(
      "gw_ciphertext_check",
    );
  });

  test("rewrites readiness_checker for legacy relayer", () => {
    const config = {
      gateway: {
        readiness_checker: {
          gw_ciphertext_check: { retry: { max_retries: 5 } },
          host_acl_check: { retry: { max_retries: 3 } },
          public_decrypt: { timeout: 60 },
          user_decrypt: { timeout: 60 },
          delegated_user_decrypt: { timeout: 30 },
        },
      },
    };
    const state = { versions: { target: "latest-supported" as const, lockName: "t", env: { RELAYER_VERSION: "v0.9.0" }, sources: [] } };
    const result = rewriteRelayerConfig(config, state);
    const checker = (result.gateway as Record<string, unknown>).readiness_checker as Record<
      string,
      unknown
    >;
    expect(checker).not.toHaveProperty("gw_ciphertext_check");
    expect(checker).not.toHaveProperty("host_acl_check");
    expect(checker).toHaveProperty("public_decrypt");
    expect(checker).toHaveProperty("user_decrypt");
    expect(checker).toHaveProperty("delegated_user_decrypt");
    expect(checker.retry).toEqual({ max_retries: 5 });
  });

  test("returns config unchanged when no gateway section", () => {
    const config = { server: { port: 8080 } };
    const state = { versions: { target: "latest-supported" as const, lockName: "t", env: { RELAYER_VERSION: "v0.9.0" }, sources: [] } };
    const result = rewriteRelayerConfig(config, state);
    expect(result).toEqual(config);
  });
});

describe("writeWritableFile", () => {
  test("writes files with group/world writable mode for bind-mounted address artifacts", async () => {
    const dir = await fs.mkdtemp(path.join(os.tmpdir(), "fhevm-env-writer-"));
    const file = path.join(dir, ".env.gateway");
    await writeWritableFile(file, "A=1\n");
    const stat = await fs.stat(file);
    expect(stat.mode & 0o777).toBe(0o666);
  });
});

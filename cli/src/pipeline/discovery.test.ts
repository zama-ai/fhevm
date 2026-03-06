import { afterEach, describe, expect, test } from "bun:test";
import { access, mkdir, rm } from "fs/promises";

import { deriveAllKeys } from "../config/keys";
import { DEFAULT_MNEMONIC, createDefaultConfig } from "../config/model";

import {
  __internal,
  discoverAndApplyMinioIp,
  discoverKmsSigner,
  fetchSignerAddress,
  parseSigningKeyHandle,
} from "./discovery";

function makeTempDir(): string {
  return `.fhevm/test-discovery/${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

afterEach(() => {
  __internal.resetDiscoveryOpsForTests();
});

describe("pipeline discovery", () => {
  test("parses signing key handle from logs", () => {
    const logs = [
      "=================GENERATING SIGNING KEYS=================",
      "created key handle: 0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    ].join("\n");

    expect(parseSigningKeyHandle(logs)).toBe(
      "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    );
    expect(parseSigningKeyHandle("no handle in logs")).toBeUndefined();
  });

  test("fetches signer address from MinIO path", async () => {
    const signer = "0x1234567890abcdef1234567890abcdef12345678";
    const handle = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    const server = Bun.serve({
      port: 0,
      fetch(req) {
        if (req.url.endsWith(`/kms-public/PUB/VerfAddress/${handle}`)) {
          return new Response(signer);
        }
        return new Response("missing", { status: 404 });
      },
    });

    try {
      const found = await fetchSignerAddress(server.url.href.replace(/\/$/, ""), "kms-public", "PUB", handle);
      expect(found).toBe(signer);
    } finally {
      server.stop(true);
    }
  });

  test("discovers and applies MinIO IP to runtime config", async () => {
    const dir = makeTempDir();
    await mkdir(dir, { recursive: true });

    const config = createDefaultConfig(deriveAllKeys(DEFAULT_MNEMONIC, 1, 1));
    __internal.setDiscoveryOpsForTests({
      discoverMinioIp: async () => "172.18.0.2",
    });

    const minioIp = await discoverAndApplyMinioIp(config);
    expect(minioIp).toBe("172.18.0.2");
    expect(config.runtime.minioIp).toBe("172.18.0.2");

    await expect(access(`${dir}/coprocessor.env`)).rejects.toBeDefined();

    await rm(dir, { recursive: true, force: true });
  });

  test("discovers kms signer by polling logs and querying MinIO", async () => {
    const config = createDefaultConfig(deriveAllKeys(DEFAULT_MNEMONIC, 1, 1));

    const handle = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const signer = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    let polls = 0;

    const server = Bun.serve({
      port: 0,
      fetch(req) {
        if (req.url.endsWith(`/kms-public/PUB/VerfAddress/${handle}`)) {
          return new Response(signer);
        }
        return new Response("missing", { status: 404 });
      },
    });
    const url = new URL(server.url.href);
    config.runtime.minioIp = url.hostname;
    config.ports.minioApi = Number.parseInt(url.port, 10);

    __internal.setDiscoveryOpsForTests({
      getContainerLogs: async () => {
        polls += 1;
        if (polls < 2) {
          return "warming up";
        }
        return `key handle: ${handle}`;
      },
    });

    try {
      const found = await discoverKmsSigner(config, { timeoutMs: 2_000, pollIntervalMs: 10 });
      expect(found).toBe(signer);
      expect(config.runtime.kmsSigner).toBe(signer);
    } finally {
      server.stop(true);
    }
  });

  test("fails with docker error when signer discovery times out", async () => {
    const config = createDefaultConfig(deriveAllKeys(DEFAULT_MNEMONIC, 1, 1));
    __internal.setDiscoveryOpsForTests({
      getContainerLogs: async () => "still booting",
    });

    await expect(discoverKmsSigner(config, { timeoutMs: 30, pollIntervalMs: 10 })).rejects.toMatchObject({
      exitCode: 3,
      step: "kms-signer-discovery",
      service: "kms-core",
    });
  });
});

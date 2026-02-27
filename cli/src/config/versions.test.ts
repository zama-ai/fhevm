import { afterEach, describe, expect, it } from "bun:test";
import { mkdir, rm } from "fs/promises";
import { ExitCode, FhevmCliError } from "../errors";
import {
  VERSION_TTL_MS,
  buildVersionEnvVars,
  loadVersionCache,
  resolveVersion,
  saveVersionCache,
  type VersionCache,
  type VersionGroup,
} from "./versions";

const originalFetch = globalThis.fetch;

function mockFetch(
  implementation: (...args: Parameters<typeof fetch>) => ReturnType<typeof fetch>,
): typeof fetch {
  const fn = implementation as typeof fetch;
  fn.preconnect = originalFetch.preconnect;
  return fn;
}

function createCachePath(): Promise<string> {
  const root = ".fhevm/test-cache";
  const dir = `${root}/${Date.now()}-${Math.random().toString(16).slice(2)}`;
  return mkdir(dir, { recursive: true }).then(() => `${dir}/version-cache.json`);
}

async function cleanupCachePath(cachePath: string): Promise<void> {
  const index = cachePath.lastIndexOf("/");
  const dir = index === -1 ? "." : cachePath.slice(0, index);
  await rm(dir, { recursive: true, force: true });
}

function setFetchTag(tag: string): void {
  globalThis.fetch = mockFetch(async () =>
    new Response(JSON.stringify({ tag_name: tag }), {
      status: 200,
      headers: { "content-type": "application/json" },
    }),
  );
}

afterEach(() => {
  globalThis.fetch = originalFetch;
  delete process.env.COPROCESSOR_VERSION;
  delete process.env.COPROCESSOR_TFHE_WORKER_VERSION;
});

describe("versions", () => {
  it("uses env var override first", async () => {
    process.env.COPROCESSOR_VERSION = "v-env";
    const cachePath = await createCachePath();
    setFetchTag("v-network");

    const resolved = await resolveVersion("coprocessor", cachePath);
    expect(resolved).toBe("v-env");

    await cleanupCachePath(cachePath);
  });

  it("uses per-service env override when group override is not set", async () => {
    process.env.COPROCESSOR_TFHE_WORKER_VERSION = "v-service";
    const cachePath = await createCachePath();
    setFetchTag("v-network");

    const resolved = await resolveVersion("coprocessor", cachePath);
    expect(resolved).toBe("v-service");

    await cleanupCachePath(cachePath);
  });

  it("returns pinned version when set", async () => {
    const cachePath = await createCachePath();
    let called = false;
    globalThis.fetch = mockFetch(async () => {
      called = true;
      return new Response("{}");
    });

    const resolved = await resolveVersion("coprocessor", cachePath);
    expect(resolved).toBe("2458fa9");
    expect(called).toBe(false);

    await cleanupCachePath(cachePath);
  });

  it("reads a valid cache entry without network", async () => {
    const cachePath = await createCachePath();
    const cache: VersionCache = {
      ttlMs: VERSION_TTL_MS,
      versions: {
        core: { version: "v-cached", fetchedAt: Date.now() },
      },
    };
    await saveVersionCache(cache, cachePath);
    let called = false;
    globalThis.fetch = mockFetch(async () => {
      called = true;
      return new Response("{}");
    });

    const resolved = await resolveVersion("core", cachePath);
    expect(resolved).toBe("v-cached");
    expect(called).toBe(false);

    await cleanupCachePath(cachePath);
  });

  it("fetches and updates cache when existing entry is expired", async () => {
    const cachePath = await createCachePath();
    const cache: VersionCache = {
      ttlMs: VERSION_TTL_MS,
      versions: {
        core: { version: "v-old", fetchedAt: Date.now() - VERSION_TTL_MS - 1 },
      },
    };
    await saveVersionCache(cache, cachePath);
    setFetchTag("v-fresh");

    const resolved = await resolveVersion("core", cachePath);
    const stored = await loadVersionCache(cachePath);

    expect(resolved).toBe("v-fresh");
    expect(stored?.versions.core?.version).toBe("v-fresh");

    await cleanupCachePath(cachePath);
  });

  it("maps group versions to compose env vars", () => {
    const versions: Record<VersionGroup, string> = {
      coprocessor: "v1",
      "kms-connector": "v2",
      contracts: "v3",
      core: "v4",
      relayer: "v5",
      "test-suite": "v6",
    };

    const mapped = buildVersionEnvVars(versions);
    expect(mapped.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v1");
    expect(mapped.CONNECTOR_GW_LISTENER_VERSION).toBe("v2");
    expect(mapped.GATEWAY_VERSION).toBe("v3");
    expect(mapped.CORE_VERSION).toBe("v4");
    expect(mapped.RELAYER_MIGRATE_VERSION).toBe("v5");
    expect(mapped.TEST_SUITE_VERSION).toBe("v6");
  });

  it("keeps per-service env overrides when building compose version vars", () => {
    process.env.COPROCESSOR_TFHE_WORKER_VERSION = "v-tfhe";
    const mapped = buildVersionEnvVars({
      coprocessor: "v1",
      "kms-connector": "v2",
      contracts: "v3",
      core: "v4",
      relayer: "v5",
      "test-suite": "v6",
    });

    expect(mapped.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v-tfhe");
    expect(mapped.COPROCESSOR_GW_LISTENER_VERSION).toBe("v1");
  });

  it("throws config error when network fetch fails and cache is empty", async () => {
    const cachePath = await createCachePath();
    globalThis.fetch = mockFetch(async () => new Response("{}", { status: 503 }));

    try {
      await resolveVersion("core", cachePath);
      throw new Error("expected resolveVersion to fail");
    } catch (error) {
      expect(error).toBeInstanceOf(FhevmCliError);
      expect((error as FhevmCliError).exitCode).toBe(ExitCode.CONFIG);
    }

    await cleanupCachePath(cachePath);
  });
});

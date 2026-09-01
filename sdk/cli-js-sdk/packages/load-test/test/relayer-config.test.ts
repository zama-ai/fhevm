import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";

import { snapshotRelayerConfig } from "../src/collectors/relayer-config";
import { safeArtifactText } from "../src/shared/safe-artifact";

let directory: string | undefined;

afterEach(async () => {
  if (directory) await rm(directory, { recursive: true, force: true });
  directory = undefined;
});

describe("durable artifact redaction", () => {
  it("recursively redacts nested JSON relayer configuration", async () => {
    directory = await mkdtemp(join(tmpdir(), "load-test-redaction-"));
    const path = join(directory, "relayer.json");
    await writeFile(path, JSON.stringify({
      database: {
        url: "postgres://service:db-password@safe-host/db",
        password: "db-secret",
      },
      workers: [{ apiKey: "worker-secret", concurrency: 4 }],
      maxConcurrency: 8,
    }));

    const snapshot = await snapshotRelayerConfig(path);
    expect(snapshot?.raw).not.toContain("db-secret");
    expect(snapshot?.raw).not.toContain("worker-secret");
    expect(snapshot?.raw).toContain("postgres://service:[REDACTED]@safe-host/db");
    expect(snapshot?.raw).not.toContain("db-password");
    expect(snapshot?.raw).toContain('"maxConcurrency": 8');
    expect(JSON.parse(snapshot?.raw ?? "{}")).toMatchObject({
      database: { password: "[REDACTED]" },
      workers: [{ apiKey: "[REDACTED]", concurrency: 4 }],
    });
  });

  it("redacts TOML-style secret assignments and bounds arbitrary errors", async () => {
    directory = await mkdtemp(join(tmpdir(), "load-test-redaction-"));
    const path = join(directory, "relayer.toml");
    await writeFile(path, 'max_concurrency = 8\nprivate_key = "top-secret"\n');
    const snapshot = await snapshotRelayerConfig(path);
    expect(snapshot?.raw).toContain("max_concurrency = 8");
    expect(snapshot?.raw).toContain("private_key = [REDACTED]");
    expect(snapshot?.raw).not.toContain("top-secret");

    const safe = safeArtifactText(`authorization=Bearer-token ${"a".repeat(900)}`);
    expect(safe).not.toContain("Bearer-token");
    expect(safe?.length).toBeLessThanOrEqual(500);
  });
});

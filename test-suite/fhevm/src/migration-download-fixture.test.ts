import { describe, expect, test } from "bun:test";
import { prepareMigrationDownloadFixture } from "../rollouts/v0.14-to-v0.15-gpu-key-migration/download-fixture";
import runMigration from "../rollouts/v0.14-to-v0.15-gpu-key-migration/run";
import type { RolloutRunContext } from "./commands/rollout-run";

describe("migration download fixture preflight", () => {
  test("other modes need neither a fixture nor a Rust build", async () => {
    for (const mode of ["none", "lagging-recipient", "application-interruption", "download-interrupt", "download-wrong-digest", "download-malformed"]) {
      await prepareMigrationDownloadFixture(mode, undefined, async () => {
        throw new Error("unexpected build");
      });
    }
  });

  test("awaits compilation and real fixture validation before returning", async () => {
    let finish!: () => void;
    const pending = new Promise<void>((resolve) => { finish = resolve; });
    let returned = false;
    const prepared = prepareMigrationDownloadFixture("download-wrong-key", "/fixture with spaces.bin", async (argv, options) => {
      expect(argv).toEqual(["cargo", "run", "--release", "-p", "host-listener", "--features", "test-failpoints", "--bin", "migration_test_key", "--", "--validate", "/fixture with spaces.bin"]);
      expect(options?.cwd).toEndWith("/coprocessor/fhevm-engine");
      expect(options?.env).toEqual({ SQLX_OFFLINE: "true" });
      await pending;
      return 0;
    }).then(() => { returned = true; });
    await Promise.resolve();
    expect(returned).toBe(false);
    finish();
    await prepared;
    expect(returned).toBe(true);
  });

  test("propagates build or deserialization failure", async () => {
    await expect(prepareMigrationDownloadFixture("download-wrong-key", "/invalid.bin", async () => {
      throw new Error("invalid compressed key");
    })).rejects.toThrow("invalid compressed key");
  });

  test("the runbook rejects a missing fixture before touching its context", async () => {
    const previousMode = process.env.RFC029_MIGRATION_FAULT;
    const previousFixture = process.env.RFC029_WRONG_KEY_FILE;
    process.env.RFC029_MIGRATION_FAULT = "download-wrong-key";
    delete process.env.RFC029_WRONG_KEY_FILE;
    try {
      const context = new Proxy({}, { get() { throw new Error("stack context accessed before preflight"); } });
      await expect(runMigration(context as RolloutRunContext)).rejects.toThrow("RFC029_WRONG_KEY_FILE");
    } finally {
      if (previousMode === undefined) delete process.env.RFC029_MIGRATION_FAULT;
      else process.env.RFC029_MIGRATION_FAULT = previousMode;
      if (previousFixture === undefined) delete process.env.RFC029_WRONG_KEY_FILE;
      else process.env.RFC029_WRONG_KEY_FILE = previousFixture;
    }
  });
});

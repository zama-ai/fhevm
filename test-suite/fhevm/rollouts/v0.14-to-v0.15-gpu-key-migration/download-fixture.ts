import path from "node:path";
import { REPO_ROOT } from "../../src/layout";
import { runStreaming } from "../../src/utils/process";

/** Compile and validate before booting a stack or starting timed fault ownership. */
export async function prepareMigrationDownloadFixture(
  mode: string,
  fixture: string | undefined,
  execute: typeof runStreaming = runStreaming,
): Promise<void> {
  if (mode !== "download-wrong-key") return;
  if (!fixture) throw new Error("RFC029_WRONG_KEY_FILE must name an independent compressed key fixture");
  await execute([
    "cargo", "run", "--release", "-p", "host-listener",
    "--features", "test-failpoints", "--bin", "migration_test_key",
    "--", "--validate", fixture,
  ], {
    cwd: path.join(REPO_ROOT, "coprocessor/fhevm-engine"),
    env: { SQLX_OFFLINE: "true" },
  });
}

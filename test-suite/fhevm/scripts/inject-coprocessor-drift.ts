#!/usr/bin/env bun
import { spawnSync } from "node:child_process";

import {
  DRIFT_CLEANUP_SQL,
  DRIFT_INSTALL_SQL,
  driftDatabaseName,
  parseDriftInstanceIndex,
  parsePositiveInteger,
} from "../src/ciphertext-drift";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const main = async () => {
  const env = process.env;
  const instanceIndex = parseDriftInstanceIndex(process.argv[2] ?? "1");
  const timeoutSeconds = parsePositiveInteger(env.DRIFT_INJECT_TIMEOUT_SECONDS ?? "180", "DRIFT_INJECT_TIMEOUT_SECONDS");
  const pollIntervalSeconds = parsePositiveInteger(env.DRIFT_INJECT_POLL_INTERVAL_SECONDS ?? "2", "DRIFT_INJECT_POLL_INTERVAL_SECONDS");
  const postgresContainer = env.POSTGRES_CONTAINER ?? "coprocessor-and-kms-db";
  const postgresUser = env.POSTGRES_USER ?? "postgres";
  const postgresPassword = env.POSTGRES_PASSWORD ?? "postgres";
  const dbName = driftDatabaseName(instanceIndex);

  const dockerPsql = (args: string[], input?: string) => {
    const result = spawnSync(
      "docker",
      [
        "exec",
        ...(input ? ["-i"] : []),
        "-e",
        `PGPASSWORD=${postgresPassword}`,
        postgresContainer,
        "psql",
        "-U",
        postgresUser,
        "-d",
        dbName,
        ...args,
      ],
      { encoding: "utf8", input },
    );
    if (result.status !== 0) {
      const message = result.stderr || result.stdout || `docker exec failed with exit code ${result.status ?? 1}`;
      throw new Error(message.trim());
    }
    return result.stdout.trim();
  };

  const cleanup = () => {
    dockerPsql([], DRIFT_CLEANUP_SQL);
  };

  const exitWithCleanup = (code: number) => {
    try {
      cleanup();
    } catch {
      // Best effort cleanup; the main error path already explains the failure.
    }
    process.exit(code);
  };

  process.on("SIGINT", () => exitWithCleanup(130));
  process.on("SIGTERM", () => exitWithCleanup(143));

  try {
    dockerPsql([], DRIFT_INSTALL_SQL);
    const deadline = Date.now() + timeoutSeconds * 1000;
    while (Date.now() < deadline) {
      if (dockerPsql(["-t", "-A", "-c", "SELECT consumed::int FROM drift_injection_state WHERE id = TRUE;"]) === "1") {
        const handleHex = dockerPsql(["-t", "-A", "-c", "SELECT encode(injected_handle, 'hex') FROM drift_injection_state WHERE id = TRUE;"]);
        if (handleHex) {
          process.stdout.write(`${handleHex}\n`);
          return;
        }
      }
      await sleep(pollIntervalSeconds * 1000);
    }
    throw new Error(`timed out waiting for drift injection trigger to fire in ${dbName}`);
  } finally {
    cleanup();
  }
};

try {
  await main();
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  process.stderr.write(`${message}\n`);
  process.exit(1);
}

import { expect, test } from "bun:test";
import path from "node:path";

import { loadRolloutRunbook } from "./commands/rollout-run";
import { phaseVersions, scenario } from "../rollouts/sns-worker-s3-format-migration/versions";

const CLI_DIR = path.resolve(import.meta.dir, "..");

test("loads the checked-in sns-worker S3 format migration runbook", async () => {
  await expect(
    loadRolloutRunbook(path.join(CLI_DIR, "rollouts/sns-worker-s3-format-migration/run.ts")),
  ).resolves.toBeFunction();
});

test("keeps the sns-worker migration rollout scoped to sns image and env mode", () => {
  expect(scenario).toBe("two-of-three");
  expect(phaseVersions.baseline.COPROCESSOR_SNS_WORKER_VERSION).toBe(process.env.OLD_SNS_IMAGE_TAG || "pre-s3-format");
  expect(phaseVersions.baseline.S3_MIGRATION_MODE).toBe("no");
  expect(phaseVersions.baseline.RELAYER_VERSION).toBe("v0.13.0");
  expect(phaseVersions.baseline.RELAYER_MIGRATE_VERSION).toBe("v0.13.0");
  expect(phaseVersions.baseline.CORE_VERSION).toBe("v0.13.20");
  expect(phaseVersions.baseline.TEST_SUITE_VERSION).toBe("fhevm-local");
  expect(phaseVersions.baseline.RELAYER_SDK_VERSION).toBe("");
  expect(phaseVersions.baseline.LISTENER_CORE_VERSION).toBe("v0.13.0");
  expect(phaseVersions.baseline.COPROCESSOR_HOST_LISTENER_VERSION).toBe("v0.13.0");
  expect(phaseVersions.baseline.COPROCESSOR_GW_LISTENER_VERSION).toBe("v0.13.0");

  expect(phaseVersions.sns.COPROCESSOR_SNS_WORKER_VERSION).toBe("fhevm-local");
  expect(phaseVersions.sns.S3_MIGRATION_MODE).toBe("concurrent");
  expect(phaseVersions.sns.CLEAN_OLD_S3_FORMAT_VERSION).toBe("false");
  expect(phaseVersions.sns.TEST_SUITE_VERSION).toBe(phaseVersions.baseline.TEST_SUITE_VERSION);
  expect(phaseVersions.sns.RELAYER_SDK_VERSION).toBe(phaseVersions.baseline.RELAYER_SDK_VERSION);

  expect(phaseVersions.sns.LISTENER_CORE_VERSION).toBe(phaseVersions.baseline.LISTENER_CORE_VERSION);
  expect(phaseVersions.sns.COPROCESSOR_DB_MIGRATION_VERSION).toBe(phaseVersions.baseline.COPROCESSOR_DB_MIGRATION_VERSION);
  expect(phaseVersions.sns.COPROCESSOR_HOST_LISTENER_VERSION).toBe(phaseVersions.baseline.COPROCESSOR_HOST_LISTENER_VERSION);
  expect(phaseVersions.sns.COPROCESSOR_GW_LISTENER_VERSION).toBe(phaseVersions.baseline.COPROCESSOR_GW_LISTENER_VERSION);
  expect(phaseVersions.sns.COPROCESSOR_TX_SENDER_VERSION).toBe(phaseVersions.baseline.COPROCESSOR_TX_SENDER_VERSION);
  expect(phaseVersions.sns.COPROCESSOR_TFHE_WORKER_VERSION).toBe(phaseVersions.baseline.COPROCESSOR_TFHE_WORKER_VERSION);
  expect(phaseVersions.sns.COPROCESSOR_ZKPROOF_WORKER_VERSION).toBe(phaseVersions.baseline.COPROCESSOR_ZKPROOF_WORKER_VERSION);
});

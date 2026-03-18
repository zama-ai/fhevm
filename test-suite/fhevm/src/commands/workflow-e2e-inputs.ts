import fs from "node:fs/promises";

import { Effect } from "effect";

import { PreflightError } from "../errors";
import { CommandRunner } from "../services/CommandRunner";

const WORKFLOW_BUILD_OUTPUTS = {
  "coprocessor-docker-build": {
    "db_migration_build_result": "coprocessor-db-migration-version",
    "gw_listener_build_result": "coprocessor-gw-listener-version",
    "host_listener_build_result": "coprocessor-host-listener-version",
    "sns_worker_build_result": "coprocessor-sns-worker-version",
    "tfhe_worker_build_result": "coprocessor-tfhe-worker-version",
    "tx_sender_build_result": "coprocessor-tx-sender-version",
    "zkproof_worker_build_result": "coprocessor-zkproof-worker-version",
  },
  "kms-connector-docker-build": {
    "db_migration_build_result": "connector-db-migration-version",
    "gw_listener_build_result": "connector-gw-listener-version",
    "kms_worker_build_result": "connector-kms-worker-version",
    "tx_sender_build_result": "connector-tx-sender-version",
  },
  "gateway-contracts-docker-build": {
    "build_result": "gateway-version",
  },
  "host-contracts-docker-build": {
    "build_result": "host-version",
  },
  "test-suite-docker-build": {
    "build_result": "test-suite-version",
  },
} as const;

type WorkflowNeeds = Partial<
  Record<keyof typeof WORKFLOW_BUILD_OUTPUTS, { outputs?: Record<string, string> }>
>;

export const workflowE2eInputs = (options: {
  previousCommit: string;
  newCommit: string;
  needsFile: string;
}) =>
  Effect.gen(function* () {
    const runner = yield* CommandRunner;
    const needs = yield* Effect.tryPromise({
      try: () => fs.readFile(options.needsFile, "utf8").then((text) => JSON.parse(text) as WorkflowNeeds),
      catch: (error) =>
        new PreflightError({
          message: `Failed to read workflow needs file: ${error}`,
        }),
    });

    const output: Record<string, string> = {};
    for (const [job, mapping] of Object.entries(WORKFLOW_BUILD_OUTPUTS)) {
      const needsEntry = needs[job as keyof typeof WORKFLOW_BUILD_OUTPUTS];
      for (const [buildResultKey, outputKey] of Object.entries(mapping)) {
        const selectedCommit =
          needsEntry?.outputs?.[buildResultKey] === "success"
            ? options.newCommit
            : options.previousCommit;
        const result = yield* runner.run(
          ["git", "rev-parse", "--short=7", selectedCommit],
          { allowFailure: true },
        );
        output[outputKey] =
          result.code === 0 ? result.stdout.trim() : selectedCommit.slice(0, 7);
      }
    }

    console.log(JSON.stringify(output, null, 2));
  });

import assert from "node:assert/strict";
import { test } from "node:test";

import type { UserDecryptValidationArtifact } from "@cli-fhevm-sdk/toolkit/types";
import { buildUserDecryptResultUrl } from "@cli-fhevm-sdk/toolkit/flows/relayer-result/user-decrypt";

/** Minimal artifact carrying only the fields the URL derivation reads. */
const artifact = (
  overrides: Partial<UserDecryptValidationArtifact> = {},
): UserDecryptValidationArtifact =>
  ({
    schemaVersion: 1,
    flow: "user-decrypt",
    network: "devnet",
    relayer: { jobId: "job-123" },
    ...overrides,
  }) as UserDecryptValidationArtifact;

test("derives the user-decrypt URL from the artifact network and job id", () => {
  assert.equal(
    buildUserDecryptResultUrl(artifact()),
    "https://relayer.dev.zama.cloud/v2/user-decrypt/job-123",
  );
});

test("derives the delegated-user-decrypt path segment from the artifact flow", () => {
  assert.equal(
    buildUserDecryptResultUrl(artifact({ flow: "delegated-user-decrypt" })),
    "https://relayer.dev.zama.cloud/v2/delegated-user-decrypt/job-123",
  );
});

test("--job-id override wins over the artifact job id", () => {
  assert.equal(
    buildUserDecryptResultUrl(artifact(), { jobId: "override-job" }),
    "https://relayer.dev.zama.cloud/v2/user-decrypt/override-job",
  );
});

test("--url override wins over every derived component", () => {
  assert.equal(
    buildUserDecryptResultUrl(artifact(), {
      url: "https://exotic.example/custom/path",
      jobId: "ignored",
      relayerUrl: "https://ignored.example",
    }),
    "https://exotic.example/custom/path",
  );
});

test("relayerUrl override replaces the network base and is normalized", () => {
  assert.equal(
    buildUserDecryptResultUrl(artifact(), { relayerUrl: "localhost:3000/v2" }),
    "http://localhost:3000/v2/user-decrypt/job-123",
  );
});

test("missing job id with no override throws an actionable error", () => {
  assert.throws(
    () => buildUserDecryptResultUrl(artifact({ relayer: {} })),
    /--job-id/,
  );
});

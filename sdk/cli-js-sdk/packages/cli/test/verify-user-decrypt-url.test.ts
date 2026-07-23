import { expect, test } from "vitest";

import type { UserDecryptValidationArtifact } from "@cli-fhevm-sdk/toolkit/types";
import { buildUserDecryptResultUrl } from "@cli-fhevm-sdk/toolkit/flows/relayer-result/user-decrypt";

/** Minimal artifact carrying only the fields the URL derivation reads. */
const artifact = (
  overrides: Partial<UserDecryptValidationArtifact> = {},
): UserDecryptValidationArtifact =>
  ({
    schemaVersion: 2,
    flow: "user-decrypt",
    network: "devnet",
    relayer: { jobId: "job-123" },
    ...overrides,
  }) as UserDecryptValidationArtifact;

test("derives the user-decrypt URL from the artifact network and job id", () => {
  expect(buildUserDecryptResultUrl(artifact())).toBe(
    "https://relayer.dev.zama.cloud/v2/user-decrypt/job-123",
  );
});

test("derives the delegated-user-decrypt path segment from the artifact flow", () => {
  expect(
    buildUserDecryptResultUrl(artifact({ flow: "delegated-user-decrypt" })),
  ).toBe("https://relayer.dev.zama.cloud/v2/delegated-user-decrypt/job-123");
});

test("--job-id override wins over the artifact job id", () => {
  expect(buildUserDecryptResultUrl(artifact(), { jobId: "override-job" })).toBe(
    "https://relayer.dev.zama.cloud/v2/user-decrypt/override-job",
  );
});

test("--url override wins over every derived component", () => {
  expect(
    buildUserDecryptResultUrl(artifact(), {
      url: "https://exotic.example/custom/path",
      jobId: "ignored",
      relayerUrl: "https://ignored.example",
    }),
  ).toBe("https://exotic.example/custom/path");
});

test("relayerUrl override replaces the network base and is normalized", () => {
  expect(
    buildUserDecryptResultUrl(artifact(), { relayerUrl: "localhost:3000/v2" }),
  ).toBe("http://localhost:3000/v2/user-decrypt/job-123");
});

test("missing job id with no override throws an actionable error", () => {
  expect(() => buildUserDecryptResultUrl(artifact({ relayer: {} }))).toThrow(
    /--job-id/,
  );
});

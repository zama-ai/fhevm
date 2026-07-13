import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";

import type { RequestRecord } from "../src/flows/types";
import { Recorder, type TargetRequestRecord } from "../src/runner/recorder";
import { readJsonl } from "../src/shared/jsonl";

let directory: string | undefined;

afterEach(async () => {
  if (directory) await rm(directory, { recursive: true, force: true });
  directory = undefined;
});

const record = (
  flow: RequestRecord["flow"],
  index: number,
  errorLabel: string,
  errorMessage: string,
): RequestRecord => ({
  flow,
  index,
  startedAtMs: index,
  sentRequestId: `sent-${index.toString()}`,
  pollCount: 0,
  outcome: "failed",
  errorLabel,
  errorMessage,
});

describe("Recorder durable error redaction", () => {
  it("redacts and bounds input-proof, public-decrypt, and user-decrypt errors", async () => {
    directory = await mkdtemp(join(tmpdir(), "load-test-recorder-redaction-"));
    const requestsPath = join(directory, "requests.jsonl");
    const targetAPath = join(directory, "target-a.jsonl");
    const targetBPath = join(directory, "target-b.jsonl");
    const recorder = await Recorder.open(requestsPath, {
      relayerAPath: targetAPath,
      relayerBPath: targetBPath,
    });
    const messages = [
      "request failed at postgres://service:db-password@host/db api_key=input-secret " + "x".repeat(700),
      "Authorization: Bearer public-secret signature=" + "a".repeat(130),
      JSON.stringify({ message: "KMS failed", privateKey: "user-secret", nested: { token: "nested-secret" } }),
    ];
    await recorder.record(record("input-proof", 0, "client_transport_error", messages[0] ?? ""));
    await recorder.record(record("public-decrypt", 1, "sdk_public_decrypt_error", messages[1] ?? ""));
    await recorder.record({
      ...record("user-decrypt", 2, "sdk_user_decrypt_error", messages[2] ?? ""),
      outcomeB: "failed",
      pollCountB: 0,
      errorLabelB: "candidate_sdk_error",
      errorMessageB: "transport_key=candidate-secret",
    });
    await recorder.close();

    const persisted = await readJsonl<RequestRecord>(requestsPath);
    expect(persisted.map((entry) => entry.errorLabel)).toEqual([
      "client_transport_error", "sdk_public_decrypt_error", "sdk_user_decrypt_error",
    ]);
    for (const entry of persisted) {
      expect(entry.errorMessage?.length).toBeLessThanOrEqual(500);
    }
    const serialized = JSON.stringify(persisted);
    for (const secret of [
      "db-password", "input-secret", "public-secret", "user-secret", "nested-secret",
      "candidate-secret", "a".repeat(130),
    ]) {
      expect(serialized).not.toContain(secret);
    }
    expect(serialized).toContain("[REDACTED]");
    expect(recorder.records).toEqual(persisted);

    const targetA = await readJsonl<TargetRequestRecord>(targetAPath);
    const targetB = await readJsonl<TargetRequestRecord>(targetBPath);
    expect(targetA).toHaveLength(3);
    expect(targetB).toHaveLength(1);
    expect(targetB[0]).toMatchObject({
      errorLabel: "candidate_sdk_error",
      errorMessage: "transport_key=[REDACTED]",
    });
  });
});

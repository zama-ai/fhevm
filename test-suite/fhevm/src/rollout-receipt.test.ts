import { describe, expect, test } from "bun:test";

import {
  failureDiagnosticContainerNames,
  diagnosticLogArgs,
  diagnosticLogOutput,
  startRolloutReceipt,
  formatStateHashComparison,
  kmsConnectorPartyIds,
  receiptJsonlPath,
  receiptMarkdownPath,
  requireDockerSnapshot,
} from "./commands/rollout-receipt";
import { withTempStateDir } from "./test-state";

describe("rollout receipt", () => {
  test("writes a receipt with the version changes since the previous entry", async () => {
    await withTempStateDir(async () => {
      const receipt = await startRolloutReceipt("key-migration-from-v0.14");
      await receipt.record("boot", "stack before the test", { versions: { RELAYER_VERSION: "old" } });
      await receipt.record("upgrade", "relayer", { versions: { RELAYER_VERSION: "new" } });

      const entries = (await Bun.file(receiptJsonlPath()).text())
        .trim()
        .split("\n")
        .map((line) => JSON.parse(line));
      expect(entries[0].versionChanges).toEqual([{ key: "RELAYER_VERSION", to: "old" }]);
      expect(entries[1].versionChanges).toEqual([{ key: "RELAYER_VERSION", from: "old", to: "new" }]);
      expect(await Bun.file(receiptMarkdownPath()).text()).toContain("RELAYER_VERSION");
    });
  });
  test("requires audit-critical Docker snapshots to contain inspected containers", () => {
    expect(() => requireDockerSnapshot({ containers: [], error: "docker inspect failed" })).toThrow(
      "Required Docker snapshot failed: docker inspect failed",
    );
    expect(() => requireDockerSnapshot({ containers: [] })).toThrow("contained no project containers");
    expect(() =>
      requireDockerSnapshot({
        containers: [{ image: "image", imageId: "sha256:id", name: "kms-core", state: "running" }],
      }),
    ).not.toThrow();
  });

  test("bounds diagnostic logs while retaining the last hour", () => {
    expect(diagnosticLogArgs("kms-core")).toEqual([
      "docker",
      "logs",
      "--since",
      "1h",
      "--tail",
      "10000",
      "kms-core",
    ]);
  });

  test("filters exporter noise before bounding diagnostic logs", () => {
    const output = diagnosticLogOutput(
      ["root cause", ...Array.from({ length: 2100 }, () => "BatchSpanProcessor.ExportError")].join("\n"),
      "",
      2,
    );
    expect(output).toBe("root cause");

    const bounded = diagnosticLogOutput(["first", "second", "third", "last"].join("\n"), "", 2);
    expect(bounded).toContain("first");
    expect(bounded).toContain("last");
    expect(bounded.split("\n")).toHaveLength(3);
  });

  test("discovers the exact connector databases from partial Docker state", () => {
    expect(kmsConnectorPartyIds(["kms-connector-db-migration"])).toEqual([1]);
    expect(
      kmsConnectorPartyIds([
        "kms-connector-4-db-migration",
        "kms-connector-db-migration",
        "kms-connector-3-db-migration",
        "kms-connector-4-db-migration",
        "kms-connector-3-kms-worker",
      ]),
    ).toEqual([1, 3, 4]);
  });

  test("retains runtime logs needed to diagnose rollout traffic failures", () => {
    expect(
      failureDiagnosticContainerNames([
        { image: "image", imageId: "id", name: "fhevm-relayer", state: "running" },
        { image: "image", imageId: "id", name: "coprocessor-gcs-tfhe-worker", state: "running" },
        { image: "image", imageId: "id", name: "coprocessor1-gcs-zkproof-worker", state: "running" },
        { image: "image", imageId: "id", name: "coprocessor-gcs-consensus-detector", state: "running" },
        { image: "image", imageId: "id", name: "coprocessor1-gcs-upgrade-controller", state: "running" },
        { image: "image", imageId: "id", name: "coprocessor-sns-worker", state: "exited" },
        { image: "image", imageId: "id", name: "kms-core-2", state: "running" },
        { image: "image", imageId: "id", name: "host-node", state: "running" },
      ]),
    ).toEqual([
      "fhevm-relayer",
      "coprocessor-gcs-tfhe-worker",
      "coprocessor1-gcs-zkproof-worker",
      "coprocessor-gcs-consensus-detector",
      "coprocessor1-gcs-upgrade-controller",
      "coprocessor-sns-worker",
      "kms-core-2",
    ]);
  });

  test("pinpoints divergent or missing Blue/Green state hashes", () => {
    expect(
      formatStateHashComparison([
        { database: "coprocessor", rows: ["1|100|aaa|t", "1|101|bbb|t"] },
        { database: "coprocessor_1", rows: ["1|100|aaa|t", "1|101|ccc|t", "2|50|ddd|f"] },
      ]),
    ).toBe(
      [
        "3 anchor(s) compared across coprocessor, coprocessor_1; 2 mismatch(es)",
        "1:101 coprocessor=bbb|uploaded=t coprocessor_1=ccc|uploaded=t",
        "2:50 coprocessor=missing coprocessor_1=ddd|uploaded=f",
      ].join("\n"),
    );
  });

  test("records a failed required Docker snapshot before rejecting it", async () => {
    await withTempStateDir(async () => {
      const receipt = await startRolloutReceipt("key-migration-from-v0.14", {
        async inspectContainers() {
          return { containers: [], error: "daemon unavailable" };
        },
      });
      await expect(receipt.record("upgrade-kms-operator", "KMS operator 2", { docker: true })).rejects.toThrow(
        "Required Docker snapshot failed: daemon unavailable",
      );

      const entry = JSON.parse((await Bun.file(receiptJsonlPath()).text()).trim());
      expect(entry.dockerInspectError).toBe("daemon unavailable");
      expect(await Bun.file(receiptMarkdownPath()).text()).toContain("Docker inspect failed: `daemon unavailable`");
    });
  });
});

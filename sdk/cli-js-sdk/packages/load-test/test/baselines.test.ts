import { copyFile, mkdtemp, mkdir, readFile, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";

import type { RequestRecord } from "../src/flows/types";
import { blessSuiteBaselines } from "../src/report/baselines";
import { buildReport } from "../src/report/build";
import { scenarioSchema } from "../src/scenario/schema";

let dir: string;

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-baselines-"));
});

afterEach(async () => {
  await rm(dir, { recursive: true, force: true });
});

const scenario = scenarioSchema.parse({
  name: "baseline-entry",
  flows: [{ flow: "input-proof", weight: 1 }],
  shape: { kind: "burst", count: 1 },
});

const report = (e2eLatencyMs = 100) => buildReport({
  scenario,
  network: "testnet",
  relayerUrl: "https://relayer.example",
  startedAt: "2026-01-01T00:00:00.000Z",
  endedAt: "2026-01-01T00:00:01.000Z",
  records: [{
    flow: "input-proof",
    index: 0,
    startedAtMs: 0,
    sentRequestId: "request",
    pollCount: 1,
    outcome: "succeeded",
    verified: true,
    e2eLatencyMs,
  } satisfies RequestRecord],
  submitted: 1,
  completed: 1,
  abandoned: 0,
  poolExhausted: false,
  submissionDurationMs: 1000,
  targets: [{ target: "A", relayerUrl: "https://relayer.example" }],
});

const writeSuite = async (options: { regressed?: boolean; badSecond?: boolean; latencyMs?: number } = {}) => {
  const suiteOutput = join(dir, "suite");
  const entries = ["entry", ...(options.badSecond ? ["bad"] : [])];
  for (const label of entries) await mkdir(join(suiteOutput, label), { recursive: true });
  await writeFile(
    join(suiteOutput, "entry", "report.json"),
    JSON.stringify(report(options.latencyMs)),
  );
  if (options.badSecond) await writeFile(join(suiteOutput, "bad", "report.json"), "{broken");
  await writeFile(join(suiteOutput, "suite-summary.json"), JSON.stringify({
    suite: "suite",
    startedAt: "2026-01-01T00:00:00.000Z",
    endedAt: "2026-01-01T00:00:02.000Z",
    outputRoot: suiteOutput,
    status: "completed",
    passed: !options.regressed,
    entries: entries.map((label) => ({
      label,
      scenarioName: "baseline-entry",
      outputDir: join(suiteOutput, label),
      submitted: 1,
      errorRate: 0,
      verifyFailed: 0,
      thresholdsPassed: true,
      targets: [{ target: "A", errorRate: 0, verifyFailed: 0 }],
      differentTerminalOutcomes: 0,
      ...(options.regressed ? { diff: { baseline: "old", passed: false, regressions: 1 } } : {}),
    })),
  }));
  return suiteOutput;
};

describe("baseline blessing", () => {
  it("publishes a completed validated suite explicitly", async () => {
    const suiteOutput = await writeSuite();
    const paths = await blessSuiteBaselines({
      suiteOutput,
      baselinesDir: join(dir, "baselines"),
    });
    expect(paths).toEqual([join(dir, "baselines", "testnet", "entry.json")]);
    expect(JSON.parse(await readFile(paths[0]!, "utf8"))).toMatchObject({ version: 1 });
  });

  it("requires explicit acceptance of regressions", async () => {
    const suiteOutput = await writeSuite({ latencyMs: 1_000 });
    const baselinesDir = join(dir, "baselines");
    await mkdir(join(baselinesDir, "testnet"), { recursive: true });
    await writeFile(join(baselinesDir, "testnet", "entry.json"), JSON.stringify(report(100)));
    await expect(blessSuiteBaselines({
      suiteOutput,
      baselinesDir,
    })).rejects.toThrow("--accept-regressions");
    await expect(blessSuiteBaselines({
      suiteOutput,
      baselinesDir,
      acceptRegressions: true,
    })).resolves.toHaveLength(1);
  });

  it.runIf(process.platform !== "win32")(
    "refuses a symlinked baseline path component instead of publishing outside the root",
    async () => {
      const suiteOutput = await writeSuite();
      const baselinesDir = join(dir, "baselines");
      const outside = join(dir, "outside");
      await mkdir(baselinesDir, { recursive: true });
      await mkdir(outside, { recursive: true });
      await symlink(outside, join(baselinesDir, "testnet"), "dir");

      await expect(blessSuiteBaselines({ suiteOutput, baselinesDir }))
        .rejects.toThrow(/symlinked artifact path/);
      await expect(readFile(join(outside, "entry.json")))
        .rejects.toMatchObject({ code: "ENOENT" });
    },
  );

  it.runIf(process.platform !== "win32")(
    "refuses a symlinked suite report instead of blessing an external artifact",
    async () => {
      const suiteOutput = await writeSuite();
      const reportPath = join(suiteOutput, "entry", "report.json");
      const outsideReport = join(dir, "outside-report.json");
      await copyFile(reportPath, outsideReport);
      await rm(reportPath);
      await symlink(outsideReport, reportPath, "file");

      await expect(blessSuiteBaselines({
        suiteOutput,
        baselinesDir: join(dir, "baselines"),
      })).rejects.toThrow(/symlinked artifact path/);
    },
  );

  it("validates every report before publishing any baseline", async () => {
    const suiteOutput = await writeSuite({ badSecond: true });
    await expect(blessSuiteBaselines({
      suiteOutput,
      baselinesDir: join(dir, "baselines"),
    })).rejects.toThrow("Could not read report JSON");
    await expect(readFile(join(dir, "baselines", "testnet", "entry.json")))
      .rejects.toMatchObject({ code: "ENOENT" });
  });

  it("refuses to overwrite a corrupt existing baseline", async () => {
    const suiteOutput = await writeSuite();
    const destination = join(dir, "baselines", "testnet", "entry.json");
    await mkdir(join(dir, "baselines", "testnet"), { recursive: true });
    await writeFile(destination, "{corrupt");
    await expect(blessSuiteBaselines({
      suiteOutput,
      baselinesDir: join(dir, "baselines"),
    })).rejects.toThrow("Could not read report JSON");
    expect(await readFile(destination, "utf8")).toBe("{corrupt");
  });
});

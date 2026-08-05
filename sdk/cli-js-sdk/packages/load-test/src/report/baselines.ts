import { constants, existsSync } from "node:fs";
import { copyFile, mkdir, open, readFile, rename, rm } from "node:fs/promises";
import { dirname, join } from "node:path";
import { randomUUID } from "node:crypto";
import { z } from "zod";

import { artifactSlugSchema, safeJoinNoSymlinks } from "../shared/paths";
import { diffReports } from "./diff";
import { readReportFile } from "./runtime";

const suiteEntrySchema = z.object({
  label: artifactSlugSchema,
  scenarioName: artifactSlugSchema,
  outputDir: z.string(),
  submitted: z.number().int().nonnegative(),
  errorRate: z.number().finite().nonnegative(),
  verifyFailed: z.number().int().nonnegative(),
  thresholdsPassed: z.boolean(),
  diff: z.object({
    baseline: z.string(),
    passed: z.boolean(),
    regressions: z.number().int().nonnegative(),
  }).strict().optional(),
}).passthrough();

const suiteStatusSchema = z.enum(["completed", "blocked", "interrupted", "failed"]);

const suiteSummarySchema = z.object({
  suite: artifactSlugSchema,
  startedAt: z.iso.datetime(),
  endedAt: z.iso.datetime(),
  outputRoot: z.string(),
  status: suiteStatusSchema,
  passed: z.boolean(),
  entries: z.array(suiteEntrySchema).min(1),
}).strict();

export type BlessBaselinesOptions = Readonly<{
  suiteOutput: string;
  baselinesDir: string;
  /** Explicitly accept valid reports whose prior baseline diff regressed. */
  acceptRegressions?: boolean;
}>;

type StagedBaseline = {
  destination: string;
  temporary: string;
  backup?: string;
  published: boolean;
};

const syncFile = async (path: string): Promise<void> => {
  const handle = await open(path, constants.O_RDONLY);
  try {
    await handle.sync();
  } finally {
    await handle.close();
  }
};

/** Validates every candidate before publishing any baseline. */
export const blessSuiteBaselines = async (
  options: BlessBaselinesOptions,
): Promise<readonly string[]> => {
  const summaryPath = await safeJoinNoSymlinks(options.suiteOutput, "suite-summary.json");
  let summaryValue: unknown;
  try {
    summaryValue = JSON.parse(await readFile(summaryPath, "utf8")) as unknown;
  } catch (error) {
    throw new Error(`Could not read completed suite summary at ${summaryPath}.`, { cause: error });
  }
  // A non-completed suite (blocked/interrupted/failed) is never bless-able and
  // may legitimately carry no entries, so reject it on status alone before the
  // strict schema runs — otherwise an empty `entries` array surfaces as a raw
  // ZodError instead of the clean "Cannot bless…" message.
  const earlyStatus = suiteStatusSchema.safeParse(
    (summaryValue as { status?: unknown } | null)?.status,
  );
  if (earlyStatus.success && earlyStatus.data !== "completed") {
    throw new Error(`Cannot bless a suite with status ${earlyStatus.data}.`);
  }
  const summary = suiteSummarySchema.parse(summaryValue);
  if (summary.status !== "completed") {
    throw new Error(`Cannot bless a suite with status ${summary.status}.`);
  }
  if (summary.entries.some((entry) => !entry.thresholdsPassed)) {
    throw new Error("Cannot bless a suite with threshold failures.");
  }
  const staged: StagedBaseline[] = [];
  let network: string | undefined;
  try {
    for (const entry of summary.entries) {
      const reportPath = await safeJoinNoSymlinks(
        options.suiteOutput,
        entry.label,
        "report.json",
      );
      const report = await readReportFile(reportPath);
      if (report.run.status !== "completed" || !report.thresholds.passed) {
        throw new Error(`Report ${reportPath} is not a completed threshold-passing report.`);
      }
      if (report.run.scenario.name !== entry.scenarioName) {
        throw new Error(`Suite entry ${entry.label} does not match its report scenario.`);
      }
      network ??= report.run.network;
      if (report.run.network !== network) {
        throw new Error("All reports in a blessed suite must target the same network.");
      }
      await mkdir(options.baselinesDir, { recursive: true });
      const destination = await safeJoinNoSymlinks(
        options.baselinesDir,
        network,
        `${entry.label}.json`,
      );
      if (existsSync(destination)) {
        const existing = await readReportFile(destination);
        const actualDiff = diffReports(existing, report);
        if (!actualDiff.passed && !options.acceptRegressions) {
          throw new Error(
            `Candidate ${entry.label} has ${actualDiff.regressions.length.toString()} actual regression(s); ` +
              "pass --accept-regressions to bless intentionally.",
          );
        }
      }
      await mkdir(dirname(destination), { recursive: true });
      await safeJoinNoSymlinks(options.baselinesDir, network, `${entry.label}.json`);
      const temporary = join(
        dirname(destination),
        `.${entry.label}.baseline-${process.pid.toString()}-${randomUUID()}.tmp`,
      );
      await copyFile(reportPath, temporary, constants.COPYFILE_EXCL);
      await syncFile(temporary);
      staged.push({ destination, temporary, published: false });
    }

    for (const item of staged) {
      if (existsSync(item.destination)) {
        item.backup = `${item.destination}.backup-${process.pid.toString()}-${randomUUID()}`;
        await rename(item.destination, item.backup);
      }
      await rename(item.temporary, item.destination);
      item.published = true;
    }
  } catch (error) {
    for (const item of [...staged].reverse()) {
      if (item.published) await rm(item.destination, { force: true });
      if (item.backup && existsSync(item.backup)) {
        await rename(item.backup, item.destination);
      }
      await rm(item.temporary, { force: true });
    }
    throw error;
  }

  await Promise.all(staged.map(async (item) => {
    if (item.backup) await rm(item.backup, { force: true });
  }));
  return staged.map((item) => item.destination);
};

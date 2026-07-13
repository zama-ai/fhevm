import type { Command } from "@commander-js/extra-typings";

import { parseBoundedNonNegativeNumber, parseNonNegativeNumber, readReport } from "../shared";

export const registerReportCommands = (program: Command): void => {
  const report = program.command("report").description("Render and compare validated run reports");

  report.command("render <report>").description("Render Markdown from a report.json")
    .action(async (reportPath) => {
      const { renderMarkdownReport } = await import("../../report/render-md");
      console.log(renderMarkdownReport(await readReport(reportPath)));
    });

  report.command("diff <baseline> <current>")
    .description("Compare compatible reports; exits non-zero on regression")
    .option(
      "--max-latency-increase <fraction>",
      "allowed relative p95/p99 latency increase over baseline, e.g. 0.2 = 20% (default 0.20)",
      parseNonNegativeNumber,
    )
    .option(
      "--max-error-rate-increase <fraction>",
      "allowed absolute error-rate increase over baseline, e.g. 0.01 = 1 percentage point; range [0, 1] (default 0.01)",
      parseBoundedNonNegativeNumber("--max-error-rate-increase", 1),
    )
    .action(async (baselinePath, currentPath, options) => {
      const [{ diffReports }, { logger }] = await Promise.all([
        import("../../report/diff"), import("../../shared/logger"),
      ]);
      const result = diffReports(await readReport(baselinePath), await readReport(currentPath), {
        latencyTolerance: options.maxLatencyIncrease as number | undefined,
        errorRateTolerance: options.maxErrorRateIncrease as number | undefined,
      });
      for (const note of result.notes) logger.warn(note);
      if (result.passed) { logger.success("No regressions."); return; }
      for (const regression of result.regressions) {
        logger.error(`${regression.flow} ${regression.metric}: ${regression.baseline.toString()} -> ${regression.current.toString()} (${(regression.relativeChange * 100).toFixed(1)}%)`);
      }
      process.exitCode = 1;
    });
};

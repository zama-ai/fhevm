import type { Command } from "@commander-js/extra-typings";

export const registerBaselineCommands = (program: Command): void => {
  const baseline = program.command("baseline").description("Explicitly manage validated baselines");

  baseline.command("bless <suite-output>")
    .description("Atomically publish every threshold-passing report from a completed suite")
    .option("--baselines-dir <dir>", "baseline reports root", "baselines")
    .option("--accept-regressions", "explicitly accept valid reports that regressed")
    .action(async (suiteOutput, options) => {
      const [{ blessSuiteBaselines }, { logger }] = await Promise.all([
        import("../../report/baselines"), import("../../shared/logger"),
      ]);
      const paths = await blessSuiteBaselines({
        suiteOutput,
        baselinesDir: options.baselinesDir,
        acceptRegressions: Boolean(options.acceptRegressions),
      });
      for (const path of paths) logger.success(`Baseline blessed: ${path}`);
    });
};

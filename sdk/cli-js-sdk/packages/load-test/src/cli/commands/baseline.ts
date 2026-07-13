import type { Command } from "@commander-js/extra-typings";

export const registerBaselineCommands = (program: Command): void => {
  const baseline = program.command("baseline").description("Explicitly manage validated baselines");

  baseline.command("list")
    .description("List stored baselines (name and last-updated time)")
    .option("--baselines-dir <dir>", "baseline reports root", "baselines")
    .action(async (options) => {
      const [{ readdir, stat }, { join }, { logger }] = await Promise.all([
        import("node:fs/promises"),
        import("node:path"),
        import("../../shared/logger"),
      ]);
      const baselinesDir = options.baselinesDir;
      let networkEntries: { name: string }[];
      try {
        networkEntries = (await readdir(baselinesDir, { withFileTypes: true }))
          .filter((entry) => entry.isDirectory());
      } catch (error) {
        if ((error as NodeJS.ErrnoException).code === "ENOENT") {
          logger.info(`No baselines found under ${baselinesDir}.`);
          return;
        }
        throw error;
      }
      let found = false;
      for (const { name: network } of networkEntries.sort((a, b) => a.name.localeCompare(b.name))) {
        const networkDir = join(baselinesDir, network);
        const files = (await readdir(networkDir, { withFileTypes: true }))
          .filter((entry) => entry.isFile() && entry.name.endsWith(".json"))
          .map((entry) => entry.name)
          .sort();
        for (const file of files) {
          found = true;
          const label = file.slice(0, -".json".length);
          const stats = await stat(join(networkDir, file));
          logger.info(`${network}/${label} — updated ${stats.mtime.toISOString()}`);
        }
      }
      if (!found) logger.info(`No baselines found under ${baselinesDir}.`);
    });

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

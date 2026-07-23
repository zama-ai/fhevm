import type { Command } from "@commander-js/extra-typings";

import { emitJson, useJsonOutput, withFormatOption } from "../shared";

export const registerBaselineCommands = (program: Command): void => {
  const baseline = program.command("baseline").description("Explicitly manage validated baselines");

  withFormatOption(baseline.command("list")
    .description("List stored baselines (name and last-updated time)")
    .option("--baselines-dir <dir>", "baseline reports root", "baselines"))
    .action(async (options) => {
      const json = await useJsonOutput(options);
      const [{ readdir, stat }, { join }, { logger }] = await Promise.all([
        import("node:fs/promises"),
        import("node:path"),
        import("../../shared/logger"),
      ]);
      const baselinesDir = options.baselinesDir;
      let networkEntries: { name: string }[] = [];
      try {
        networkEntries = (await readdir(baselinesDir, { withFileTypes: true }))
          .filter((entry) => entry.isDirectory());
      } catch (error) {
        if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
      }
      const entries: { network: string; label: string; updated: string }[] = [];
      for (const { name: network } of networkEntries.sort((a, b) => a.name.localeCompare(b.name))) {
        const networkDir = join(baselinesDir, network);
        const files = (await readdir(networkDir, { withFileTypes: true }))
          .filter((entry) => entry.isFile() && entry.name.endsWith(".json"))
          .map((entry) => entry.name)
          .sort();
        for (const file of files) {
          const label = file.slice(0, -".json".length);
          const stats = await stat(join(networkDir, file));
          entries.push({ network, label, updated: stats.mtime.toISOString() });
        }
      }
      if (json) {
        emitJson(entries);
        return;
      }
      if (entries.length === 0) {
        logger.info(`No baselines found under ${baselinesDir}.`);
        return;
      }
      for (const entry of entries) {
        logger.info(`${entry.network}/${entry.label} — updated ${entry.updated}`);
      }
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

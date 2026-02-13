import type { Command } from "commander";
import { log } from "../log.js";
import { dockerExec } from "../docker.js";
import { TEST_SUITES, SUITE_NAMES } from "../test-suites.js";

export function registerTestCommand(program: Command): void {
  program
    .command("test")
    .argument("<suite>", `Test suite to run (${SUITE_NAMES.join("|")}|debug)`)
    .description("Run tests against the deployed FHEVM stack")
    .option("-v, --verbose", "Enable verbose output")
    .option("-n, --network <name>", "Specify network", "staging")
    .option("-g, --grep <pattern>", "Override default test grep pattern")
    .option("-r, --no-relayer", "Disable Rust relayer")
    .option("--no-hardhat-compile", "Skip Hardhat compilation step")
    .option("--parallel", "Run tests in parallel")
    .action(async (suite: string, opts) => {
      // Debug mode: launch interactive shell
      if (suite === "debug") {
        log.info("Starting debug session...");
        const exitCode = await dockerExec(
          "fhevm-test-suite-e2e-debug",
          ["bash"],
          { interactive: true },
        );
        process.exit(exitCode);
      }

      // Look up the suite
      const suiteConfig = TEST_SUITES[suite];
      if (!suiteConfig && !opts.grep) {
        log.error(`Unknown test suite: ${suite}`);
        log.error(`Valid suites are: ${SUITE_NAMES.join(", ")}, debug`);
        process.exit(1);
      }

      // Build docker exec args
      const dockerArgs: string[] = ["./run-tests.sh"];

      if (opts.verbose) dockerArgs.push("-v");
      dockerArgs.push("-n", opts.network);
      if (!opts.relayer) dockerArgs.push("-r");
      if (!opts.hardhatCompile) dockerArgs.push("--no-hardhat-compile");

      // Use the explicit --grep if provided, otherwise use the suite's default
      const useParallel = opts.parallel || suiteConfig?.parallel;
      if (useParallel) dockerArgs.push("--parallel");

      const grepPattern = opts.grep ?? suiteConfig?.grep;
      if (grepPattern) {
        dockerArgs.push("-g", grepPattern);
      }

      const label = suiteConfig?.label ?? suite.toUpperCase();
      log.info(`[TEST] ${label}`);

      const exitCode = await dockerExec(
        "fhevm-test-suite-e2e-debug",
        dockerArgs,
      );

      if (exitCode !== 0) {
        process.exit(exitCode);
      }
    });
}

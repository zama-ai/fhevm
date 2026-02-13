import type { Command } from "commander";
import { log } from "../log.js";
import { getContainerState } from "../docker.js";
import { STEPS } from "../dag.js";
import chalk from "chalk";

export function registerStatusCommand(program: Command): void {
  program
    .command("status")
    .description("Show status of all FHEVM stack containers")
    .action(async () => {
      log.info("Checking FHEVM stack status...\n");

      const rows: { step: string; container: string; status: string; detail: string }[] = [];

      for (const step of STEPS) {
        if (step.services.length === 0) {
          rows.push({
            step: step.id,
            container: "(script-only)",
            status: "-",
            detail: "",
          });
          continue;
        }

        for (const svc of step.services) {
          const state = await getContainerState(svc.container);

          if (!state) {
            rows.push({
              step: step.id,
              container: svc.container,
              status: chalk.gray("not found"),
              detail: "",
            });
            continue;
          }

          let statusStr: string;
          let detail = "";

          if (state.OOMKilled) {
            statusStr = chalk.red("OOM-killed");
          } else if (state.Status === "running") {
            statusStr = chalk.green("running");
          } else if (state.Status === "exited" && state.ExitCode === 0) {
            statusStr = chalk.green("exited (0)");
          } else if (state.Status === "exited") {
            statusStr = chalk.red(`exited (${state.ExitCode})`);
          } else {
            statusStr = chalk.yellow(state.Status);
          }

          rows.push({
            step: step.id,
            container: svc.container,
            status: statusStr,
            detail,
          });
        }
      }

      // Print table
      const stepW = Math.max(6, ...rows.map((r) => r.step.length));
      const containerW = Math.max(10, ...rows.map((r) => r.container.length));

      const header = `${"STEP".padEnd(stepW)}  ${"CONTAINER".padEnd(containerW)}  STATUS`;
      console.log(chalk.bold(header));
      console.log("-".repeat(header.length + 10));

      for (const row of rows) {
        console.log(
          `${row.step.padEnd(stepW)}  ${row.container.padEnd(containerW)}  ${row.status}${row.detail ? "  " + row.detail : ""}`,
        );
      }

      // Check for OOM-killed containers
      const oomContainers = rows.filter((r) => r.status.includes("OOM"));
      if (oomContainers.length > 0) {
        console.log();
        log.warn(`${oomContainers.length} container(s) were OOM-killed! Consider increasing Docker memory.`);
      }
    });
}

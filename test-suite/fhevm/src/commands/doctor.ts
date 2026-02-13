import type { Command } from "commander";
import { log } from "../log.js";
import { getContainerState } from "../docker.js";
import { STEPS } from "../dag.js";
import { createConnection } from "net";

const REQUIRED_PORTS = [8545, 8546, 9000, 9001, 5432, 5433, 3000, 50051];

export function registerDoctorCommand(program: Command): void {
  program
    .command("doctor")
    .description("Run pre-flight checks for the FHEVM stack")
    .action(async () => {
      let allPassed = true;

      // 1. Docker daemon reachable
      log.info("Checking Docker daemon...");
      const dockerInfoProc = Bun.spawn(["docker", "info"], {
        stdout: "pipe",
        stderr: "pipe",
      });
      const dockerInfoExit = await dockerInfoProc.exited;
      if (dockerInfoExit !== 0) {
        log.error("Docker daemon is not reachable. Is Docker running?");
        process.exit(1);
      }
      log.success("Docker daemon is reachable");

      // 2. Docker Compose v2 available
      log.info("Checking Docker Compose...");
      const composeProc = Bun.spawn(["docker", "compose", "version"], {
        stdout: "pipe",
        stderr: "pipe",
      });
      const composeVersion = (await new Response(composeProc.stdout).text()).trim();
      const composeExit = await composeProc.exited;
      if (composeExit !== 0) {
        log.error("Docker Compose v2 is not available. Please install it.");
        allPassed = false;
      } else {
        log.success(`Docker Compose: ${composeVersion}`);
      }

      // 3. Docker memory check
      log.info("Checking Docker memory...");
      const memProc = Bun.spawn(
        ["docker", "info", "--format", "{{.MemTotal}}"],
        { stdout: "pipe", stderr: "pipe" },
      );
      const memStr = (await new Response(memProc.stdout).text()).trim();
      await memProc.exited;
      const memBytes = parseInt(memStr, 10);
      if (!isNaN(memBytes)) {
        const memGiB = memBytes / (1024 ** 3);
        if (memGiB < 12) {
          log.error(`Docker memory: ${memGiB.toFixed(1)} GiB — minimum 12 GiB required`);
          allPassed = false;
        } else if (memGiB < 16) {
          log.warn(`Docker memory: ${memGiB.toFixed(1)} GiB — 16 GiB recommended`);
        } else {
          log.success(`Docker memory: ${memGiB.toFixed(1)} GiB`);
        }
      } else {
        log.warn("Could not determine Docker memory allocation");
      }

      // 4. Required ports free
      log.info("Checking required ports...");
      for (const port of REQUIRED_PORTS) {
        const inUse = await isPortInUse(port);
        if (inUse) {
          log.warn(`Port ${port} is already in use`);
          allPassed = false;
        } else {
          log.success(`Port ${port} is available`);
        }
      }

      // 5. If stack is running, check for OOM-killed containers
      log.info("Checking running stack health...");
      let oomCount = 0;
      for (const step of STEPS) {
        for (const svc of step.services) {
          const state = await getContainerState(svc.container);
          if (state?.OOMKilled) {
            log.error(`${svc.container} was OOM-killed!`);
            oomCount++;
          }
        }
      }
      if (oomCount > 0) {
        log.error(`${oomCount} container(s) were OOM-killed. Increase Docker memory.`);
        allPassed = false;
      } else {
        log.success("No OOM-killed containers detected");
      }

      // Summary
      console.log();
      if (allPassed) {
        log.success("All pre-flight checks passed!");
      } else {
        log.warn("Some checks failed. Please address the issues above before deploying.");
        process.exit(1);
      }
    });
}

/** Check if a TCP port is in use by attempting a connection */
function isPortInUse(port: number): Promise<boolean> {
  return new Promise((resolve) => {
    const socket = createConnection({ port, host: "127.0.0.1" });
    socket.setTimeout(1000);

    socket.on("connect", () => {
      socket.destroy();
      resolve(true);
    });

    socket.on("timeout", () => {
      socket.destroy();
      resolve(false);
    });

    socket.on("error", () => {
      resolve(false);
    });
  });
}

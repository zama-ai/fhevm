import path from "node:path";
import { UPGRADE_SERVICES } from "./manifest";
import type { CommandDeps } from "./command-contracts";

export function createAdminHandlers(deps: CommandDeps) {
  const {
    PROJECT,
    COMPOSE_DIR,
    COLORS,
    runCommand,
    usageError,
    cliError,
    logWarn,
    readContainerLogs,
    detectExpectedPause,
    detectEnforcedPause,
    loadActiveVersionsIfPresent,
    localEnvFile,
    composeFile,
  } = deps;

  function pauseOrUnpause(command: "pause" | "unpause", contractsArg?: string): void {
    if (contractsArg !== "gateway" && contractsArg !== "host") {
      usageError(`Unknown service: ${contractsArg ?? ""}`);
    }

    const action = command === "pause" ? "PAUSE" : "UNPAUSE";
    const composePath = path.resolve(COMPOSE_DIR, `${contractsArg}-${command}-docker-compose.yml`);
    const waitService = `${contractsArg}-sc-${command}`;

    console.log(`${COLORS.lightBlue}[${action}]${COLORS.reset} ${COLORS.bold}${command === "pause" ? "Pausing" : "Unpausing"} ${contractsArg}...${COLORS.reset}`);
    runCommand(["docker", "compose", "-p", PROJECT, "-f", composePath, "up", "-d"], { check: true });
    console.log(`${COLORS.yellow}[WAIT]${COLORS.reset} ${COLORS.bold}Waiting for ${command} operation to complete...${COLORS.reset}`);
    const waitResult = runCommand(["docker", "compose", "-p", PROJECT, "-f", composePath, "wait", waitService], { capture: true, check: false, allowFailure: true });
    if (waitResult.status !== 0) {
      const logs = readContainerLogs(waitService);
      const alreadyInRequestedState =
        (command === "unpause" && detectExpectedPause(logs)) || (command === "pause" && detectEnforcedPause(logs));

      if (!alreadyInRequestedState) {
        const output = [waitResult.stdout.trim(), waitResult.stderr.trim(), logs.trim()].filter(Boolean).join("\n");
        throw new Error(`Failed to ${command} ${contractsArg}${output ? `\n${output}` : ""}`);
      }

      const stateLabel = command === "unpause" ? "already unpaused" : "already paused";
      logWarn(`${contractsArg} contracts are ${stateLabel}; treating ${command} as successful.`);
    }
    console.log(`${COLORS.green}[SUCCESS]${COLORS.reset} ${COLORS.bold}${contractsArg} ${command}d successfully${COLORS.reset}`);
  }

  function upgrade(service?: string): void {
    if (!service || !UPGRADE_SERVICES.includes(service)) {
      usageError(`Unknown service: ${service ?? ""}`);
    }

    loadActiveVersionsIfPresent();

    const envFile = localEnvFile(service);
    const compose = composeFile(service);

    console.log(`${COLORS.lightBlue}[UPGRADE]${COLORS.reset} ${COLORS.bold}Upgrading ${service}...${COLORS.reset}`);
    runCommand(["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "up", "-d"], { check: true });
    console.log(`${COLORS.green}[SUCCESS]${COLORS.reset} ${COLORS.bold}${service} upgraded successfully${COLORS.reset}`);
  }

  function logs(service?: string): void {
    if (!service) {
      usageError("Service name is required");
    }

    const projectContainers = runCommand(
      ["docker", "ps", "-a", "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Names}}"],
      { capture: true, check: true },
    ).stdout
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);

    const aliases: Record<string, string[]> = {
      relayer: ["fhevm-relayer"],
      "test-suite": ["fhevm-test-suite-e2e-debug"],
      minio: ["fhevm-minio"],
      core: ["kms-core"],
      database: ["coprocessor-and-kms-db"],
    };

    const candidates = [service, ...(aliases[service] ?? [])];
    let container = candidates.find((candidate) => projectContainers.includes(candidate));

    if (!container) {
      container = candidates
        .flatMap((candidate) => projectContainers.filter((name) => name.includes(candidate)))
        .find(Boolean);
    }

    if (!container) {
      const available = projectContainers.length === 0 ? "none" : projectContainers.join(", ");
      cliError("E_LOGS_CONTAINER_NOT_FOUND", `No container found for '${service}' in project '${PROJECT}'. Available: ${available}`);
    }

    console.log(`${COLORS.lightBlue}[LOGS]${COLORS.reset} ${COLORS.bold}Showing logs for ${container}...${COLORS.reset}`);
    runCommand(["docker", "logs", container], { check: true });
  }

  return {
    pauseOrUnpause,
    upgrade,
    logs,
  };
}

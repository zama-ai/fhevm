import path from "node:path";
import type { CommandDeps } from "./command-contracts";
import type { CleanOptions } from "./types";

function parseCleanArgs(args: string[], usageError: (message: string) => never): CleanOptions {
  const options: CleanOptions = {
    purgeImages: false,
    purgeBuildCache: false,
    purgeNetworks: false,
    allFhevmProjects: false,
  };

  for (const arg of args) {
    if (arg === "--purge") {
      options.purgeImages = true;
      options.purgeBuildCache = true;
      options.purgeNetworks = true;
      continue;
    }
    if (arg === "--purge-images") {
      options.purgeImages = true;
      continue;
    }
    if (arg === "--purge-build-cache") {
      options.purgeBuildCache = true;
      continue;
    }
    if (arg === "--purge-networks") {
      options.purgeNetworks = true;
      continue;
    }
    if (arg === "--purge-local-cache") {
      options.purgeBuildCache = true;
      continue;
    }
    if (arg === "--all-fhevm-projects") {
      options.allFhevmProjects = true;
      continue;
    }
    usageError(`Unknown option for clean: ${arg}`);
  }

  return options;
}

function listComposeProjects(runCommand: CommandDeps["runCommand"]): string[] {
  const result = runCommand(["docker", "compose", "ls", "--format", "json"], {
    capture: true,
    check: false,
    allowFailure: true,
  });
  if (result.status !== 0 || !result.stdout.trim()) {
    return [];
  }
  try {
    const parsed = JSON.parse(result.stdout) as Array<{ Name?: string }>;
    if (!Array.isArray(parsed)) {
      return [];
    }
    return parsed
      .map((entry) => entry?.Name?.trim() ?? "")
      .filter((name) => name !== "");
  } catch {
    return [];
  }
}

function removeDockerResourcesByPrefix(
  runCommand: CommandDeps["runCommand"],
  prefix: string,
  removeVolumes: boolean,
): void {
  const containerNames = runCommand(["docker", "ps", "-a", "--format", "{{.Names}}"], {
    capture: true,
    check: false,
    allowFailure: true,
  }).stdout
    .split("\n")
    .map((line) => line.trim())
    .filter((name) => name.startsWith(prefix));
  if (containerNames.length > 0) {
    runCommand(["docker", "rm", "-f", ...containerNames], {
      check: false,
      allowFailure: true,
    });
  }

  const networks = runCommand(["docker", "network", "ls", "--format", "{{.Name}}"], {
    capture: true,
    check: false,
    allowFailure: true,
  }).stdout
    .split("\n")
    .map((line) => line.trim())
    .filter((name) => name.startsWith(prefix));
  for (const network of networks) {
    runCommand(["docker", "network", "rm", network], {
      check: false,
      allowFailure: true,
    });
  }

  if (removeVolumes) {
    const volumes = runCommand(["docker", "volume", "ls", "--format", "{{.Name}}"], {
      capture: true,
      check: false,
      allowFailure: true,
    }).stdout
      .split("\n")
      .map((line) => line.trim())
      .filter((name) => name.startsWith(prefix));
    for (const volume of volumes) {
      runCommand(["docker", "volume", "rm", volume], {
        check: false,
        allowFailure: true,
      });
    }
  }
}

function cleanupAllFhevmProjects(
  runCommand: CommandDeps["runCommand"],
  logInfo: (message: string) => void,
  removeVolumes: boolean,
): void {
  const prefix = "fhevm-";
  const projects = listComposeProjects(runCommand).filter((name) => name.startsWith(prefix));
  if (projects.length > 0) {
    logInfo(`Cleaning all fhevm compose projects: ${projects.join(", ")}`);
  } else {
    logInfo("No active fhevm compose projects found; cleaning prefixed leftovers.");
  }

  for (const project of projects) {
    const containerIds = runCommand(
      ["docker", "ps", "-a", "--filter", `label=com.docker.compose.project=${project}`, "--format", "{{.ID}}"],
      { capture: true, check: false, allowFailure: true },
    ).stdout
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);
    if (containerIds.length > 0) {
      runCommand(["docker", "rm", "-f", ...containerIds], {
        check: false,
        allowFailure: true,
      });
    }

    const networkIds = runCommand(
      ["docker", "network", "ls", "--filter", `label=com.docker.compose.project=${project}`, "--format", "{{.ID}}"],
      { capture: true, check: false, allowFailure: true },
    ).stdout
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);
    for (const networkId of networkIds) {
      runCommand(["docker", "network", "rm", networkId], {
        check: false,
        allowFailure: true,
      });
    }

    if (removeVolumes) {
      const volumeNames = runCommand(
        ["docker", "volume", "ls", "--filter", `label=com.docker.compose.project=${project}`, "--format", "{{.Name}}"],
        { capture: true, check: false, allowFailure: true },
      ).stdout
        .split("\n")
        .map((line) => line.trim())
        .filter(Boolean);
      for (const volumeName of volumeNames) {
        runCommand(["docker", "volume", "rm", volumeName], {
          check: false,
          allowFailure: true,
        });
      }
    }
  }

  removeDockerResourcesByPrefix(runCommand, prefix, removeVolumes);
}

export function createCleanHandlers(deps: CommandDeps) {
  const {
    PROJECT,
    COMPOSE_DIR,
    COLORS,
    runCommand,
    usageError,
    logInfo,
    cleanupKnownStack,
    purgeProjectImages,
    purgeLocalBuildxCache,
  } = deps;

  function cleanupTracingStack(): void {
    const tracingCompose = path.resolve(COMPOSE_DIR, "tracing-docker-compose.yml");
    runCommand(["docker", "compose", "-p", PROJECT, "-f", tracingCompose, "down", "-v"], {
      check: false,
      allowFailure: true,
    });
  }

  function clean(args: string[]): void {
    const options = parseCleanArgs(args, usageError);
    console.log(`${COLORS.lightBlue}[CLEAN]${COLORS.reset} ${COLORS.bold}Cleaning up FHEVM stack...${COLORS.reset}`);
    if (options.allFhevmProjects) {
      cleanupAllFhevmProjects(runCommand, logInfo, true);
    } else {
      cleanupKnownStack(true);
      cleanupTracingStack();
    }

    if (options.purgeNetworks) {
      const networkList = runCommand(["docker", "network", "ls", "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Name}}"], {
        capture: true,
        check: true,
      });
      for (const network of networkList.stdout.split("\n").map((line) => line.trim()).filter(Boolean)) {
        runCommand(["docker", "network", "rm", network], { check: false, allowFailure: true });
      }
    }

    if (options.purgeImages) {
      logInfo("Removing images referenced by fhevm compose services only.");
      purgeProjectImages();
    }

    if (options.purgeBuildCache) {
      logInfo("Removing local fhevm Buildx cache directory.");
      purgeLocalBuildxCache();
    }

    console.log(`${COLORS.green}[SUCCESS]${COLORS.reset} ${COLORS.bold}FHEVM stack cleaned successfully${COLORS.reset}`);
  }

  return { clean };
}

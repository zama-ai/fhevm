import { log } from "./log.js";
import { composeFile, localEnvFile, PROJECT_NAME } from "./paths.js";
import { spawn, isDryRun } from "./executor.js";

interface ComposeUpOpts {
  component: string;
  build?: boolean;
  env?: Record<string, string>;
}

interface ComposeDownOpts {
  component: string;
  volumes?: boolean;
  removeOrphans?: boolean;
}

/** Run `docker compose up -d` for a component */
export async function composeUp(opts: ComposeUpOpts): Promise<void> {
  const args = [
    "docker", "compose",
    "-p", PROJECT_NAME,
    "--env-file", localEnvFile(opts.component),
    "-f", composeFile(opts.component),
    "up", "-d",
  ];
  if (opts.build) args.push("--build");

  const proc = spawn(args, {
    stdio: ["inherit", "inherit", "inherit"],
    env: { ...process.env, ...opts.env },
  });
  const exitCode = await proc.exited;
  if (exitCode !== 0) {
    throw new Error(`docker compose up failed for ${opts.component} (exit ${exitCode})`);
  }
}

/** Run `docker compose down` for a component */
export async function composeDown(opts: ComposeDownOpts): Promise<void> {
  const envPath = localEnvFile(opts.component);
  // Fall back to base env file if local doesn't exist
  const { existsSync } = await import("fs");
  const { envFile } = await import("./paths.js");
  const resolvedEnv = existsSync(envPath) ? envPath : envFile(opts.component);

  const args = [
    "docker", "compose",
    "-p", PROJECT_NAME,
    "--env-file", resolvedEnv,
    "-f", composeFile(opts.component),
    "down",
  ];
  if (opts.volumes) args.push("-v");
  if (opts.removeOrphans) args.push("--remove-orphans");

  const proc = spawn(args, {
    stdio: ["inherit", "inherit", "inherit"],
  });
  const exitCode = await proc.exited;
  // Don't throw on down failures — best effort cleanup
  if (exitCode !== 0) {
    log.warn(`docker compose down returned exit code ${exitCode} for ${opts.component}`);
  }
}

/** Run `docker compose -p fhevm down` (project-wide, no specific component) */
export async function composeDownAll(opts?: { volumes?: boolean; removeOrphans?: boolean }): Promise<void> {
  const args = ["docker", "compose", "-p", PROJECT_NAME, "down"];
  if (opts?.volumes) args.push("-v");
  if (opts?.removeOrphans) args.push("--remove-orphans");

  const proc = spawn(args, {
    stdio: ["inherit", "inherit", "inherit"],
  });
  await proc.exited;
}

export type ServiceExpect = "running" | "complete";

/**
 * Wait for a container to reach the expected state.
 * "running" — container must be in running state.
 * "complete" — container must have exited with code 0.
 *
 * In dry-run mode, immediately logs and returns.
 */
export async function waitForService(
  containerName: string,
  expect: ServiceExpect,
  maxRetries = 30,
  intervalMs = 5000,
): Promise<void> {
  if (isDryRun()) {
    log.info(`[dry-run] wait ${containerName} → ${expect}`);
    return;
  }

  const expectRunning = expect === "running";

  if (expectRunning) {
    log.info(`Waiting for ${containerName} to be running...`);
  } else {
    log.info(`Waiting for ${containerName} to complete...`);
  }

  for (let i = 1; i <= maxRetries; i++) {
    // Find the container
    const psProc = spawn(
      ["docker", "ps", "-a", "--filter", `name=${containerName}$`, "--format", "{{.ID}}"],
      { stdout: "pipe", stderr: "pipe" },
    );
    const containerId = (await new Response(psProc.stdout!).text()).trim();
    await psProc.exited;

    if (!containerId) {
      log.warn(`Container for ${containerName} not found, waiting...`);
      await Bun.sleep(intervalMs);
      continue;
    }

    // Inspect the container state
    const inspectProc = spawn(
      ["docker", "inspect", "--format", "{{json .State}}", containerId],
      { stdout: "pipe", stderr: "pipe" },
    );
    const stateJson = (await new Response(inspectProc.stdout!).text()).trim();
    await inspectProc.exited;

    let state: { Status: string; ExitCode: number; OOMKilled?: boolean };
    try {
      state = JSON.parse(stateJson);
    } catch {
      log.warn(`Failed to parse container state for ${containerName}, retrying...`);
      await Bun.sleep(intervalMs);
      continue;
    }

    // Check OOM
    if (state.OOMKilled) {
      const logsText = await getContainerLogs(containerId);
      throw new Error(
        `${containerName} was OOM-killed!\n${logsText}`,
      );
    }

    // Check expected state
    if (expectRunning && state.Status === "running") {
      log.info(`${containerName} is now running`);
      return;
    }

    if (!expectRunning && state.Status === "exited" && state.ExitCode === 0) {
      log.info(`${containerName} completed successfully`);
      return;
    }

    if (state.Status === "exited" && state.ExitCode !== 0) {
      const logsText = await getContainerLogs(containerId);
      throw new Error(
        `${containerName} failed with exit code ${state.ExitCode}\n${logsText}`,
      );
    }

    // Still waiting
    if (i < maxRetries) {
      log.warn(
        `${containerName} not ready yet (status: ${state.Status}), waiting ${intervalMs / 1000}s... (${i}/${maxRetries})`,
      );
      await Bun.sleep(intervalMs);
    } else {
      const logsText = await getContainerLogs(containerId);
      throw new Error(
        `${containerName} failed to reach desired state within the expected time\n${logsText}`,
      );
    }
  }
}

/** Get the IP address of a running container */
export async function getContainerIp(containerName: string): Promise<string> {
  if (isDryRun()) {
    log.info(`[dry-run] getContainerIp(${containerName}) → 172.17.0.2`);
    return "172.17.0.2";
  }

  const proc = spawn(
    [
      "docker", "inspect", "-f",
      "{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}",
      containerName,
    ],
    { stdout: "pipe", stderr: "pipe" },
  );
  const ip = (await new Response(proc.stdout!).text()).trim();
  const exitCode = await proc.exited;

  if (exitCode !== 0 || !ip) {
    throw new Error(`Could not find IP address for ${containerName} container`);
  }

  return ip;
}

/** Execute a command inside a running container */
export async function dockerExec(
  containerName: string,
  cmd: string[],
  opts?: { interactive?: boolean },
): Promise<number> {
  const args = ["docker", "exec"];
  if (opts?.interactive) args.push("-it");
  args.push(containerName, ...cmd);

  const proc = spawn(args, {
    stdio: ["inherit", "inherit", "inherit"],
  });
  return proc.exited;
}

/** Get logs from a container */
async function getContainerLogs(containerId: string): Promise<string> {
  const proc = spawn(
    ["docker", "logs", containerId],
    { stdout: "pipe", stderr: "pipe" },
  );
  const stdout = await new Response(proc.stdout!).text();
  const stderr = await new Response(proc.stderr!).text();
  await proc.exited;
  return stdout + stderr;
}

/** Check if a container is currently running */
export async function isContainerRunning(containerName: string): Promise<boolean> {
  if (isDryRun()) {
    log.info(`[dry-run] isContainerRunning(${containerName}) → false`);
    return false;
  }

  const proc = spawn(
    ["docker", "ps", "--filter", `name=${containerName}`, "--format", "{{.Names}}"],
    { stdout: "pipe", stderr: "pipe" },
  );
  const output = (await new Response(proc.stdout!).text()).trim();
  await proc.exited;
  return output.includes(containerName);
}

/** Get container state via docker inspect, returns null if container not found */
export async function getContainerState(
  containerName: string,
): Promise<{ Status: string; ExitCode: number; OOMKilled: boolean } | null> {
  if (isDryRun()) {
    return null;
  }

  const proc = spawn(
    ["docker", "inspect", "--format", "{{json .State}}", containerName],
    { stdout: "pipe", stderr: "pipe" },
  );
  const output = (await new Response(proc.stdout!).text()).trim();
  const exitCode = await proc.exited;

  if (exitCode !== 0) return null;

  try {
    return JSON.parse(output);
  } catch {
    return null;
  }
}

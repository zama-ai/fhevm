import { mkdir } from "fs/promises";
import { join } from "path";

import type { DotFhevmPaths } from "../config/dotfhevm";
import { getContainerLogs, listProjectContainers } from "../docker/containers";
import { DOCKER_PROJECT } from "../docker/types";

interface DiagnosticsOps {
  listProjectContainers: typeof listProjectContainers;
  getContainerLogs: typeof getContainerLogs;
}

const DEFAULT_OPS: DiagnosticsOps = {
  listProjectContainers,
  getContainerLogs,
};

let ops: DiagnosticsOps = DEFAULT_OPS;

export async function captureDiagnosticLogs(
  paths: DotFhevmPaths,
  options: { tailLines?: number } = {},
): Promise<string[]> {
  const containers = await ops.listProjectContainers(DOCKER_PROJECT, { all: false });
  const running = containers.filter((container) => container.state === "running");

  if (running.length === 0) {
    return [];
  }

  await mkdir(paths.logs, { recursive: true });

  const tail = options.tailLines ?? 200;
  return Promise.all(
    running.map(async (container) => {
      const logs = await ops.getContainerLogs(container.name, { tail });
      const filePath = join(paths.logs, `${container.service}.log`);
      await Bun.write(filePath, logs.length > 0 ? `${logs}\n` : "");
      return filePath;
    }),
  );
}

export const __internal = {
  resetOpsForTests(): void {
    ops = DEFAULT_OPS;
  },
  setOpsForTests(overrides: Partial<DiagnosticsOps>): void {
    ops = { ...DEFAULT_OPS, ...overrides };
  },
};

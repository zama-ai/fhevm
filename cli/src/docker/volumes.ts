import { mkdir } from "fs/promises";
import { resolve } from "path";

import { ExitCode, FhevmCliError } from "../errors";
import { toLogLines } from "./logs";
import { exec, type ShellResult } from "../utils/shell";

const VOLUME_NAME_PATTERN = /^[a-zA-Z0-9][a-zA-Z0-9_.-]*$/;

interface VolumeOps {
  exec: typeof exec;
}

const DEFAULT_OPS: VolumeOps = { exec };
let ops: VolumeOps = DEFAULT_OPS;

function ensureVolumeName(volumeName: string): void {
  if (VOLUME_NAME_PATTERN.test(volumeName)) {
    return;
  }

  throw new FhevmCliError({
    exitCode: ExitCode.CONFIG,
    step: "docker-volumes",
    message: `invalid docker volume name: ${volumeName}`,
  });
}

function toDockerError(message: string, volumeName: string, result: ShellResult): FhevmCliError {
  const output = result.stderr || result.stdout;
  return new FhevmCliError({
    exitCode: ExitCode.DOCKER,
    step: "docker-volumes",
    service: volumeName,
    message: `${message} (exit ${result.exitCode})`,
    logLines: toLogLines(output, 20),
  });
}

export async function exportVolume(volumeName: string, destPath: string): Promise<void> {
  ensureVolumeName(volumeName);
  const destination = resolve(destPath);
  await mkdir(destination, { recursive: true });

  const result = await ops.exec([
    "docker",
    "run",
    "--rm",
    "-v",
    `${volumeName}:/source`,
    "-v",
    `${destination}:/dest`,
    "alpine",
    "sh",
    "-c",
    "cp -a /source/. /dest/",
  ]);

  if (result.exitCode !== 0) {
    throw toDockerError(`failed to export volume ${volumeName}`, volumeName, result);
  }
}

export async function importVolume(sourcePath: string, volumeName: string): Promise<void> {
  ensureVolumeName(volumeName);
  const source = resolve(sourcePath);
  await mkdir(source, { recursive: true });

  const result = await ops.exec([
    "docker",
    "run",
    "--rm",
    "-v",
    `${source}:/source`,
    "-v",
    `${volumeName}:/dest`,
    "alpine",
    "sh",
    "-c",
    "cp -a /source/. /dest/",
  ]);

  if (result.exitCode !== 0) {
    throw toDockerError(`failed to import volume ${volumeName}`, volumeName, result);
  }
}

export async function volumeExists(volumeName: string): Promise<boolean> {
  ensureVolumeName(volumeName);
  const result = await ops.exec(["docker", "volume", "inspect", volumeName]);
  return result.exitCode === 0;
}

export const __internal = {
  isValidVolumeName(name: string): boolean {
    return VOLUME_NAME_PATTERN.test(name);
  },
  resetOpsForTests(): void {
    ops = DEFAULT_OPS;
  },
  setOpsForTests(overrides: Partial<VolumeOps>): void {
    ops = { ...DEFAULT_OPS, ...overrides };
  },
};

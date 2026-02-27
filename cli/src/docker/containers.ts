import { exec } from "../utils/shell";

import type { ContainerInfo, ContainerState } from "./types";

interface ComposePsRow {
  Name?: string;
  Service?: string;
  State?: string;
  Status?: string;
  RunningFor?: string;
  Health?: string;
  ExitCode?: number | string;
  Publishers?: Array<{ URL?: string; PublishedPort?: number; TargetPort?: number; Protocol?: string }>;
}

const DEFAULT_LOG_TAIL = 20;
const NETWORK_NAME_PATTERN = /^[a-zA-Z0-9][a-zA-Z0-9_.-]*$/;

function normalizeContainerState(state: string | undefined): ContainerState {
  if (!state) {
    return "not-found";
  }

  const normalized = state.trim().toLowerCase().split(/[\s(]/)[0] ?? "";
  switch (normalized) {
    case "running":
    case "exited":
    case "restarting":
    case "paused":
    case "dead":
    case "created":
    case "removing":
      return normalized;
    default:
      return "not-found";
  }
}

function normalizeHealth(raw: string | undefined): ContainerInfo["health"] {
  if (!raw) {
    return "none";
  }

  const health = raw.trim().toLowerCase();
  if (health === "healthy" || health === "unhealthy" || health === "starting") {
    return health;
  }

  return "none";
}

function formatPublishers(row: ComposePsRow): string | undefined {
  if (!row.Publishers?.length) {
    return undefined;
  }

  return row.Publishers.map((publisher) => {
    const host = publisher.URL ?? "0.0.0.0";
    const published = publisher.PublishedPort ?? 0;
    const target = publisher.TargetPort ?? 0;
    const protocol = publisher.Protocol ?? "tcp";
    return `${host}:${published}->${target}/${protocol}`;
  }).join(",");
}

function normalizeUptime(row: ComposePsRow): string | undefined {
  const runningFor = row.RunningFor?.trim();
  if (runningFor) {
    return runningFor;
  }

  const match = row.Status?.trim().match(/^up\s+(.+)$/i);
  if (!match?.[1]) {
    return undefined;
  }

  return match[1].split(" (")[0]?.trim() || undefined;
}

function parseSingleRow(raw: string): ComposePsRow | undefined {
  const trimmed = raw.trim();
  if (!trimmed) {
    return undefined;
  }

  try {
    return JSON.parse(trimmed) as ComposePsRow;
  } catch {
    return undefined;
  }
}

function isValidNetworkName(network: string): boolean {
  return NETWORK_NAME_PATTERN.test(network);
}

export function parseComposePsOutput(raw: string): ContainerInfo[] {
  const trimmed = raw.trim();
  if (!trimmed) {
    return [];
  }

  let rows: ComposePsRow[] = [];

  if (trimmed.startsWith("[")) {
    try {
      rows = JSON.parse(trimmed) as ComposePsRow[];
    } catch {
      return [];
    }
  } else {
    rows = trimmed
      .split("\n")
      .map((line) => parseSingleRow(line))
      .filter((row): row is ComposePsRow => Boolean(row));
  }

  const parsed: ContainerInfo[] = [];
  for (const row of rows) {
    const name = row.Name?.trim();
    const service = row.Service?.trim() ?? name;
    if (!name || !service) {
      continue;
    }

    const state = normalizeContainerState(row.State ?? row.Status);
    const health = normalizeHealth(row.Health);
    const exitCode =
      typeof row.ExitCode === "string"
        ? Number.parseInt(row.ExitCode, 10)
        : typeof row.ExitCode === "number"
          ? row.ExitCode
          : undefined;

    parsed.push({
      name,
      service,
      state,
      health,
      exitCode: Number.isFinite(exitCode) ? exitCode : undefined,
      ports: formatPublishers(row),
      uptime: normalizeUptime(row),
    });
  }

  return parsed;
}

export async function getContainerState(containerName: string): Promise<ContainerState> {
  const result = await exec([
    "docker",
    "inspect",
    "--format",
    "{{.State.Status}}",
    containerName,
  ]);
  if (result.exitCode !== 0) {
    return "not-found";
  }

  return normalizeContainerState(result.stdout);
}

export async function getContainerExitCode(containerName: string): Promise<number | undefined> {
  const state = await getContainerState(containerName);
  if (state !== "exited" && state !== "dead") {
    return undefined;
  }

  const result = await exec([
    "docker",
    "inspect",
    "--format",
    "{{.State.ExitCode}}",
    containerName,
  ]);
  if (result.exitCode !== 0) {
    return undefined;
  }

  const value = Number.parseInt(result.stdout, 10);
  return Number.isFinite(value) ? value : undefined;
}

export async function getContainerIp(
  containerName: string,
  network?: string,
): Promise<string | undefined> {
  if (network && !isValidNetworkName(network)) {
    return undefined;
  }

  const format = network
    ? `{{with index .NetworkSettings.Networks \"${network}\"}}{{.IPAddress}}{{end}}`
    : "{{range .NetworkSettings.Networks}}{{.IPAddress}} {{end}}";

  const result = await exec(["docker", "inspect", "--format", format, containerName]);
  if (result.exitCode !== 0) {
    return undefined;
  }

  const values = result.stdout
    .trim()
    .split(/\s+/)
    .map((value) => value.trim())
    .filter(Boolean);

  return values[0];
}

export async function getContainerLogs(
  containerName: string,
  options: { tail?: number; since?: string } = {},
): Promise<string> {
  const args = ["docker", "logs", "--tail", String(options.tail ?? DEFAULT_LOG_TAIL)];
  if (options.since) {
    args.push("--since", options.since);
  }
  args.push(containerName);

  const result = await exec(args);
  if (result.exitCode !== 0) {
    return "";
  }

  return [result.stdout, result.stderr].filter(Boolean).join("\n").trim();
}

export async function listProjectContainers(
  project: string,
  options: { all?: boolean } = {},
): Promise<ContainerInfo[]> {
  const args = ["docker", "compose", "-p", project, "ps", "--format", "json"];
  if (options.all !== false) {
    args.push("-a");
  }

  const result = await exec(args);
  if (result.exitCode !== 0) {
    return [];
  }

  return parseComposePsOutput(result.stdout);
}

export async function containerExists(containerName: string): Promise<boolean> {
  const result = await exec([
    "docker",
    "inspect",
    "--format",
    "{{.Id}}",
    containerName,
  ]);
  return result.exitCode === 0 && result.stdout.length > 0;
}

export const __internal = {
  isValidNetworkName,
};

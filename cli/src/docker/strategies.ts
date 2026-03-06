import { exec } from "../utils/shell";

import { getContainerExitCode, getContainerLogs, getContainerState } from "./containers";
import type { ContainerState } from "./types";

const REQUEST_TIMEOUT_MS = 5_000;
const LOG_TAIL_LINES = 100;

async function safeFetch(url: string, init: RequestInit): Promise<Response | undefined> {
  try {
    return await fetch(url, {
      ...init,
      signal: AbortSignal.timeout(REQUEST_TIMEOUT_MS),
    });
  } catch {
    return undefined;
  }
}

export async function checkComposeHealthcheck(containerName: string): Promise<boolean> {
  const result = await exec([
    "docker",
    "inspect",
    "--format",
    "{{.State.Health.Status}}",
    containerName,
  ]);

  if (result.exitCode !== 0) {
    return false;
  }

  return result.stdout.trim().toLowerCase() === "healthy";
}

export async function checkRpc(url: string, method: string): Promise<boolean> {
  const response = await safeFetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      jsonrpc: "2.0",
      method,
      params: [],
      id: 1,
    }),
  });

  if (!response?.ok) {
    return false;
  }

  try {
    const payload = (await response.json()) as { error?: unknown; result?: unknown };
    return payload.error === undefined && payload.result !== undefined;
  } catch {
    return false;
  }
}

export async function checkHttp(url: string): Promise<boolean> {
  const response = await safeFetch(url, { method: "GET" });
  return Boolean(response?.ok);
}

export async function checkLogSentinel(containerName: string, pattern: string): Promise<boolean> {
  if (!pattern.trim()) {
    return false;
  }

  const logs = await getContainerLogs(containerName, { tail: LOG_TAIL_LINES });
  return logs.includes(pattern);
}

export async function checkDockerState(containerName: string): Promise<boolean> {
  const state = await getContainerState(containerName);
  return state === "running";
}

export async function checkExitCode(containerName: string): Promise<boolean> {
  const state = await getContainerState(containerName);
  if (state !== "exited" && state !== "dead") {
    return false;
  }

  const exitCode = await getContainerExitCode(containerName);
  return exitCode === 0;
}

export async function detectCrash(
  containerName: string,
): Promise<{ state: ContainerState; exitCode?: number } | null> {
  const state = await getContainerState(containerName);
  if (state !== "exited" && state !== "dead" && state !== "restarting") {
    return null;
  }

  const exitCode = await getContainerExitCode(containerName);
  return { state, exitCode };
}

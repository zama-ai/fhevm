import type { ServiceDefinition } from "../config/service-map";

export interface ComposeOptions {
  project: string;
  files: string[];
  envFile?: string;
  envVars?: Record<string, string>;
  cwd?: string;
}

export interface ComposeUpOptions extends ComposeOptions {
  build?: boolean;
  noBuild?: boolean;
  noCache?: boolean;
  wait?: boolean;
  waitTimeout?: number;
  services?: string[];
}

export interface ComposeDownOptions extends ComposeOptions {
  volumes?: boolean;
  removeImages?: "all" | "local";
  timeout?: number;
}

export interface ComposeLogsOptions extends ComposeOptions {
  services?: string[];
  follow?: boolean;
  tail?: number;
  format?: "json";
  noColor?: boolean;
}

export type ContainerState =
  | "running"
  | "exited"
  | "restarting"
  | "paused"
  | "dead"
  | "created"
  | "removing"
  | "not-found";

export interface ContainerInfo {
  name: string;
  service: string;
  state: ContainerState;
  health?: "healthy" | "unhealthy" | "starting" | "none";
  exitCode?: number;
  ports?: string;
  uptime?: string;
}

export interface ReadinessOptions {
  timeoutMs: number;
  pollIntervalMs: number;
  onCrash?: (service: string, exitCode: number) => void;
  onPoll?: (service: string, elapsedMs: number) => void;
}

export interface ReadinessResult {
  service: string;
  ready: boolean;
  elapsedMs: number;
  error?: string;
}

export interface ServiceStartOptions {
  envFileByName: ReadonlyMap<string, string>;
  envVars?: Record<string, string>;
  build?: boolean;
  noCache?: boolean;
  waitTimeoutMs?: number;
  cwd?: string;
}

export interface ServiceWaitOptions {
  timeoutMs?: number;
  pollIntervalMs?: number;
}

export interface ServiceStartupPlan {
  service: ServiceDefinition;
  envFile: string;
}

export const DEFAULT_TIMEOUTS = {
  serviceMs: 150_000,
  relayerMs: 120_000,
  keyBootstrapMs: 300_000,
  oneShotMs: 300_000,
} as const;

export const DEFAULT_POLL_INTERVAL_MS = 1_000;
export const DOCKER_PROJECT = "fhevm";
export const WAIT_TIMEOUT_ENV_NAME = "FHEVM_WAIT_TIMEOUT";

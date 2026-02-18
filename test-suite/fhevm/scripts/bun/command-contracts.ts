import type { RunResult } from "./process";

export type CommandFn = (args: string[], options?: { capture?: boolean; check?: boolean; allowFailure?: boolean }) => RunResult;

export type CommandDeps = {
  PROJECT: string;
  COMPOSE_DIR: string;
  COLORS: Record<string, string>;
  runCommand: CommandFn;
  sleep: (seconds: number) => void;
  usageError: (message: string) => never;
  cliError: (code: string, message: string) => never;
  logInfo: (message: string) => void;
  logWarn: (message: string) => void;
  color: (text: string, tone: string) => string;
  resolveProjectContainerName: (logicalName: string) => string | undefined;
  isContainerRunningExact: (containerName: string) => boolean;
  readContainerLogs: (containerName: string) => string;
  detectExpectedPause: (logs: string) => boolean;
  detectEnforcedPause: (logs: string) => boolean;
  cleanupKnownStack: (removeVolumes: boolean) => void;
  purgeProjectImages: () => void;
  purgeLocalBuildxCache: () => void;
  loadActiveVersionsIfPresent: () => void;
  localEnvFile: (component: string) => string;
  composeFile: (component: string) => string;
  ensureHostPortAssignments: () => Promise<void>;
  runComposeUp: (command: string[]) => { status: number; output: string };
};

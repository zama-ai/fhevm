/**
 * Engine-agnostic Stack interface consumed by rollout runbooks.
 *
 * This module has no imports from compose, readiness, state, or test
 * machinery — only from types.ts. Any engine (docker-compose, kind, …) can
 * implement Stack without pulling in the local-only runtime layer.
 */
import type { LocalOverride, State, VersionBundle, VersionTarget } from "./types";

type StackUpOptions = {
  lockFile: string;
  overrides?: LocalOverride[];
  scenario?: string;
};

type StackRuntimeUpgradeOptions = {
  lockFile?: string;
};

type StackVersionLockOptions = {
  allowedVersionKeys: string[];
  lockFile: string;
  overrides?: LocalOverride[];
};

type StackTestOptions = {
  network?: string;
  noHardhatCompile?: boolean;
  parallel?: boolean;
};

type StackContractTaskOptions = {
  env?: Record<string, string>;
};

type StackLockOptions = {
  versions: Record<string, string>;
  sources?: string[];
  target?: VersionTarget;
};

/**
 * Stack is the engine-agnostic interface passed to rollout runbooks.
 * Implementations live in commands/rollout-run.ts; runbooks only depend on
 * this interface so they remain portable across runtime engines.
 */
export type Stack = {
  applyVersionLock(label: string, options: StackVersionLockOptions): Promise<void>;
  readState(): Promise<State>;
  refreshDiscovery(): Promise<void>;
  runGatewayContractTask(command: string, options?: StackContractTaskOptions): Promise<void>;
  runHostContractTask(command: string, options?: StackContractTaskOptions): Promise<void>;
  snapshotContracts(surface: "host" | "gateway"): Promise<void>;
  stateDir(): string;
  test(profile?: string, options?: StackTestOptions): Promise<void>;
  up(options: StackUpOptions): Promise<void>;
  upgradeRuntimeGroup(group: string, options?: StackRuntimeUpgradeOptions): Promise<void>;
  writeVersionLock(name: string, options: StackLockOptions): Promise<string>;
};

/** A rollout runbook receives a Stack and drives the upgrade sequence. */
export type Runbook = (stack: Stack) => Promise<void> | void;

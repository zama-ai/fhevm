// EXEMPLAR — interface skeleton, not a working implementation

/**
 * Engine-agnostic Stack API.
 *
 * Canonical promotion of RolloutRunContext (test-suite/fhevm/src/commands/rollout-run.ts)
 * to a typed interface over kind + Helm.  Every method maps 1-to-1 to a Helm or kubectl
 * primitive; see inline doc comments.
 *
 * Data contracts assumed by every implementation:
 *   MANIFEST   — charts/<chart>/values/<env>.yaml carrying image.tag for each component
 *   ENDPOINT   — env vars (kmsCoreEndpoints, *_ENDPOINT) that redirect a service to an
 *                external/local process instead of the in-cluster one
 *   RUNBOOKS   — stateful procedures expressed as (ctx: Stack) => Promise<void>
 */

// ---------------------------------------------------------------------------
// Shared option bags (mirrors existing rollout-run.ts types)
// ---------------------------------------------------------------------------

export type UpOptions = {
  /** Path to a values file (MANIFEST) that pins image.tag for every chart. */
  lockFile: string;
  /** Per-group local overrides applied as additional --set-file flags. */
  overrides?: GroupOverride[];
  /** Optional scenario values file (charts/<chart>/values/<scenario>.yaml). */
  scenario?: string;
};

export type UpgradeOptions = {
  /** If provided, reads image.tag from this lock file instead of the running state. */
  lockFile?: string;
};

export type ContractTaskOptions = {
  /** Extra env vars injected into the Job pod (e.g. CONTRACT_ADDRESS=0x…). */
  env?: Record<string, string>;
};

export type TestOptions = {
  /** Hardhat network name passed to the test runner (default: "staging"). */
  network?: string;
  /** Skip solc recompilation when true (default: true in rollout context). */
  noHardhatCompile?: boolean;
  /** Run test files in parallel when true. */
  parallel?: boolean;
};

export type PinOptions = {
  /** Helm values keys to write (e.g. { "image.tag": "1.2.3" }). */
  values: Record<string, string>;
  /** Destination values file relative to the chart directory. */
  outputFile: string;
};

export type LogsOptions = {
  /** Tail this many lines from the end; omit to stream from beginning. */
  tail?: number;
  /** Stream continuously (kubectl logs -f). */
  follow?: boolean;
};

export type GroupOverride = {
  /** Helm release group name (e.g. "coprocessor", "kms-connector"). */
  group: string;
  /** Extra values file to merge for this group. */
  valuesFile?: string;
};

// ---------------------------------------------------------------------------
// Discovery / state types
// ---------------------------------------------------------------------------

/** Deployed contract addresses read from the sc-addresses ConfigMap. */
export type ContractAddresses = {
  ACL_CONTRACT_ADDRESS?: string;
  KMS_GENERATION_ADDRESS?: string;
  TFHE_EXECUTOR_ADDRESS?: string;
  GATEWAY_CONTRACT_ADDRESS?: string;
  [key: string]: string | undefined;
};

/** Persisted runtime state written by `up` and mutated by `upgrade`. */
export type StackState = {
  /** Helm release names that are currently installed. */
  releases: string[];
  /** Resolved contract addresses at the time of last discovery. */
  contractAddresses: ContractAddresses;
  /** Image tags in use per release group (mirrors the MANIFEST). */
  imageTags: Record<string, string>;
};

// ---------------------------------------------------------------------------
// Stack interface
// ---------------------------------------------------------------------------

export interface Stack {
  // -------------------------------------------------------------------------
  // Lifecycle
  // -------------------------------------------------------------------------

  /**
   * up — `helm upgrade --install --wait` each chart in dependency order
   * (anvil-node → contracts → kms-connector → coprocessor → listener).
   * Merges charts/<chart>/values/<env>.yaml + lockFile + overrides.
   */
  up(options: UpOptions): Promise<void>;

  /**
   * down — `helm uninstall` every release installed by this stack, then
   * deletes the sc-addresses ConfigMap and any leftover PVCs.
   */
  down(): Promise<void>;

  /**
   * upgrade — `helm upgrade` a single release group with a new image.tag
   * (sets image.tag via --set); Kubernetes rolls the Deployment in-place.
   * Waits for the rollout to complete (`kubectl rollout status`).
   */
  upgrade(group: string, options?: UpgradeOptions): Promise<void>;

  // -------------------------------------------------------------------------
  // Contract tasks (one-shot Jobs)
  // -------------------------------------------------------------------------

  /**
   * hostTask — runs the contracts chart sc-deploy Job image with `command`
   * against the host chain (anvil-node); writes results into the
   * sc-addresses ConfigMap on the host network namespace.
   */
  hostTask(command: string, options?: ContractTaskOptions): Promise<void>;

  /**
   * gatewayTask — same as hostTask but targets the gateway-contracts chart;
   * uses the gateway-sc-deploy Job definition and the gateway network.
   */
  gatewayTask(command: string, options?: ContractTaskOptions): Promise<void>;

  /**
   * snapshotContracts — triggers the contracts chart scUpgrade path, which
   * re-runs the sc-deploy Job with SNAPSHOT=true, recording compiled
   * artefacts for the given surface into the sc-addresses ConfigMap.
   */
  snapshotContracts(surface: "host" | "gateway"): Promise<void>;

  // -------------------------------------------------------------------------
  // Testing
  // -------------------------------------------------------------------------

  /**
   * test — runs a test Job (or `helm test`) against the running stack,
   * using the named profile to select test files and parameters.
   */
  test(profile?: string, options?: TestOptions): Promise<void>;

  // -------------------------------------------------------------------------
  // State & discovery
  // -------------------------------------------------------------------------

  /**
   * state — returns the persisted StackState written by `up`/`upgrade`;
   * throws if the stack is not running.
   */
  state(): Promise<StackState>;

  /**
   * discovery — reads the sc-addresses ConfigMap (`kubectl get configmap
   * sc-addresses -o json`) and returns the decoded ContractAddresses.
   */
  discovery(): Promise<ContractAddresses>;

  /**
   * refreshDiscovery — re-reads the sc-addresses ConfigMap and propagates
   * updated addresses into every running Deployment as env-var patches
   * (ACL_CONTRACT_ADDRESS, KMS_GENERATION_ADDRESS, …).
   */
  refreshDiscovery(): Promise<void>;

  /**
   * pin — writes `options.values` into `options.outputFile` as a Helm
   * values file (YAML), creating the MANIFEST artefact consumed by `up`.
   */
  pin(options: PinOptions): Promise<string>;

  // -------------------------------------------------------------------------
  // Chaos / read-state primitives
  // -------------------------------------------------------------------------

  /**
   * exec — `kubectl exec <pod> -- <command>` and returns combined stdout.
   */
  exec(pod: string, command: string[]): Promise<string>;

  /**
   * sql — `kubectl exec <pod> -- psql -c <query>` (or equivalent DB CLI);
   * convenience wrapper around exec for the postgres sidecar.
   */
  sql(pod: string, query: string): Promise<string>;

  /**
   * stop — `kubectl scale deployment/<name> --replicas=0`; halts the pod
   * without removing the Helm release.
   */
  stop(deploymentName: string): Promise<void>;

  /**
   * start — `kubectl scale deployment/<name> --replicas=<n>` (default 1);
   * restores a deployment stopped with `stop`.
   */
  start(deploymentName: string, replicas?: number): Promise<void>;

  /**
   * restart — `kubectl rollout restart deployment/<name>` then waits for
   * rollout; triggers a pod replacement without changing the replica count.
   */
  restart(deploymentName: string): Promise<void>;

  /**
   * logs — `kubectl logs <pod>` (optionally --tail / -f); returns the log
   * text as a string when not following, or streams it.
   */
  logs(pod: string, options?: LogsOptions): Promise<string>;

  /**
   * waitForLog — polls `kubectl logs <pod>` until `pattern` matches a line
   * or `timeoutMs` elapses; rejects with a timeout error on expiry.
   */
  waitForLog(pod: string, pattern: RegExp, timeoutMs?: number): Promise<void>;

  /**
   * chain — sends a JSON-RPC request directly to the anvil-node endpoint
   * and returns the parsed result; used for on-chain assertions.
   */
  chain<T = unknown>(method: string, params?: unknown[]): Promise<T>;

  /**
   * until — generic poll: calls `predicate` every `intervalMs` (default
   * 1 000) until it returns true or `timeoutMs` elapses.
   */
  until(predicate: () => Promise<boolean>, timeoutMs?: number, intervalMs?: number): Promise<void>;
}
